#[derive(Debug)]
pub struct Reg {
    data: u64,
}

impl Reg {
    pub fn get(&self) -> u64 {
        self.data
    }
    pub fn set(self, v: u64) -> Reg {
        Reg {
            data: v
        }
    }
    pub fn new() -> Reg {
        Reg { data: 0 }
    }
}