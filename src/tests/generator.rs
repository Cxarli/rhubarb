use alloc::{borrow::ToOwned, vec};

use crate::{
    asts::bobo::{Instruction::*, Load::*, Reg},
    generate,
};

#[test]
fn zero() {
    assert_eq!(
        generate(vec! {
            Load(Mvi(Reg::A, 0)),
        }),
        Ok("MVI A,0\n".to_owned())
    );
}
