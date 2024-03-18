use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Cpsr {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
}

impl Cpsr {
    pub fn from_u8(nzcv: u8) -> Cpsr {
        Cpsr {
            n: ((nzcv >> 3 & 1) == 1),
            z: ((nzcv >> 2 & 1) == 1),
            c: ((nzcv >> 1 & 1) == 1),
            v: ((nzcv >> 0 & 1) == 1),
        }
    }

    pub fn to_u8(&self) -> u8 {
        ((self.n as u8) & 1) << 3
        | ((self.z as u8) & 1) << 2
        | ((self.c as u8) & 1) << 1
        | ((self.v as u8) & 1) << 0
    }
}