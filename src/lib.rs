#![no_std]
extern crate alloc;

#[cfg(test)]
mod tests;

mod error;
pub use error::Error;

mod token;
pub use token::Token;

mod lexer;
pub use lexer::lex;

pub mod asts;

mod parser;
pub use parser::parse;

mod transformer;
pub use transformer::transform;

mod generator;
pub use generator::generate;
