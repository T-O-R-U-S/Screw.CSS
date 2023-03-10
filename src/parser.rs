use std::fmt::{Display, Formatter};

use chumsky::error::Cheap;
use chumsky::Parser;
use chumsky::prelude::{end, filter, just, recursive};
use derive_more::{Display, IsVariant};
use itertools::Itertools;
use lightningcss::values::length::{LengthPercentage, LengthValue};
use lightningcss::values::percentage::Percentage;

use crate::tokenizer::{Keyword, Token};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Display)]
/// CSS units
pub enum Unit {
    #[display(fmt = "px")]
    Px,
    #[display(fmt = "pc")]
    Pc,
    #[display(fmt = "pt")]
    Pt,
    #[display(fmt = "in")]
    In,
    #[display(fmt = "cm")]
    Cm,
    #[display(fmt = "mm")]
    Mm,
    #[display(fmt = "vh")]
    Vh,
    #[display(fmt = "vw")]
    Vw,
    #[display(fmt = "vmin")]
    VMin,
    #[display(fmt = "vmax")]
    VMax,
    #[display(fmt = "rem")]
    Rem,
    #[display(fmt = "em")]
    Em,
    #[display(fmt = "deg")]
    Deg,
    #[display(fmt = "%")]
    Percent,
    /// arbitrary/unspecified unit
    #[display(fmt = "")]
    Arbitrary,
}

pub struct ParameterDisplay(pub Vec<Parameter>);

impl Display for ParameterDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = self.0.iter().fold(String::new(), |string, token| {
            string + " " + &token.to_string()
        });

        write!(f, "{string} ")
    }
}

#[derive(Debug, Clone, Display, IsVariant)]
pub enum Parameter {
    #[display(fmt = "{_0}{_1}")]
    Unit(f32, Unit),
    Ident(String),
    #[display(fmt = "{_0:?}")]
    String(String),
    Keyword(Keyword),
    #[display(fmt = "{name}:{parameter}")]
    Function {
        name: String,
        parameter: Box<Parameter>,
    },
    #[display(fmt = "{name}({}):{parameter}", "ParameterDisplay(procedural_parameters.clone())")]
    ProceduralFunction {
        name: String,
        procedural_parameters: Vec<Parameter>,
        parameter: Box<Parameter>,
    },
    #[display(fmt = "({})", "ParameterDisplay(_0.clone())")]
    Group(Vec<Parameter>),
    #[display(fmt = "@{_0:?}")]
    InjectedCSS(String),
}

impl Parameter {
    pub fn unwrap_unit(self) -> (f32, Unit) {
        match self {
            Self::Unit(x, y) => (x, y),
            unexpected => panic!("Expected Unit, found {unexpected}")
        }
    }
    pub fn unwrap_ident(self) -> String {
        match self {
            Self::Ident(id) => id,
            unexpected => panic!("Expected Ident, found {unexpected}")
        }
    }
    pub fn unwrap_string(self) -> String {
        match self {
            Self::String(str) => str,
            unexpected => panic!("Expected String, found {unexpected}")
        }
    }
    pub fn unwrap_keyword(self) -> Keyword {
        match self {
            Self::Keyword(keyword) => keyword,
            unexpected => panic!("Expected Keyword, found {unexpected}")
        }
    }
    pub fn unwrap_function(self) -> (String, Box<Parameter>) {
        match self {
            Self::Function { name, parameter } => (name, parameter),
            unexpected => panic!("Expected Function, found {unexpected}")
        }
    }
    pub fn unwrap_procedural(self) -> (String, Vec<Parameter>, Box<Parameter>) {
        match self {
            Self::ProceduralFunction { name, procedural_parameters, parameter } => (name, procedural_parameters, parameter),
            unexpected => panic!("Expected Procedural Function, found {unexpected}")
        }
    }
    pub fn unwrap_group(self) -> Vec<Parameter> {
        match self {
            Self::Group(group) => group,
            unexpected => panic!("Expected Group, found {unexpected}")
        }
    }
    pub fn negate(self, negate: bool) -> Parameter {
        if !negate {
            return self;
        }

        match self {
            Parameter::Unit(num, unit) => Parameter::Unit(num * -1.0, unit),
            Parameter::Group(parameters)
            =>
                Parameter::Group(
                    parameters.into_iter().map(|x| x.negate(true)).collect_vec()
                ),
            any => any
        }
    }
}

pub fn parser() -> impl Parser<Token, Vec<Parameter>, Error=Cheap<Token>> {
    recursive(|value| {
        let negator = just(Token::Negator).or_not().map(|x| x.is_some());
        let num = negator.clone()
            .then(filter(|x: &Token| x.is_number()))
            .map(|(negative, x)| {
                let num = x.unwrap_number();

                if negative {
                    -num
                } else {
                    num
                }
            })
            .labelled("Number");
        let unit = filter(|x: &Token| x.is_unit()).map(|x| x.unwrap_unit()).labelled("Unit");
        let group = filter(|x: &Token| x.is_group()).map(|x| x.unwrap_group()).map(|x| parser().parse(x).unwrap()).labelled("Group");
        let ident = filter(|x: &Token| x.is_ident()).map(|x| x.unwrap_ident()).labelled("Ident");
        let measurement = num.then(unit.or_not()).map(|(num, unit)| Parameter::Unit(num, unit.unwrap_or(Unit::Arbitrary))).labelled("Measurement");
        let string = filter(|x: &Token| x.is_string()).map(|x| x.unwrap_string()).labelled("String");
        let keyword = filter(|x: &Token| x.is_keyword()).map(|x| x.unwrap_keyword()).labelled("String");

        let injected_css = just(Token::At).ignore_then(string);

        let function = negator.then(ident)
            .labelled("Function identifier")
            .then_ignore(just(Token::Colon).labelled("Colon"))
            .then(
                value.clone().labelled("Function input")
            )
            .map(|((negative, name), parameter): ((bool, String), Parameter)| Parameter::Function {
                name,
                parameter: Box::new(parameter.negate(negative)),
            }).labelled("Function");

        let procedural_function = ident.then(group).then_ignore(just(Token::Colon)).then(value).map(
            |((name, procedural_parameters), parameter)| {
                Parameter::ProceduralFunction {
                    name,
                    procedural_parameters,
                    parameter: Box::new(parameter),
                }
            }
        ).labelled("Procedural function");

        procedural_function.or(
            function
        )
            .or(
                measurement
            )
            .or(
                keyword.map(Parameter::Keyword)
            )
            .or(
                ident.map(Parameter::Ident)
            )
            .or(
                injected_css.map(Parameter::InjectedCSS)
            )
            .or(
                string.map(Parameter::String)
            )
            .or(
                group.map(Parameter::Group)
            )
    }).repeated().then_ignore(end())
}

impl TryInto<LengthPercentage> for Parameter {
    type Error = Parameter;

    fn try_into(self) -> Result<LengthPercentage, Self::Error> {
        match self {
            Parameter::Unit(num, unit) => {
                Ok(LengthPercentage::Dimension((match unit {
                    Unit::Px => LengthValue::Px,
                    Unit::Pc => LengthValue::Pc,
                    Unit::Pt => LengthValue::Pt,
                    Unit::In => LengthValue::In,
                    Unit::Cm => LengthValue::Cm,
                    Unit::Mm => LengthValue::Mm,
                    Unit::Vh => LengthValue::Vh,
                    Unit::Vw => LengthValue::Vw,
                    Unit::VMin => LengthValue::Vmin,
                    Unit::VMax => LengthValue::Vmax,
                    Unit::Rem => LengthValue::Rem,
                    Unit::Em => LengthValue::Em,
                    Unit::Deg => return Err(self),
                    Unit::Percent => return Ok(LengthPercentage::Percentage(Percentage(num))),
                    Unit::Arbitrary => return Ok(LengthPercentage::Dimension(LengthValue::Em(num / 4.0)))
                })(num)))
            }
            parameter => Err(parameter)
        }
    }
}