/**!
 * The lexer is the first part of a compiler, and is responsible
 * for converting the input (string, file, etc)  into a list of
 * tokens recognised by the compiler.
 */
use crate::{Error, Token};

/**
 * Convert a stream of characters into a list of tokens.
 */
pub fn lex(_input: &str) -> Result<Vec<Token>, Error> {
    unimplemented!();
}
