pub mod emulator;
pub mod assembler;
mod test;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, emulator!");
}

#[wasm_bindgen]
pub fn greet_number(v: u64) {
    alert(&format!("test: {}", v));
}