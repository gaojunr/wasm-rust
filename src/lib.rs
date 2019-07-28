mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(i: i32) -> i32 {
    match i {
        0 => 0,
        1 => 1,
        _ => greet(i - 1) + greet(i - 2),
    }
}
