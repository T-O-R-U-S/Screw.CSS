use std::fmt::{Display, Formatter};
use std::iter::{Enumerate, Peekable};
use std::vec::IntoIter;
use chumsky::combinator::{Map, Then};
use chumsky::Parser;
use chumsky::prelude::{end, filter, filter_map, just, recursive, Recursive, skip_then_retry_until};
use crate::errors::{Error, Unexpected::{Token as UnexpectedToken, Parameter as UnexpectedParameter, TokenUnspecified, ParameterUnspecified}};
use crate::tokenizer::{Keyword, Token};
use derive_more::{Display, FromStr, IsVariant, Unwrap};
use chumsky::error::Cheap;
use chumsky::primitive::Filter;
use crate::errors::Error::Unexpected;

#[derive(Debug, Clone, PartialOrd, PartialEq, Display)]
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
    Arbitrary
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

#[derive(Debug, Clone, Display, Unwrap, IsVariant)]
pub enum Parameter {
    #[display(fmt = "{_0}{_1}")]
    Unit(f64, Unit),
    Ident(String),
    #[display(fmt = "{_0:?}")]
    String(String),
    Keyword(Keyword),
    #[display(fmt = "{_0}:({})", "ParameterDisplay(_1.clone())")]
    Function(String, Vec<Parameter>),
    #[display(fmt = "{_0}({}):({})", "ParameterDisplay(_1.clone())", "ParameterDisplay(_2.clone())")]
    ProceduralFunction(String, Vec<Parameter>, Vec<Parameter>)
}

pub fn parser() -> impl Parser<Token, Vec<Parameter>, Error=Cheap<Token>> {
    recursive(|value| {
        let num = filter(|x: &Token| x.is_number()).map(|x| x.unwrap_number()).labelled("Number");
        let unit = filter(|x: &Token| x.is_unit()).map(|x| x.unwrap_unit()).labelled("Unit");
        let group = filter(|x: &Token| x.is_group()).map(|x| x.unwrap_group()).map(|x| parser().parse(x).unwrap()).labelled("Group");
        let ident = filter(|x: &Token| x.is_ident()).map(|x| x.unwrap_ident()).labelled("Ident");
        let unit = num.then(unit.or_not()).map(|(num, unit)| Parameter::Unit(num, unit.unwrap_or(Unit::Arbitrary))).labelled("Unit");
        let string = filter(|x: &Token| x.is_string()).map(|x| x.unwrap_string()).labelled("String");

        let function = ident
            .labelled("Function identifier")
            .then_ignore(just(Token::Colon).or_not())
            .labelled("Colon")
            .then(
                group.or(
                    value
                )
            )
            .labelled("Function input")
            .map(|(name, parameters)| Parameter::Function(name, parameters));

        let procedural_function = function.clone().then_ignore(just(Token::Colon)).then(group).map(
            |(procedural_function, parameters)| {
                let (function_name, procedural_parameters) = procedural_function.unwrap_function();

                Parameter::ProceduralFunction(function_name, procedural_parameters, parameters)
            }
        );

        procedural_function.or(
            function.labelled("Function")
        )
            .or(
                unit.labelled("Unit input")
            )
            .or(
                ident.map(Parameter::Ident)
            )
            .or(
                string.map(Parameter::String)
            )
            .repeated()
    }).then_ignore(end())
}