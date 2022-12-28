use chumsky::Parser;

use screw_css::errors::Error;
use screw_css::parser::parser;
use screw_css::tokenizer::lex;

fn main() -> Result<(), Error> {
    let text = include_str!("../../text.screw");

    let result = lex(text.to_string())?;

    println!("{result:?}");

    let parsed = parser().parse(result).unwrap();

    println!("{parsed:?}");

    Ok(())
}