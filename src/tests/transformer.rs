use crate::tests::parser::{add, number};
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
fn simple_sum() {
    assert_eq!(
        transform(add(number(1), number(2))),
        Ok(vec! {
            Load(Mvi(Reg::A, 1)),
            Load(Mvi(Reg::B, 2)),
            Arithmetic(Add(Reg::B)),
        })
    );
}
