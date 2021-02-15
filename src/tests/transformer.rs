use alloc::vec;

use crate::tests::parser::{number, subtract};
use crate::{
    asts::bobo::{Arithmetic::*, Instruction::*, Load::*, Reg},
    transform,
};

#[test]
fn zero() {
    assert_eq!(
        transform(number(0)),
        Ok(vec! {
            Load(Mvi(Reg::A, 0)),
        })
    );
}

#[test]
fn simple_subtraction() {
    assert_eq!(
        transform(subtract(number(6), number(4))),
        Ok(vec! {
            Load(Mvi(Reg::B, 4)),
            Load(Mvi(Reg::A, 6)),
            Arithmetic(Sub(Reg::B)),
        })
    );
}
