pub struct Mem {
    mem: Vec<u8>,
}

impl Mem {
    pub fn new(val: Vec<u8>) -> Mem {
        Mem {
            mem: val,
        }
    }
    pub fn get_word(&self, addr: u64) -> u64 {
        let mut val = 0;
        for i in 0..8 {
            val |= (self.mem[addr as usize + i] as u64) << (8 * (8 - i - 1));
        }
        val
    }
    pub fn set_word(mut self, addr: u64, val: u64) -> Mem {
        for i in 0..8 {
            self.mem[addr as usize + i] = ((val >> (8 * (8 - i - 1))) & 0xff) as u8;
        }
        Mem {
            mem: self.mem,
        }
    }
}

impl Mem {
    pub fn dump(&self) -> Vec<u8> {
        self.mem.clone()
    }
    pub fn get_size(&self) -> usize {
        self.mem.len()
    }
}