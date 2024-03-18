use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Instr {
    pub cond_code: u64,
    pub set_flags: bool,
    pub c_is_imm: bool,
    pub op_code: u64,
    pub reg_d_mem: u64,
    pub reg_a: u64,
    pub reg_b: u64,
    pub reg_c: u64,
}

#[wasm_bindgen]
impl Instr {
    #[wasm_bindgen(constructor)]
    pub fn new(v: u64) -> Instr {
        // the first 4 bits of v
        let cond_code = (v & (0xff << (64 - 4))) >> (64 - 4);
        // the next bit
        let set_flags = v & (0x1 << (64 - 5)) != 0;
        // the next bit
        let c_is_imm = v & (0x1 << (64 - 6)) != 0;
        // the next 14 bits
        let op_code = (v & (0x3fff << (64 - 20))) >> (64 - 20);
        // the next 4 bits
        let reg_d_mem = (v & (0xf << (64 - 24))) >> (64 - 24);
        // the next 4 bits
        let reg_a = (v & (0xf << (64 - 28))) >> (64 - 28);
        // the next 4 bits
        let reg_b = (v & (0xf << (64 - 32))) >> (64 - 32);
        // the next four byte
        let reg_c = (v & (0xffff << (64 - 64))) >> (64 - 64);
        Instr {
            cond_code,
            set_flags,
            c_is_imm,
            op_code,
            reg_d_mem,
            reg_a,
            reg_b,
            reg_c,
        }
    }
}

// 2 bits for instr type, 12 bits for instr

#[wasm_bindgen]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Operation {
    Nop = 0b00_0000_0000_0000,
    Hlt = 0b00_0000_0000_0001,
    Mov = 0b01_0000_0000_0000,
    Add = 0b01_0000_0000_0001,
    Sub = 0b01_0000_0000_0010,
    Mul = 0b01_0000_0000_0011,
    Div = 0b01_0000_0000_0100,
    SMul = 0b01_0000_0000_0101,
    SDiv = 0b01_0000_0000_0110,
    Modu = 0b01_0000_0000_0111,
    SModu = 0b01_0000_0000_1000,
    Mvn = 0b01_0000_0000_1001,
    And = 0b01_0000_0000_1010,
    Orr = 0b01_0000_0000_1011,
    Eor = 0b01_0000_0000_1100,
    Lsl = 0b01_0000_1000_0000,
    Lsr = 0b01_0000_1000_0001,
    Asr = 0b01_0000_1000_0010,
    Rol = 0b01_0000_1000_0011,
    Ror = 0b01_0000_1000_0100,

    Ldr = 0b10_0000_0000_0000,
    Str = 0b10_0000_0000_0001,
    Pop = 0b10_0000_0000_0010,
    Push = 0b10_0000_0000_0011,

    B = 0b11_0000_0000_1000,
    Bl = 0b11_0000_0000_1001,
}

#[wasm_bindgen]
impl Operation {
    #[wasm_bindgen(constructor)]
    pub fn new(op_code: u64) -> Operation {
        match op_code {
            0b00_0000_0000_0000 => Operation::Nop,
            0b00_0000_0000_0001 => Operation::Hlt,
            0b01_0000_0000_0000 => Operation::Mov,
            0b01_0000_0000_0001 => Operation::Add,
            0b01_0000_0000_0010 => Operation::Sub,
            0b01_0000_0000_0011 => Operation::Mul,
            0b01_0000_0000_0100 => Operation::Div,
            0b01_0000_0000_0101 => Operation::SMul,
            0b01_0000_0000_0110 => Operation::SDiv,
            0b01_0000_0000_0111 => Operation::Modu,
            0b01_0000_0000_1000 => Operation::SModu,
            0b01_0000_0000_1001 => Operation::Mvn,
            0b01_0000_0000_1010 => Operation::And,
            0b01_0000_0000_1011 => Operation::Orr,
            0b01_0000_0000_1100 => Operation::Eor,
            0b01_0000_1000_0000 => Operation::Lsl,
            0b01_0000_1000_0001 => Operation::Lsr,
            0b01_0000_1000_0010 => Operation::Asr,
            0b01_0000_1000_0011 => Operation::Rol,
            0b01_0000_1000_0100 => Operation::Ror,
            0b10_0000_0000_0000 => Operation::Ldr,
            0b10_0000_0000_0001 => Operation::Str,
            0b10_0000_0000_0010 => Operation::Pop,
            0b10_0000_0000_0011 => Operation::Push,

            0b11_0000_0000_1000 => Operation::B,
            0b11_0000_0000_1001 => Operation::Bl,
            _ => panic!("invalid instr"),
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum OperationType {
    Special = 0b00,
    DataProc = 0b01,
    Mem = 0b10,
    Branch = 0b11,
}

#[wasm_bindgen]
impl OperationType {
    #[wasm_bindgen(constructor)]
    pub fn new(op_code: u64) -> OperationType {
        let type_code = (op_code >> 12) & 0b11;
        match type_code {
            0b00 => OperationType::Special,
            0b01 => OperationType::DataProc,
            0b10 => OperationType::Mem,
            0b11 => OperationType::Branch,
            _ => panic!("invalid instr type"),
        }
    }
}

#[wasm_bindgen]
pub fn decode_op(op_code: u64) -> Operation {
    Operation::new(op_code)
}

#[wasm_bindgen]
pub fn decode_op_type(op_code: u64) -> OperationType {
    OperationType::new(op_code)
}
