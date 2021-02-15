#![no_std]

/**
 * Rhubarb is a very simple compiler from a basic math equation
 * containing just numbers and +-/%* into a series of instructions
 * which fit the Intel 8080 manual.
 *
 * It is a no_std library
 */
extern crate alloc;

#[cfg(test)]
mod tests;

mod error;
pub use error::Error;

mod token;
pub use token::Token;

mod lexer;
pub use lexer::lex;

/**
 * The collection of supported ASTs
 */
pub mod asts;

mod parser;
pub use parser::parse;

mod transformer;
pub use transformer::transform;

mod generator;
pub use generator::generate;
