use wasm_bindgen::prelude::*;
use super::alu::Alu;
use super::decoder::Decoder;
use super::mem_addr_calculator::MemAddressCalculator;
use super::wire::{SingleWire, Wire};
use super::mem::Mem;
use super::instr::*;
use super::reg_file::*;

pub const MEM_SIZE: usize = 4 * 1024;

#[wasm_bindgen]
pub struct CoreSys {
    pub op: Wire,
    pub cond: Wire,
    pub r_a: Wire,
    pub r_b: Wire,
    pub r_c: Wire,
    pub r_c_imm: SingleWire,
    pub r_d_mem: Wire,
    pub out_a: Wire,
    pub out_b: Wire,
    pub out_c: Wire,
    pub out_d_mem: Wire,
    pub out_m_b: Wire,
    pub out_m_o: Wire,
    pub out_m_s: Wire,
    pub addr_bus: Wire,
    pub data_bus: Wire,
    pub instr: Wire,
    pub pc_mem: Wire,
    pub write_flags: SingleWire,
    pub write_regs: SingleWire,
    pub int_data: Wire,
    pub int: Wire,
    pub query: Wire,

    int_table: Vec<u64>,
    memory: Mem,
    decoder: Decoder,
    alu: Alu,
    mem_cal: MemAddressCalculator,
    reg_file: RegFile,
}

#[wasm_bindgen]
impl CoreSys {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CoreSys {
        let mut ret = CoreSys {
            op: Wire::new(),
            cond: Wire::new(),
            r_a: Wire::new(),
            r_b: Wire::new(),
            r_c: Wire::new(),
            r_c_imm: SingleWire::new(),
            r_d_mem: Wire::new(),
            out_a: Wire::new(),
            out_b: Wire::new(),
            out_c: Wire::new(),
            out_d_mem: Wire::new(),
            out_m_b: Wire::new(),
            out_m_o: Wire::new(),
            out_m_s: Wire::new(),
            addr_bus: Wire::new(),
            data_bus: Wire::new(),
            instr: Wire::new(),
            write_flags: SingleWire::new(),
            pc_mem: Wire::new(),
            write_regs: SingleWire::new(),
            int_data: Wire::new(),
            int: Wire::new(),
            query: Wire::new(),

            int_table: Vec::new(),
            memory: Mem::new(vec![0; MEM_SIZE]),
            decoder: Decoder::new(),
            alu: Alu::new(),
            mem_cal: MemAddressCalculator::new(),
            reg_file: RegFile::new(),
        };
        ret = ret.set_pc_sp();
        ret
    }
}

#[wasm_bindgen]
impl CoreSys {
    pub fn dump_mem(&self) -> Vec<u8> {
        self.memory.dump()
    }
    pub fn dump_common_regs(&self) -> Vec<u64> {
        self.reg_file.dump_common()
    }
    pub fn dump_cpsr(&self) -> u8 {
        self.reg_file.dump_cpsr()
    }
}

#[wasm_bindgen]
impl CoreSys {
    pub fn fetch(mut self) -> CoreSys {
        self.instr = self.instr.set(
            self.memory.get_word(
                self.reg_file.get_pc()
            )
        );
        self.reg_file = self.reg_file.next_pc();
        self
    }
    pub fn set_int_table(mut self, table: Vec<u64>) -> CoreSys {
        self.int_table = table;
        self
    }
    pub fn decode(mut self) -> CoreSys {
        let instr = self.decoder.decode(self.instr.get());
        self.op = self.op.set(instr.op_code);
        self.cond = self.cond.set(instr.cond_code);
        self.r_d_mem = self.r_d_mem.set(instr.reg_d_mem);
        self.r_a = self.r_a.set(instr.reg_a);
        self.r_b = self.r_b.set(instr.reg_b);
        self.r_c = self.r_c.set(instr.reg_c);
        self.r_c_imm = self.r_c_imm.set(instr.c_is_imm);
        self.write_flags = self.write_flags.set(instr.set_flags);
        let decoded_op = Operation::new(instr.op_code);
        if decoded_op == Operation::B {
            self.r_d_mem = self.r_d_mem.set(
                PC as u64
            )
        } else if decoded_op == Operation::Bl {
            self.reg_file = self.reg_file.set_lr();
            self.r_d_mem = self.r_d_mem.set(
                PC as u64
            )
        }
        if decoded_op == Operation::Push {
            let next_sp = self.reg_file.get(SP as u64) - 8;
            self.reg_file = self.reg_file.set(
                SP as u64, next_sp
            )
        }
        self.write_regs = self.write_regs.set(
            !(decoded_op == Operation::Cmp || decoded_op == Operation::Cmn || decoded_op == Operation::Teq || decoded_op == Operation::Tst)
        );
        self
    }
    pub fn read_reg(mut self) -> CoreSys {
        let op_type: OperationType = decode_op_type(self.op.get());
        match op_type {
            OperationType::DataProc | OperationType::Special | OperationType::Branch => {
                self.out_a = self.out_a.set(self.reg_file.get(self.r_a.get()));
                self.out_b = self.out_b.set(self.reg_file.get(self.r_b.get()));
                self.out_c = self.out_c.set(self.reg_file.get_imm(self.r_c.get(), self.r_c_imm.get()));
            }
            OperationType::Mem => {
                self.out_m_b = self.out_m_b.set(self.reg_file.get(self.r_a.get()));
                self.out_m_o = self.out_m_o.set(self.reg_file.get(self.r_b.get()));
                self.out_m_s = self.out_m_s.set(self.reg_file.get_imm(self.r_c.get(), self.r_c_imm.get()));
                self.out_d_mem = self.out_d_mem.set(self.reg_file.get(self.r_d_mem.get()));
            }
        }
        self
    }
    pub fn execute(mut self) -> CoreSys {
        let code_type = decode_op_type(self.op.get());
        if code_type == OperationType::Mem {
            self.data_bus = self.data_bus.set(self.out_d_mem.get());
        } else {
            let (result, flags, interruption) = self.alu.cal(self.op.get(), self.out_b.get(), self.out_c.get());
            self.int = self.int.set(interruption);
            self.data_bus = self.data_bus.set(result);
            if self.write_flags.get() {
                self.reg_file = self.reg_file.set_cpsr(flags);
            }
        }
        let op = decode_op(self.op.get());
        if op == Operation::Mvi {
            self.data_bus = self.data_bus.set(
                self.int_data.get()
            );
        } else if op == Operation::Qry {
            self.query = self.query.set(
                self.data_bus.get()
            );
        }
        self
    }
    pub fn mem(mut self) -> CoreSys {
        self.addr_bus = self.addr_bus.set(
            self.mem_cal.calculate(self.out_m_b.get(), self.out_m_o.get(), self.out_m_s.get())
        );
        let op_code = Operation::new(self.op.get());
        match op_code {
            Operation::Ldr => {
                self.data_bus = self.data_bus.set(
                    self.memory.get_word(self.addr_bus.get())
                );
            },
            Operation::Str => {
                self.memory = self.memory.set_word(self.addr_bus.get(), self.data_bus.get());
            },
            Operation::Push => {
                self.memory = self.memory.set_word(self.reg_file.get(SP as u64), self.data_bus.get());
            }
            Operation::Pop => {
                self.data_bus = self.data_bus.set(
                    self.memory.get_word(self.reg_file.get(SP as u64))
                )
            }
            _ => {

            }
        };
        self
    }
    pub fn interrupt(mut self, int: u64, data: u64) -> CoreSys {
        self.int = self.int.set(int);
        self.int_data = self.int_data.set(data);
        self
    }
    pub fn write_back(mut self) -> CoreSys {
        let op: Operation = decode_op(self.op.get());
        if op == Operation::Pop {
            let next_sp = self.reg_file.get(SP as u64) + 8;
            self.reg_file = self.reg_file.set(
                SP as u64, next_sp
            )
        }
        self.reg_file = self.reg_file.set(
            self.r_d_mem.get(), self.data_bus.get()
        );
        self
    }
    pub fn load_mem(mut self, val: Vec<u8>) -> CoreSys {
        let mut loaded_mem = vec![0; MEM_SIZE];
        for i in 0..val.len() {
            loaded_mem[i] = val[i];
        }
        self.memory = Mem::new(loaded_mem);
        self
    }
    pub fn set_pc_sp(mut self) -> CoreSys {
        self.reg_file = self.reg_file.set(
            PC as u64, 0
        );
        self.reg_file = self.reg_file.set(
            SP as u64, self.memory.get_size() as u64
        );
        self
    }
    pub fn halted(&self) -> bool {
        let op = Operation::new(self.op.get());
        op == Operation::Hlt
    }

    pub fn step(mut self) -> CoreSys {
        if self.halted() {
            return self;
        }
        if self.int.get() != 0 {
            // bl to the interrupt handler
            let handler = self.int_table[self.int.get() as usize];
            self.reg_file = self.reg_file.set_lr();
            self.reg_file = self.reg_file.set(
                PC as u64, handler
            );
            self.int = self.int.set(0);
            return self;
        }
        self = self.fetch();
        self = self.decode();
        let op = Operation::new(self.op.get());
        if op == Operation::Nop {
            return self;
        } else if op == Operation::Hlt {
            return self;
        }
        self = self.read_reg();
        if self.reg_file.get_cond(self.cond.get()) {
            if op == Operation::Int {
                let out_b = self.out_b.get();
                let out_c = self.out_c.get();
                self = self.interrupt(out_b, out_c);
                return self;
            }
            self = self.execute();
            let op_type = OperationType::new(self.op.get());
            if op_type == OperationType::Mem {
                self = self.mem();
            }
            if self.write_regs.get() {
                self = self.write_back();
            }
        }
        self
    }
}

#[wasm_bindgen]
impl CoreSys {
    pub fn get_reg(&self, idx: u64) -> u64 {
        self.reg_file.get(idx)
    }
    pub fn get_next_instr(&self) -> u64 {
        self.memory.get_word(self.reg_file.get_pc())
    }
    pub fn print(&self) {
        println!("op: {:b}", self.op.get());
        println!("cond: {}", self.cond.get());
        println!("r_a: {}", self.r_a.get());
        println!("r_b: {}", self.r_b.get());
        println!("r_c: {}", self.r_c.get());
        println!("r_c_imm: {}", self.r_c_imm.get());
        println!("r_d_mem: {}", self.r_d_mem.get());
        println!("out_a: {}", self.out_a.get());
        println!("out_b: {}", self.out_b.get());
        println!("out_c: {}", self.out_c.get());
        println!("out_d_mem: {}", self.out_d_mem.get());
        println!("out_m_b: {}", self.out_m_b.get());
        println!("out_m_o: {}", self.out_m_o.get());
        println!("out_m_s: {}", self.out_m_s.get());
        println!("addr_bus: {}", self.addr_bus.get());
        println!("data_bus: {}", self.data_bus.get());
        println!("instr: {}", self.instr.get());
        println!("write_flags: {}", self.write_flags.get());
        println!("pc_mem: {}", self.pc_mem.get());
        println!("CPSR: {}", self.reg_file.dump_cpsr());
        println!("Register,");
        println!("{:?}", self.reg_file.dump_common());
        println!("Interruption: {}", self.int.get());
    }
    pub fn dump_int_table(&self) -> Vec<u64> {
        self.int_table.clone()
    }
    pub fn get_qry(&self) -> u64 {
        self.query.get()
    }
    pub fn get_int_data(&self) -> u64 {
        self.int_data.get()
    }
    pub fn get_int(&self) -> u64 {
        self.int.get()
    }
}