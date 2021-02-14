use crate::{
    asts::expr::{Ast, AstNode, BinOp, Literal},
    parse,
    token::Token::*,
};

/**
 * Helpers to create some common AST nodes
 */
pub(in crate::tests) fn number(num: i8) -> Ast {
    AstNode::Literal(Literal::Number(num))
}
pub(in crate::tests) fn add(a: Ast, b: Ast) -> Ast {
    AstNode::BinOp(Box::new(BinOp::Add(a, b)))
}
pub(in crate::tests) fn subtract(a: Ast, b: Ast) -> Ast {
    AstNode::BinOp(Box::new(BinOp::Subtract(a, b)))
}
pub(in crate::tests) fn multiply(a: Ast, b: Ast) -> Ast {
    AstNode::BinOp(Box::new(BinOp::Multiply(a, b)))
}
pub(in crate::tests) fn divide(a: Ast, b: Ast) -> Ast {
    AstNode::BinOp(Box::new(BinOp::Divide(a, b)))
}
pub(in crate::tests) fn modulo(a: Ast, b: Ast) -> Ast {
    AstNode::BinOp(Box::new(BinOp::Modulo(a, b)))
}

#[test]
fn simple_add() {
    assert_eq!(
        parse(vec! { Number(3), Plus, Number(5) }),
        Ok(add(number(3), number(5)))
    );
}

#[test]
fn simple_sub() {
    assert_eq!(
        parse(vec! { Number(8), Minus, Number(2) }),
        Ok(subtract(number(8), number(2)))
    );
}

#[test]
fn simple_mod() {
    assert_eq!(
        parse(vec! { Number(10), Percent, Number(4) }),
        Ok(modulo(number(10), number(4)))
    );
}

#[test]
fn complex() {
    // 6 + 9 / 3  becomes +(6, /(9, 3))
    assert_eq!(
        parse(vec! { Number(6), Plus, Number(9), Slash, Number(3) }),
        Ok(add(number(6), divide(number(9), number(3))))
    );

    // 6 * 3 + 9 becomes +(*(6, 3), 9)
    assert_eq!(
        parse(vec! { Number(6), Asterisk, Number(3), Plus, Number(9) }),
        Ok(add(multiply(number(6), number(3)), number(9)))
    );
}
