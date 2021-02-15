/*!
 * The transformer is the third step of a common compiler chain.
 * It takes the AST from the input language and converts it to
 * the AST supported by the output language.
 */
use crate::{
    asts::{
        bobo::{self, *},
        expr::{self, *},
    },
    Error,
};
use alloc::{borrow::ToOwned, vec, vec::Vec};

type Regs = [bool; 8];

/**
 * Convert the expr node into a list of instructions,
 * then add those to the list
 * This function recurses into both A and B, with B always being first.
 * If you can, try to swap A and B to make sure B is smaller
 */
fn traverse(
    expr: &expr::Ast,
    dest: Reg,
    regs: &Regs,
    instructions: &mut Vec<Instruction>,
) -> Result<(), Error> {
    // convert a number literal into a load
    if let AstNode::Literal(Literal::Number(x)) = expr {
        instructions.push(Instruction::Load(Load::Mvi(dest, *x)));
        return Ok(());
    }

    // get an available temporary register to put the intermediate
    // result in
    let temp_reg = {
        if regs[Reg::B as usize] {
            Reg::B
        } else if regs[Reg::C as usize] {
            Reg::C
        } else if regs[Reg::D as usize] {
            Reg::D
        } else if regs[Reg::E as usize] {
            Reg::E
        }
        // NOTE: technically we shouldn't be using these
        // special registers as a temporary one,
        // but because we don't want to write to memory
        // anyhow, we're gonna (ab)use them in this fashion
        else if regs[Reg::H as usize] {
            Reg::H
        } else if regs[Reg::L as usize] {
            Reg::L
        }
        // do keep in mind that we can't use M this way,
        // because H and L are unpredictable
        else {
            return Err(Error::Transformer("out of registers".to_owned()));
        }
    };

    // make a copy of the old regs and claim the temporary one
    let mut new_regs = *regs;
    new_regs[temp_reg as usize] = false;

    // do the calculation
    match expr {
        // already handled above
        AstNode::Literal(_) => unreachable!(),

        // convert binops
        AstNode::BinOp(bo) => match &**bo {
            BinOp::Add(a, b) => {
                traverse(&b, temp_reg, &new_regs, instructions)?;
                traverse(&a, Reg::A, &new_regs, instructions)?;
                instructions.push(Instruction::Arithmetic(Arithmetic::Add(temp_reg)));
            }

            BinOp::Subtract(a, b) => {
                traverse(&b, temp_reg, &new_regs, instructions)?;
                traverse(&a, Reg::A, &new_regs, instructions)?;
                instructions.push(Instruction::Arithmetic(Arithmetic::Sub(temp_reg)));
            }

            BinOp::Multiply(a, b) => {
                traverse(&b, temp_reg, &new_regs, instructions)?;
                traverse(&a, Reg::A, &new_regs, instructions)?;
                instructions.push(Instruction::Arithmetic(Arithmetic::Mul(temp_reg)));
            }

            BinOp::Divide(a, b) => {
                traverse(&b, temp_reg, &new_regs, instructions)?;
                traverse(&a, Reg::A, &new_regs, instructions)?;
                instructions.push(Instruction::Arithmetic(Arithmetic::Div(temp_reg)));
            }

            BinOp::Modulo(a, b) => {
                traverse(&b, temp_reg, &new_regs, instructions)?;
                traverse(&a, Reg::A, &new_regs, instructions)?;
                instructions.push(Instruction::Arithmetic(Arithmetic::Mod(temp_reg)));
            }
        },
    }

    // copy value to destination if it's not already in there
    if dest != Reg::A {
        instructions.push(Instruction::Load(Load::Mov(dest, Reg::A)));
    }

    Ok(())
}

/**
 * Convert from the expr AST to the bobo AST
 */
pub fn transform(expr: expr::Ast) -> Result<bobo::Ast, Error> {
    let mut instructions = vec![];

    let regs = [
        // B, C, D, E
        true, true, true, true, // H, L
        true, true, // M, A
        false, false,
    ];

    traverse(&expr, Reg::A, &regs, &mut instructions)?;

    Ok(instructions)
}
