use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Wire {
    data: u64,
}

#[wasm_bindgen]
impl Wire {
    pub fn get(&self) -> u64 {
        self.data
    }
    pub fn set(self, v: u64) -> Wire {
        Wire {
            data: v
        }
    }
    #[wasm_bindgen(constructor)]
    pub fn new() -> Wire {
        Wire { data: 0 }
    }
}


#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct SingleWire {
    data: bool,
}

#[wasm_bindgen]
impl SingleWire {
    pub fn get(&self) -> bool {
        self.data
    }
    pub fn set(self, v: bool) -> SingleWire {
        SingleWire {
            data: v
        }
    }
    #[wasm_bindgen(constructor)]
    pub fn new() -> SingleWire {
        SingleWire { data: false }
    }
}