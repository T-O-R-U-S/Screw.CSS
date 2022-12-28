#![feature(let_chains)]
#![feature(exact_size_is_empty)]
#![feature(type_alias_impl_trait)]

pub mod tokenizer;
pub mod errors;
pub mod parser;
pub mod compiler;

// TODO: Tests :)
#[cfg(test)]
mod tests {
    use crate::tokenizer::lex;
    use super::*;

    #[test]
    fn it_works() {

    }
}
