use std::any::Any;
use std::process::id;
use chumsky::prelude::*;
use chumsky::text::ident;
use crate::rule::{Keyword, Parameter, Size};
use crate::rule::Size::{Arbitrary, Cm, Em, In, Mm, Pc, Percent, Pt, Px, Rem, Vh, VMax, VMin, Vw};

#[derive(Debug, Clone)]
pub struct FunCall {
    function_name: String,
    function_params: Vec<Parameter>
}

pub fn tokenize(input: String) -> impl Parser<char, FunCall, Error=Simple<char>> {
    recursive(|value| {
        let unit = text::int::<char, Simple<char>>(10)
            .then(ident().or(just('%').map(|x| x.to_string())).or_not())
            .padded()
            .map(|(num, unit): (String, Option<String>)| {
                let Ok(num) = num.parse::<f64>() else {
                    unreachable!("uh oh")
                };

                let Some(unit) = unit else {
                    return Parameter::Size(Arbitrary(num))
                };

                let currennt_unit = match unit.as_str() {
                    "vh" => Vh,
                    "vw" => Vw,
                    "vmin" => VMin,
                    "vmax" => VMax,
                    "in" => In,
                    "cm" => Cm,
                    "mm" => Mm,
                    "pt" => Pt,
                    "pc" => Pc,
                    "em" => Em,
                    "rem" => Rem,
                    "%" => Percent,
                    absolute => {
                        return match absolute {
                            "px" => Parameter::Size(Px(num as u64)),
                            &_ => Parameter::Size(Arbitrary(num))
                        };
                    }
                };

                Parameter::Size(currennt_unit(num))
            });

        let keyword =
            text::keyword("grid").to(Parameter::Keyword(Keyword::Grid))
                .or(
                    text::keyword("inline").to(Parameter::Keyword(Keyword::Inline))
                );

        let parameter = keyword.clone()
            .or(unit.clone())
            .to(Parameter::Size(Size::Arbitrary(0.0)));

        let fun_group = value.repeated().delimited_by(just('('), just(')'));

        let funcall = ident().then(just(':')).ignore_then(parameter.clone());

        parameter.or(keyword).or(unit).padded().to(FunCall {
            function_name: "".into(),
            function_params: vec![]
        })
    })
        .then_ignore(end())
}