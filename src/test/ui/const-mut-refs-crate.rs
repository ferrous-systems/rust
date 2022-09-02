// run-pass
// aux-build:const_mut_refs_crate.rs

extern crate const_mut_refs_crate as other;

use other::{
    inner::{INNER_MOD_BAR, INNER_MOD_FOO},
    BAR, FOO,
};

pub static LOCAL_FOO: &'static i32 = &41;
pub static LOCAL_BAR: &'static i32 = LOCAL_FOO;

fn main() {
    assert_eq!(FOO as *const i32, BAR as *const i32);
    assert_eq!(INNER_MOD_FOO as *const i32, INNER_MOD_BAR as *const i32);
    assert_eq!(LOCAL_FOO as *const i32, LOCAL_BAR as *const i32);
}
