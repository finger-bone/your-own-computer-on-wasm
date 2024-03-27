use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
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
#[derive(PartialEq, Debug)]
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

    Cmp = 0b01_0000_0001_0101,
    Cmn = 0b01_0000_0001_0110,
    Tst = 0b01_0000_0001_0111,
    Teq = 0b01_0000_0001_1000,

    Lsl = 0b01_0000_0010_0000,
    Lsr = 0b01_0000_0010_0001,
    Asr = 0b01_0000_0010_0010,
    Rol = 0b01_0000_0010_0011,
    Ror = 0b01_0000_0010_0100,

    Mvi = 0b01_0000_0011_0000,
    Qry = 0b01_0000_0011_0001,
    Int = 0b01_0000_0011_0010,

    Ldr = 0b10_0000_0000_0000,
    Str = 0b10_0000_0000_0001,
    Pop = 0b10_0000_0000_0010,
    Push = 0b10_0000_0000_0011,

    B = 0b11_0000_0000_0000,
    Bl = 0b11_0000_0000_0001,
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
            0b01_0000_0010_0000 => Operation::Lsl,
            0b01_0000_0010_0001 => Operation::Lsr,
            0b01_0000_0010_0010 => Operation::Asr,
            0b01_0000_0010_0011 => Operation::Rol,
            0b01_0000_0010_0100 => Operation::Ror,
            // cmp cmn tst teq
            0b01_0000_0001_0101 => Operation::Cmp,
            0b01_0000_0001_0110 => Operation::Cmn,
            0b01_0000_0001_0111 => Operation::Tst,
            0b01_0000_0001_1000 => Operation::Teq,

            0b01_0000_0011_0000 => Operation::Mvi,
            0b01_0000_0011_0001 => Operation::Qry,
            0b01_0000_0011_0010 => Operation::Int,

            0b10_0000_0000_0000 => Operation::Ldr,
            0b10_0000_0000_0001 => Operation::Str,
            0b10_0000_0000_0010 => Operation::Pop,
            0b10_0000_0000_0011 => Operation::Push,

            0b11_0000_0000_0000 => Operation::B,
            0b11_0000_0000_0001 => Operation::Bl,
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

#[wasm_bindgen]
pub fn cond_code_to_string(cond: u8) -> String {
    match cond {
        0b0000 => String::from("eq"),
        0b0001 => String::from("ne"),
        0b0010 => String::from("hs"),
        0b0011 => String::from("lo"),
        0b0100 => String::from("mi"),
        0b0101 => String::from("pl"),
        0b0110 => String::from("vs"),
        0b0111 => String::from("vc"),
        0b1000 => String::from("hi"),
        0b1001 => String::from("ls"),
        0b1010 => String::from("ge"),
        0b1011 => String::from("lt"),
        0b1100 => String::from("gt"),
        0b1101 => String::from("le"),
        0b1110 => String::from(""),
        _ => panic!("invalid cond code"),
    }
}

fn generate_postfix(decoded: Instr) -> String {
    if decoded.set_flags {
        String::from("s")
    } else {
        cond_code_to_string(decoded.cond_code as u8)
    }
}

fn reg_c_to_string(decoded: Instr) -> String {
    if decoded.c_is_imm {
        format!("#{}", decoded.reg_c)
    } else {
        format!("r{}", decoded.reg_c)
    }
}

fn generate_memo_addr(decoded: Instr) -> String {
    // ra, rb, rc
    if decoded.c_is_imm && decoded.reg_c == 0 {
        format!("r{}", decoded.reg_a)
    } else {
        format!("r{}, r{}, {}", decoded.reg_a, decoded.reg_b, reg_c_to_string(decoded))
    }
}

#[wasm_bindgen]
pub fn instr_to_string(instr: u64) -> String {
    let decoded = Instr::new(instr);
    let op = decode_op(decoded.op_code);
    // convert to string
    match op {
        Operation::Nop => String::from("nop"),
        Operation::Hlt => String::from("hlt"),
        Operation::Mov => format!("mov{} r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, reg_c_to_string(decoded)),
        Operation::Add => format!("add{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Sub => format!("sub{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Mul => format!("mul{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Div => format!("div{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::SMul => format!("smul{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::SDiv => format!("sdiv{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Modu => format!("modu{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::SModu => format!("smodu{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Mvn => format!("mvn{} r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, reg_c_to_string(decoded)),
        Operation::And => format!("and{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Orr => format!("orr{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Eor => format!("eor{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Lsl => format!("lsl{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Lsr => format!("lsr{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Asr => format!("asr{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Rol => format!("rol{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Ror => format!("ror{} r{}, r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Cmp => format!("cmp r{}, {}", decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Cmn => format!("cmn r{}, {}", decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Tst => format!("tst r{}, {}", decoded.reg_b, reg_c_to_string(decoded)),
        Operation::Teq => format!("teq r{}, {}", decoded.reg_b, reg_c_to_string(decoded)),
        
        Operation::Mvi => format!("mvi{} r{}", generate_postfix(decoded), decoded.reg_d_mem),
        Operation::Qry => format!("qry{} {}", generate_postfix(decoded), reg_c_to_string(decoded)),
        Operation::Int => format!("int{} r{}, {}", generate_postfix(decoded), decoded.reg_b, reg_c_to_string(decoded)),

        Operation::Ldr => format!("ldr{} r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, generate_memo_addr(decoded)),
        Operation::Str => format!("str{} r{}, {}", generate_postfix(decoded), decoded.reg_d_mem, generate_memo_addr(decoded)),
        Operation::Pop => format!("pop{} r{}", generate_postfix(decoded), decoded.reg_d_mem),
        Operation::Push => format!("push{} r{}", generate_postfix(decoded), decoded.reg_d_mem),

        Operation::B => format!("b{} {}", generate_postfix(decoded), reg_c_to_string(decoded)),
        Operation::Bl => format!("bl{} {}", generate_postfix(decoded), reg_c_to_string(decoded)),
    }
}
