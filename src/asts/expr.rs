/**!
 * This is the AST for the input language. It only consists of operators and literals.
 */

/**
 * Alias
 */
pub type Ast = AstNode;

/**
 * A single node in the graph. This can be either an operation or a literal
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstNode {
    BinOp(Box<BinOp>),
    Literal(Literal),
}

/**
 * A Binary Operation between two other nodes
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinOp {
    Add(Ast, Ast),
    Subtract(Ast, Ast),
    Multiply(Ast, Ast),
    Divide(Ast, Ast),
    Modulo(Ast, Ast),
}

/**
 * A literal value. This can only be a number.
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Number(i8),
}