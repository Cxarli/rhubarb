/**!
 * The generator is the last step in the compiler chain.
 * It takes the AST either directly from the parser or after
 * some optimisation steps, and turns it into the wanted output
 * code.
 */
use crate::{asts::bobo::Ast, Error};

/**
 * Turn the AST into output 8080 assembly code.
 */
pub fn generate(_ast: Ast) -> Result<String, Error> {
    unimplemented!();
}
