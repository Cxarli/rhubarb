use crate::{
    asts::bobo::{self, Ast},
    Error,
};
/**!
 * The generator is the last step in the compiler chain.
 * It takes the AST either directly from the parser or after
 * some optimisation steps, and turns it into the wanted output
 * code.
 */
use alloc::{format, string::String};

/**
 * Turn the AST into output 8080 assembly code.
 */
pub fn generate(ast: Ast) -> Result<String, Error> {
    use bobo::{Arithmetic::*, Instruction::*, Load::*};

    // the output string
    let mut output = String::new();

    // loop over all instructions
    for instruction in ast {
        output += &match instruction {
            // comments
            Comment(text) => format!("; {}\n", text),

            // loads
            Load(Mvi(reg, val)) => format!("MVI {},{}\n", reg.name(), val),
            Load(Mov(dest, src)) => format!("MOV {},{}\n", dest.name(), src.name()),

            // arithmetic
            Arithmetic(Add(reg)) => format!("ADD {}\n", reg.name()),
            Arithmetic(Sub(reg)) => format!("SUB {}\n", reg.name()),
            Arithmetic(Mul(reg)) => format!("MUL {}\n", reg.name()),
            Arithmetic(Div(reg)) => format!("DIV {}\n", reg.name()),
            Arithmetic(Mod(reg)) => format!("MOD {}\n", reg.name()),
        };
    }

    Ok(output)
}
