use wasm_bindgen::prelude::*;
mod helper;
mod nodes;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_test() {
    assert_eq!(1 + 1, add(1, 1));
}
