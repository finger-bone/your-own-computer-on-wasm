use super::{cpsr::Cpsr, instr::Operation};

pub struct Alu {}

impl Alu {
    pub fn new() -> Alu {
        Alu {}
    }
    pub fn cal(&self, op: u64, b: u64, c: u64) -> (u64, u8, u64) {
        let operation = Operation::new(op);
        match operation {
            // n z c v
            // negative, zero, carry, overflow
            Operation::Mov => {
                let result = c;
                (
                    result,
                    Cpsr {
                        n: ((c >> 63) & 1) == 1,
                        z: c == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0,
                )
            }
            Operation::Add => {
                let result = b.wrapping_add(c);
                let carry = if b > u64::MAX - c { 1 } else { 0 };
                let overflow = ((b as i64).overflowing_add(c as i64).1) as u8;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: overflow == 1,
                        c: carry == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Cmn => {
                let result = b.wrapping_add(c);
                let carry = if b > u64::MAX - c { 1 } else { 0 };
                let overflow = ((b as i64).overflowing_add(c as i64).1) as u8;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: overflow == 1,
                        c: carry == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Sub => {
                let result = b.wrapping_sub(c);
                let carry = if b < c { 1 } else { 0 };
                let overflow = ((b as i64).overflowing_sub(c as i64).1) as u8;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: overflow == 1,
                        c: carry == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Cmp => {
                let result = b.wrapping_sub(c);
                let carry = if b < c { 1 } else { 0 };
                let overflow = ((b as i64).overflowing_sub(c as i64).1) as u8;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: overflow == 1,
                        c: carry == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Mul => {
                let result = b.wrapping_mul(c);
                let carry = if b > u64::MAX / c { 1 } else { 0 };
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: carry == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Div => {
                if c == 0 {
                    return (
                        0,
                        Cpsr {
                            n: false,
                            z: false,
                            v: false,
                            c: false,
                        }
                        .to_u8(),
                        1,
                    )
                }
                let result = b / c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::SMul => {
                let result = (b as i64).wrapping_mul(c as i64) as u64;
                let overflow = ((b as i64).overflowing_mul(c as i64).1) as u8;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: overflow == 1,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::SDiv => {
                if c == 0 {
                    return (
                        0,
                        Cpsr {
                            n: false,
                            z: false,
                            v: false,
                            c: false,
                        }
                        .to_u8(),
                        1,
                    )
                }
                let result = (b as i64 / c as i64) as u64;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Modu => {
                if c == 0 {
                    return (
                        0,
                        Cpsr {
                            n: false,
                            z: false,
                            v: false,
                            c: false,
                        }
                        .to_u8(),
                        1,
                    )
                }
                let result = b % c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::SModu => {
                if c == 0 {
                   return (
                        0,
                        Cpsr {
                            n: false,
                            z: false,
                            v: false,
                            c: false,
                        }
                        .to_u8(),
                        1,
                    )
                }
                let result = (b as i64 % c as i64) as u64;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Mvn => {
                let result = !c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0,
                )
            }
            Operation::And => {
                let result = b & c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0,
                )
            }
            Operation::Tst => {
                let result = b & c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Orr => {
                let result = b | c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Eor => {
                let result = b ^ c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Teq => {
                let result = b ^ c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: false,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Lsl => {
                let result = b << c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: ((b << (c - 1)) & 0x8000000000000000) != 0,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Lsr => {
                let result = b >> c;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: ((b >> (c - 1)) & 1) == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Asr => {
                let result = (b as i64 >> c) as u64;
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: ((b as i64 >> (c - 1)) & 1) == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Rol => {
                let c = (c % 64) as u32;
                let result = b.rotate_left(c);
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: ((b.rotate_left(c - 1)) & 1) == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Ror => {
                let c = (c % 64) as u32;
                let result = b.rotate_right(c);
                (
                    result,
                    Cpsr {
                        n: ((result >> 63) & 1) == 1,
                        z: result == 0,
                        v: false,
                        c: ((b.rotate_right(c - 1)) & 1) == 1,
                    }
                    .to_u8(),
                    0
                )
            }
            Operation::Str => (c, 0, 0),
            Operation::Push => (c, 0, 0),
            Operation::Bl => (c, 0, 0),
            Operation::B => (c, 0, 0),
            Operation::Qry => (c, 0, 0),
            _ => (0, 0, 0),
        }
    }
}
