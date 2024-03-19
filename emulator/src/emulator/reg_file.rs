use super::{cpsr::Cpsr, reg::Reg};

#[derive(Debug)]
pub enum ConditionCode {
    EQ = 0b0000, // Equal
    NE = 0b0001, // Not Equal
    HS = 0b0010, // Unsigned higher or same (or carry set)
    LO = 0b0011, // Unsigned lower (or carry clear)
    MI = 0b0100, // Negative
    PL = 0b0101, // Positive or Zero
    VS = 0b0110, // Signed Overflow
    VC = 0b0111, // No Signed Overflow
    HI = 0b1000, // Unsigned Higher
    LS = 0b1001, // Unsigned Lower or Same
    GE = 0b1010, // Signed Greater Than or Equal
    LT = 0b1011, // Signed Less Than
    GT = 0b1100, // Signed Greater Than
    LE = 0b1101, // Signed Less Than or Equal
    AL = 0b1110, // Always Executed
}

impl ConditionCode {
    pub fn from_u8(nzcv: u8) -> ConditionCode {
        match nzcv {
            0b0000 => ConditionCode::EQ,
            0b0001 => ConditionCode::NE,
            0b0010 => ConditionCode::HS,
            0b0011 => ConditionCode::LO,
            0b0100 => ConditionCode::MI,
            0b0101 => ConditionCode::PL,
            0b0110 => ConditionCode::VS,
            0b0111 => ConditionCode::VC,
            0b1000 => ConditionCode::HI,
            0b1001 => ConditionCode::LS,
            0b1010 => ConditionCode::GE,
            0b1011 => ConditionCode::LT,
            0b1100 => ConditionCode::GT,
            0b1101 => ConditionCode::LE,
            0b1110 => ConditionCode::AL,
            _ => panic!("Invalid condition code."),
        }
    }
}

pub const REG_NUMBER: usize = 16;
pub const PC: usize = 15;
pub const LR: usize = 14;
pub const SP: usize = 13;

#[derive(Debug)]
pub struct RegFile {
    // r13 sp
    // r14 lr
    // r15 pc
    regs: Vec<Reg>,
    cpsr: Cpsr,
}

impl RegFile {
    pub fn new() -> RegFile {
        RegFile {
            regs: (0..REG_NUMBER).map(|_| Reg::new()).collect(),
            cpsr: Cpsr {
                n: false,
                z: false,
                c: false,
                v: false,
            },
        }
    }

    pub fn get(&self, reg_num: u64) -> u64 {
        self.regs[reg_num as usize].get()
    }

    pub fn get_imm(&self, reg_num: u64, imm: bool) -> u64 {
        if imm {
            reg_num
        } else {
            self.regs[reg_num as usize].get()
        }
    }

    pub fn set(mut self, reg_num: u64, val: u64) -> RegFile {
        self.regs[reg_num as usize] = Reg::new().set(val);
        RegFile {
            regs: self.regs,
            cpsr: Cpsr {
                n: self.cpsr.n,
                z: self.cpsr.z,
                c: self.cpsr.c,
                v: self.cpsr.v,
            },
        }
    }
    pub fn get_cond(&self, cond_code: u64) -> bool {
        let con = ConditionCode::from_u8(cond_code as u8);
        match con {
            ConditionCode::EQ => self.cpsr.z,
            ConditionCode::NE => !self.cpsr.z,
            ConditionCode::HS => self.cpsr.c,
            ConditionCode::LO => !self.cpsr.c,
            ConditionCode::MI => self.cpsr.n,
            ConditionCode::PL => !self.cpsr.n,
            ConditionCode::VS => self.cpsr.v,
            ConditionCode::VC => !self.cpsr.v,
            ConditionCode::HI => self.cpsr.c && !self.cpsr.z,
            ConditionCode::LS => !self.cpsr.c || self.cpsr.z,
            ConditionCode::GE => self.cpsr.n == self.cpsr.v,
            ConditionCode::LT => self.cpsr.n != self.cpsr.v,
            ConditionCode::GT => !self.cpsr.z && (self.cpsr.n == self.cpsr.v),
            ConditionCode::LE => self.cpsr.z || (self.cpsr.n != self.cpsr.v),
            ConditionCode::AL => true,
        }
    }
    pub fn set_cpsr(self, nzcv: u8) -> RegFile {
        RegFile {
            regs: self.regs,
            cpsr: Cpsr::from_u8(nzcv)
        }
    }
    pub fn get_pc(&self) -> u64 {
        self.regs[PC].get()
    }
    pub fn set_lr(mut self) -> RegFile {
        self.regs[LR] = Reg::new().set(self.regs[PC].get());
        RegFile {
            regs: self.regs,
            cpsr: Cpsr {
                n: self.cpsr.n,
                z: self.cpsr.z,
                c: self.cpsr.c,
                v: self.cpsr.v,
            },
        }
    }
    pub fn push_stack(mut self) -> RegFile {
        self.regs[SP] = Reg::new().set(self.regs[SP].get() - 8);
        RegFile {
            regs: self.regs,
            cpsr: Cpsr {
                n: self.cpsr.n,
                z: self.cpsr.z,
                c: self.cpsr.c,
                v: self.cpsr.v,
            },
        }
    }
    pub fn pop_stack(mut self) -> RegFile {
        self.regs[SP] = Reg::new().set(self.regs[SP].get() + 8);
        RegFile {
            regs: self.regs,
            cpsr: Cpsr {
                n: self.cpsr.n,
                z: self.cpsr.z,
                c: self.cpsr.c,
                v: self.cpsr.v,
            },
        }
    }

    pub fn next_pc(self) -> RegFile {
        let next_pc = self.get_pc() + 8;
        self.set(
            PC as u64, next_pc
        )
    }

    pub fn dump_common(&self) -> Vec<u64> {
        self.regs.iter().map(|r| r.get()).collect()
    }

    pub fn dump_cpsr(&self) -> u8 {
        self.cpsr.to_u8()
    }
}