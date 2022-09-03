// run-pass
// aux-build:const_mut_refs_crate.rs

extern crate const_mut_refs_crate as other;

use other::{
    inner::{INNER_MOD_BAR, INNER_MOD_FOO},
    BAR, FOO,
};

pub static LOCAL_FOO: &'static i32 = &41;
pub static LOCAL_BAR: &'static i32 = LOCAL_FOO;
pub static LOCAL_BAZ: &'static i32 = FOO;

static DOUBLE_REF: &&i32 = &&99;
static ONE_STEP_ABOVE: &i32 = *DOUBLE_REF;

pub fn main() {
    assert_eq!(FOO as *const i32, BAR as *const i32);
    assert_eq!(INNER_MOD_FOO as *const i32, INNER_MOD_BAR as *const i32);
    assert_eq!(LOCAL_FOO as *const i32, LOCAL_BAR as *const i32);
    assert_eq!(*DOUBLE_REF as *const i32, ONE_STEP_ABOVE as *const i32);

    // assert_eq!(FOO as *const i32, LOCAL_BAZ as *const i32);
    // above fails
    // ---- [ui] src/test/ui/const-mut-refs-crate.rs stdout ----
    //
    // error: test run failed!
    // status: exit status: 101
    // command: "..rust/build/x86_64-unknown-linux-gnu/test/ui/const-mut-refs-crate/a"
    // stdout: none
    // --- stderr -------------------------------
    // thread 'main' panicked at 'assertion failed: `(left == right)`
    //   left: `0x7f3f52fe5000`,
    //  right: `0x55f4daa4b000`', ..rust/src/test/ui/const-mut-refs-crate.rs:20:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // ------------------------------------------
}
