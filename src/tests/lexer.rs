use crate::{lex, token::Token::*};

#[test]
fn simple_numbers() {
    assert_eq!(lex("1"), Ok(vec! { Number(1) }));
    assert_eq!(lex("42"), Ok(vec! { Number(42) }));
    assert_eq!(lex("250"), Ok(vec! { Number(250) }));
}

#[test]
fn simple_add() {
    assert_eq!(lex("1 + 2"), Ok(vec! { Number(1), Plus, Number(2) }));
}

#[test]
fn simple_sub() {
    assert_eq!(lex("4 - 2"), Ok(vec! { Number(4), Minus, Number(2) }));
}

#[test]
fn simple_mul() {
    assert_eq!(lex("3 * 5"), Ok(vec! { Number(3), Asterisk, Number(5) }));
}

#[test]
fn simple_div() {
    assert_eq!(lex("9 / 3"), Ok(vec! { Number(9), Slash, Number(3) }));
}

#[test]
fn simple_mod() {
    assert_eq!(lex("10 % 4"), Ok(vec! { Number(10), Percent, Number(4) }));
}
