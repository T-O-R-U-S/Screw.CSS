use chumsky::Parser;
use itertools::Itertools;

use screw_css::errors::Error;
use screw_css::parser::{Parameter, ParameterDisplay, parser};
use screw_css::tokenizer::lex;

fn main() -> Result<(), Error> {
    let text = include_str!("../../text.screw");

    let result = lex(text.to_string())?;

    let enumerated_result = result.iter().enumerate().collect_vec();

    println!("{enumerated_result:?}");

    let parsed = parser().parse(result).unwrap();

    println!("{:#?}", parsed);

    Ok(())
}