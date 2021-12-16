#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use problem::my_macro;
# [serde (rename_all = snake_case)]
enum MyStruct {
    Variant,
}
fn main() {}
