use wasm_bindgen::prelude::*;

use super::instr::Instr;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Decoder {

}

#[wasm_bindgen]
impl Decoder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Decoder {
        Decoder {
        }
    }
    pub fn decode(self, v: u64) -> Instr {
        Instr::new(v)
    }
}