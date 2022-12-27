use thiserror::Error;
use crate::parser::Parameter;
use crate::tokenizer::Token;

#[derive(Error, Debug)]
pub enum Unexpected {
    #[error("Unexpected character {0}")]
    Char(char),
    #[error("Unexpected token {0:?}")]
    Token(Token),
    #[error("Unexpected parameter {0:?}")]
    Parameter(Parameter),
    #[error("Unexpected token")]
    TokenUnspecified,
    #[error("Unexpected parameter")]
    ParameterUnspecified,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected end of file while parsing")]
    UnexpectedEOF,
    #[error("{0}")]
    Unexpected(Unexpected),
    #[error("Incorrectly formatted number")]
    IncorrectNumber
}