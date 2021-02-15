/*!
 * The general error capturer across the entire compiler chain.
 */
use alloc::string::String;

/**
 * A description of an error somewhere along the compiler chain
 */
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Lexer(String),
    Parser(String),
    Transformer(String),
    Generator(String),
}
