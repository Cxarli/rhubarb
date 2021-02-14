/**!
 * The transformer is the third step of a common compiler chain.
 * It takes the AST from the input language and converts it to
 * the AST supported by the output language.
 */
use crate::{
    asts::{bobo, expr},
    Error,
};

/**
 * Convert from the expr AST to the bobo AST
 */
pub fn transform(_expr: expr::Ast) -> Result<bobo::Ast, Error> {
    unimplemented!();
}
