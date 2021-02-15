/**!
 * This is the AST for the 8080 output language. It consists of a series of instructions.
 * Each instruction can either be a single operation or loading a literal into a register.
 */
use alloc::{vec::Vec, string::String};

/**
 * For simplicity, this is an alias
 */
pub type Ast = AstRoot;

/**
 * The root node of the graph only supports a list of instructions
 */
pub type AstRoot = Vec<Instruction>;

/**
 * An instruction is one task done on the controller.
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Instruction {
    Comment(String),
    Arithmetic(Arithmetic),
    Load(Load),
}

/**
 * Arithmetic instructions are executed on the Accumulator (register A)
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Arithmetic {
    /// Add the value of the register to A
    Add(Reg),
    /// Subtract the value of the register from A
    Sub(Reg),
    /// Multiply the value of the register with A
    Mul(Reg),
    Divide(Reg),
    Modulo(Reg),
}

/**
 * Load a value into a register
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Load {
    /// Move the value into the given register
    Mvi(Reg, u8),
}

/**
 * A register is a location where data is stored
 * This can either be used temporarily (B, C, D, E),
 * or to access memory (H, L, M).
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Reg {
    /// B, C, D, E are normal registers
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,

    /// H and L refer to the memory High and Low address
    H = 0b100,
    L = 0b101,

    /// M refers to the memory
    M = 0b110,

    /// A is the accumulator
    A = 0b111,
}
