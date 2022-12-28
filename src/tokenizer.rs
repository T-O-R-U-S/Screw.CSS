use derive_more::{Display, IsVariant, Unwrap};

use crate::errors::{Error, Unexpected};
use crate::parser::Unit;

// TODO: Some keywords!
#[derive(Debug, Clone, PartialOrd, PartialEq, Display, Unwrap, IsVariant)]
pub enum Keyword {}

#[derive(Debug, Clone, PartialOrd, PartialEq, Display, Unwrap, IsVariant)]
pub enum Token {
    #[display(fmt = "-")]
    Negator,
    #[display(fmt = ":")]
    Colon,
    #[display(fmt = "@")]
    At,
    #[display(fmt = "{_0:?}")]
    String(String),
    #[display(fmt = "{_0}")]
    Ident(String),
    #[display(fmt = "{_0}")]
    Number(f64),
    #[display(fmt = "TODO: Group formatting")]
    Group(Vec<Token>),
    #[display(fmt = "TODO: Block formatting")]
    Block(Vec<Token>),
    #[display(fmt = "{_0}")]
    Keyword(Keyword),
    Unit(Unit),
}

pub fn lex(input: String) -> Result<Vec<Token>, Error> {
    let mut input = input.chars().peekable();

    let mut output = vec![];

    while let Some(character) = input.next() {
        let token = match character {
            ignored if ignored.is_whitespace() => continue,
            '-' => Token::Negator,
            '@' => Token::At,
            ':' => Token::Colon,
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::from(character);

                while let Some(character) = input.next_if(|x| matches!(x, 'a'..='z' | 'A'..='Z' | '-')) {
                    ident.push(character)
                }

                match ident.as_str() {
                    "deg" => Token::Unit(Unit::Deg),
                    "em" => Token::Unit(Unit::Em),
                    "rem" => Token::Unit(Unit::Rem),
                    "vmax" => Token::Unit(Unit::VMax),
                    "vmin" => Token::Unit(Unit::VMin),
                    "vw" => Token::Unit(Unit::Vw),
                    "vh" => Token::Unit(Unit::Vh),
                    "cm" => Token::Unit(Unit::Cm),
                    "mm" => Token::Unit(Unit::Mm),
                    "in" => Token::Unit(Unit::In),
                    "pt" => Token::Unit(Unit::Pt),
                    "pc" => Token::Unit(Unit::Pc),
                    "px" => Token::Unit(Unit::Px),
                    _ => Token::Ident(ident)
                }
            }
            '0'..='9' => {
                let mut number = String::from(character);

                let mut radix = 10;

                if number.as_str() == "0" {
                    match input.next() {
                        Some('o') => radix = 8,
                        Some('x') => radix = 16,
                        Some('b') => radix = 2,
                        Some(digit @ '0'..='9') => { number.push(digit) }
                        Some(_) => return Err(Error::IncorrectNumber),
                        None => {}
                    }
                }

                while let Some(character) = input.next_if(|x| matches!(x, '0'..='9' | 'a'..='f' | '.')) {
                    number.push(character)
                }

                if radix != 10 {
                    let number = u64::from_str_radix(&number, radix).or(Err(Error::IncorrectNumber))? as f64;

                    Token::Number(number)
                } else {
                    let number = number.parse().or(Err(Error::IncorrectNumber))?;

                    Token::Number(number)
                }
            }
            '(' => {
                let mut open_brackets = 1;

                let mut to_parse = String::new();

                while open_brackets != 0 {
                    match input.next() {
                        Some('(') => {
                            open_brackets += 1;
                            to_parse.push('(');
                        }
                        Some(')') => {
                            open_brackets -= 1;
                            if open_brackets != 0 {
                                to_parse.push(')')
                            }
                        }

                        Some('\\') => to_parse.push(input.next().unwrap_or('\x00')),

                        Some(character) => to_parse.push(character),

                        None => return Err(Error::UnexpectedEOF)
                    }
                }

                Token::Group(lex(to_parse)?)
            }
            '{' => {
                let mut open_brackets = 1;

                let mut to_parse = String::new();

                while open_brackets != 0 {
                    match input.next() {
                        Some('{') => {
                            open_brackets += 1;
                            to_parse.push('{');
                        }
                        Some('}') => {
                            open_brackets -= 1;
                            if open_brackets != 0 {
                                to_parse.push('}')
                            }
                        }

                        Some('\\') => to_parse.push(input.next().unwrap_or('\x00')),

                        Some(character) => to_parse.push(character),

                        None => return Err(Error::UnexpectedEOF)
                    }
                }

                Token::Block(lex(to_parse)?)
            }
            '"' => {
                let mut output = String::new();

                while let Some(character) = input.next() {
                    match character {
                        '\\' => output.push(input.next().unwrap_or('\x00')),
                        '"' => break,
                        character => output.push(character)
                    }
                }

                Token::String(output)
            }
            '$' => todo!("Variable support is not yet done, or finalized."),
            '%' => Token::Unit(Unit::Percent),
            unexpected => return Err(Error::Unexpected(Unexpected::Char(unexpected)))
        };

        output.push(token)
    }

    Ok(output)
}