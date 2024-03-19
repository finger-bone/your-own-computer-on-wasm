use std::collections::HashMap;

use wasm_bindgen::{prelude::wasm_bindgen};

use crate::emulator::reg_file::{LR, PC, SP};

use self::{
    preprocess::{expand_ite, expand_push_pop},
    trim::{remove_comments, remove_empty_lines, remove_whitespace},
};

use super::*;
#[derive(Debug)]
pub enum AssemblerIntermediary {
    Assembled(u64),
    Original(String),
}

pub fn assemble_raw(lines: Vec<String>) -> Vec<AssemblerIntermediary> {
    // expand .ascii, .word
    let mut ret = Vec::new();
    let mut it = lines.iter().peekable();
    loop {
        let line = it.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line.starts_with(".asciz") {
            let line = it.next().unwrap();
            let bytes = line.as_bytes().to_vec();
            // .ascii "hello"
            // the first byte is the length of the string
            // and the rest are the ascii values of the string
            // each char will take up a word even if they only use 1 byte
            // for convenience
            ret.push(AssemblerIntermediary::Assembled(bytes.len() as u64));
            for byte in bytes {
                ret.push(AssemblerIntermediary::Assembled(byte as u64));
            }
        } else if line.starts_with(".word") {
            // can be positive or negative
            // but if use 0x or 0b, it must be positive
            let line = it.next().unwrap();
            if line.starts_with("0x") {
                let num = u64::from_str_radix(&line[2..], 16).unwrap();
                ret.push(AssemblerIntermediary::Assembled(num));
            } else if line.starts_with("0b") {
                let num = u64::from_str_radix(&line[2..], 2).unwrap();
                ret.push(AssemblerIntermediary::Assembled(num));
            } else {
                let num = line.parse::<i64>().unwrap();
                ret.push(AssemblerIntermediary::Assembled(num as u64));
            }
        } else {
            ret.push(AssemblerIntermediary::Original(line.clone()));
        }
    }
    ret
}

pub fn generate_label_map(
    lines: &Vec<AssemblerIntermediary>,
) -> (HashMap<String, usize>, Vec<AssemblerIntermediary>) {
    let mut label_map = HashMap::new();
    let mut ret = Vec::<AssemblerIntermediary>::new();
    let mut line_count = 0 as usize;
    for line in lines {
        match line {
            AssemblerIntermediary::Original(line) => {
                if line.ends_with(':') {
                    label_map.insert(
                        line.clone().trim_end_matches(":").to_string(),
                        line_count * 8,
                    );
                } else {
                    ret.push(AssemblerIntermediary::Original(line.clone()));
                    line_count += 1;
                }
            }
            AssemblerIntermediary::Assembled(ref v) => {
                ret.push(AssemblerIntermediary::Assembled(*v));
                line_count += 1;
            }
        }
    }
    (label_map, ret)
}

pub fn parse_cond(postfix: &str) -> u64 {
    if postfix.ends_with("eq") {
        0b0000
    } else if postfix.ends_with("ne") {
        0b0001
    } else if postfix.ends_with("cs") {
        0b0010
    } else if postfix.ends_with("cc") {
        0b0011
    } else if postfix.ends_with("mi") {
        0b0100
    } else if postfix.ends_with("pl") {
        0b0101
    } else if postfix.ends_with("vs") {
        0b0110
    } else if postfix.ends_with("vc") {
        0b0111
    } else if postfix.ends_with("hi") {
        0b1000
    } else if postfix.ends_with("ls") {
        0b1001
    } else if postfix.ends_with("ge") {
        0b1010
    } else if postfix.ends_with("lt") {
        0b1011
    } else if postfix.ends_with("gt") {
        0b1100
    } else if postfix.ends_with("le") {
        0b1101
    } else if postfix.ends_with("al") {
        0b1110
    } else if postfix.is_empty() || postfix.starts_with("s") {
        0b1110
    } else {
        panic!("Unknown condition: {}", postfix);
    }
}

pub fn parse_instruction(l: &str) -> (u64, bool, u64, &str) {
    let parts = l.split_whitespace().collect::<Vec<&str>>();
    let to_parse = parts[0];
    let mut postfix = "";
    let opcode: u64;
    let mut set_flags = false;
    let mut cond_code = 0b1110 as u64;
    let op_name: &str;
    if to_parse.starts_with("nop") {
        opcode = 0b00_0000_0000_0000;
        op_name = "nop";
    } else if to_parse.starts_with("hlt") {
        opcode = 0b00_0000_0000_0001;
        op_name = "hlt";
    } else if to_parse.starts_with("mov") {
        opcode = 0b01_0000_0000_0000;
        postfix = &to_parse[3..];
        op_name = "mov";
    } else if to_parse.starts_with("add") {
        opcode = 0b01_0000_0000_0001;
        postfix = &to_parse[3..];
        op_name = "add";
    } else if to_parse.starts_with("sub") {
        opcode = 0b01_0000_0000_0010;
        postfix = &to_parse[3..];
        op_name = "sub";
    } else if to_parse.starts_with("mul") {
        opcode = 0b01_0000_0000_0011;
        postfix = &to_parse[3..];
        op_name = "mul";
    } else if to_parse.starts_with("div") {
        opcode = 0b01_0000_0000_0100;
        postfix = &to_parse[3..];
        op_name = "div";
    } else if to_parse.starts_with("smul") {
        opcode = 0b01_0000_0000_0101;
        postfix = &to_parse[4..];
        op_name = "smul";
    } else if to_parse.starts_with("sdiv") {
        opcode = 0b01_0000_0000_0110;
        postfix = &to_parse[4..];
        op_name = "sdiv";
    } else if to_parse.starts_with("modu") {
        opcode = 0b01_0000_0000_0111;
        postfix = &to_parse[4..];
        op_name = "modu";
    } else if to_parse.starts_with("smodu") {
        opcode = 0b01_0000_0000_1000;
        postfix = &to_parse[5..];
        op_name = "smodu";
    } else if to_parse.starts_with("mvn") {
        opcode = 0b01_0000_0000_1001;
        postfix = &to_parse[3..];
        op_name = "mvn";
    } else if to_parse.starts_with("and") {
        opcode = 0b01_0000_0000_1010;
        postfix = &to_parse[3..];
        op_name = "and";
    } else if to_parse.starts_with("orr") {
        opcode = 0b01_0000_0000_1011;
        postfix = &to_parse[3..];
        op_name = "orr";
    } else if to_parse.starts_with("eor") {
        opcode = 0b01_0000_0000_1100;
        postfix = &to_parse[3..];
        op_name = "eor";
    } else if to_parse.starts_with("cmp") {
        opcode = 0b01_0000_0001_0101;
        postfix = &to_parse[3..];
        set_flags = true;
        op_name = "cmp";
    } else if to_parse.starts_with("cmn") {
        opcode = 0b01_0000_0001_0110;
        postfix = &to_parse[3..];
        set_flags = true;
        op_name = "cmn";
    } else if to_parse.starts_with("tst") {
        opcode = 0b01_0000_0001_0111;
        postfix = &to_parse[3..];
        set_flags = true;
        op_name = "tst";
    } else if to_parse.starts_with("teq") {
        opcode = 0b01_0000_0001_1000;
        postfix = &to_parse[3..];
        set_flags = true;
        op_name = "teq";
    } else if to_parse.starts_with("lsl") {
        opcode = 0b01_0000_0010_0000;
        postfix = &to_parse[3..];
        op_name = "lsl";
    } else if to_parse.starts_with("lsr") {
        opcode = 0b01_0000_0010_0001;
        postfix = &to_parse[3..];
        op_name = "lsr";
    } else if to_parse.starts_with("asr") {
        opcode = 0b01_0000_0010_0010;
        postfix = &to_parse[3..];
        op_name = "asr";
    } else if to_parse.starts_with("rol") {
        opcode = 0b01_0000_0010_0011;
        postfix = &to_parse[3..];
        op_name = "rol";
    } else if to_parse.starts_with("ror") {
        opcode = 0b01_0000_0010_0100;
        postfix = &to_parse[3..];
        op_name = "ror";
    } else if to_parse.starts_with("mvi") {
        opcode = 0b01_0000_0011_0000;
        postfix = &to_parse[3..];
        op_name = "mvi";
    } else if to_parse.starts_with("ldr") {
        opcode = 0b10_0000_0000_0000;
        postfix = &to_parse[3..];
        op_name = "ldr";
    } else if to_parse.starts_with("str") {
        opcode = 0b10_0000_0000_0001;
        postfix = &to_parse[3..];
        op_name = "str";
    } else if to_parse.starts_with("pop") {
        opcode = 0b10_0000_0000_0010;
        postfix = &to_parse[3..];
        op_name = "pop";
    } else if to_parse.starts_with("push") {
        opcode = 0b10_0000_0000_0011;
        postfix = &to_parse[4..];
        op_name = "push";
    } else if to_parse.starts_with("bl") {
        opcode = 0b11_0000_0000_0001;
        postfix = &to_parse[2..];
        op_name = "bl";
    } else if to_parse.starts_with("b") {
        opcode = 0b11_0000_0000_0000;
        postfix = &to_parse[1..];
        op_name = "b";
    } else {
        panic!("Unknown instruction: {}", to_parse);
    }
    if postfix.starts_with("s") {
        set_flags = true;
    } else {
        cond_code = parse_cond(postfix);
    }
    (opcode, set_flags, cond_code, op_name)
}

const NO_OPERANDS: [&str; 2] = ["nop", "hlt"];
const D_OPERAND: [&str; 3] = ["mvi", "pop", "push"];
const C_OPERAND: [&str; 2] = ["b", "bl"];
const B_C_OPERAND: [&str; 4] = ["cmp", "cmn", "tst", "teq"];
const D_C_OPERAND: [&str; 2] = ["mov", "mvn"];
const D_B_C_OPERAND: [&str; 16] = [
    "add", "sub", "mul", "div", "smul", "sdiv", "modu", "smodu", "and", "orr", "eor", "lsl", "lsr",
    "asr", "rol", "ror",
];
const D_A_B_C_OPERAND: [&str; 2] = ["str", "ldr"];

pub fn parse_operand(operand: &str, label_map: &HashMap<String, usize>) -> (u64, bool) {
    if operand.starts_with("lr") {
        (LR as u64, false)
    } else if operand.starts_with("sp") {
        (SP as u64, false)
    } else if operand.starts_with("pc") {
        (PC as u64, false)
    } else if operand.starts_with("r") {
        (operand[1..].parse::<u64>().unwrap(), false)
    } else if operand.starts_with("#") {
        (operand[1..].parse::<u64>().unwrap(), true)
    } else if operand.starts_with("=") {
        let label = operand[1..].to_string();
        let label = label_map.get(&label).unwrap_or_else(|| {
            panic!("Unknown label: {}", label);
        });
        (*label as u64, true)
    } else {
        panic!("Unknown operand: {}", operand);
    }
}

pub fn split_operands(l: &str) -> Vec<&str> {
    let mut parts = l.split_whitespace().collect::<Vec<&str>>();
    let mut ret = Vec::new();
    for i in 1..parts.len() {
        parts[i] = parts[i].trim_end_matches(',');
        ret.push(parts[i]);
    }
    ret
}

pub fn operand_to_u64(l: &str, op_name: &str, label_map: HashMap<String, usize>) -> (u64, bool) {
    // 44 bits
    // 4 for d, 4 for a, 4 for b, 32 for c
    if NO_OPERANDS.contains(&op_name) {
        (0, false)
    } else if D_OPERAND.contains(&op_name) {
        let splitted = split_operands(l);
        let (rd, _) = parse_operand(splitted[0], &label_map);
        (rd << 40, false)
    } else if C_OPERAND.contains(&op_name) {
        let splitted = split_operands(l);
        let (rc, is_imm) = parse_operand(splitted[0], &label_map);
        (rc, is_imm)
    } else if B_C_OPERAND.contains(&op_name) {
        let splitted = split_operands(l);
        let (rb, _) = parse_operand(splitted[0], &label_map);
        let (rc, is_imm) = parse_operand(splitted[1], &label_map);
        (rb << 32 | rc, is_imm)
    } else if D_C_OPERAND.contains(&op_name) {
        let splitted = split_operands(l);
        let (rd, _) = parse_operand(splitted[0], &label_map);
        let (rc, is_imm) = parse_operand(splitted[1], &label_map);
        (rd << 40 | rc, is_imm)
    } else if D_B_C_OPERAND.contains(&op_name) {
        let splitted = split_operands(l);
        let (rd, _) = parse_operand(splitted[0], &label_map);
        let (rb, _) = parse_operand(splitted[1], &label_map);
        let (rc, is_imm) = parse_operand(splitted[2], &label_map);
        (rd << 40 | rb << 32 | rc, is_imm)
    } else if D_A_B_C_OPERAND.contains(&op_name) {
        let splitted = split_operands(l);
        if splitted.len() == 2 {
            let (rd, _) = parse_operand(splitted[0], &label_map);
            let (ra, _) = parse_operand(splitted[1], &label_map);
            (rd << 40 | ra << 36, true)
        } else {
            let (rd, _) = parse_operand(splitted[0], &label_map);
            let (ra, _) = parse_operand(splitted[1], &label_map);
            let (rb, _) = parse_operand(splitted[2], &label_map);
            let (rc, is_imm) = parse_operand(splitted[3], &label_map);
            (rd << 40 | ra << 36 | rb << 32 | rc, is_imm)
        }
    } else {
        panic!("Unknown operand: {}", op_name);
    }
}

pub fn to_binary(lines: &Vec<String>) -> Vec<u64> {
    // expand the raw
    let intermediate = assemble_raw(lines.clone());
    // map the label to the line number
    let (label_map, intermediate) = generate_label_map(&intermediate);
    // start actually assembling
    let mut ret = Vec::<u64>::new();
    for line in intermediate {
        match line {
            AssemblerIntermediary::Assembled(v) => {
                ret.push(v);
            }
            AssemblerIntermediary::Original(s) => {
                let (opcode, set_flags, cond_code, op_name) = parse_instruction(&s);
                let (operand, is_imm) = operand_to_u64(&s, op_name, label_map.clone());
                // 64 bit instruction
                // the highest 4 bits are the condition code
                // the next bit is the set flags bit
                // the next bit is is_imm
                ret.push(
                    cond_code << 60
                        | (set_flags as u64) << 59
                        | (is_imm as u64) << 58
                        | opcode << 44
                        | operand,
                )
            }
        }
    }
    ret
}

pub fn to_memory(assembled: Vec<u64>) -> Vec<u8> {
    let mut ret = Vec::new();
    for v in assembled {
        ret.push((v >> 56) as u8);
        ret.push((v >> 48) as u8);
        ret.push((v >> 40) as u8);
        ret.push((v >> 32) as u8);
        ret.push((v >> 24) as u8);
        ret.push((v >> 16) as u8);
        ret.push((v >> 8) as u8);
        ret.push(v as u8);
    }
    ret
}

pub fn preprocess(text: String) -> Vec<String> {
    let mut lines = text.split("\n").map(|x| String::from(x)).collect();
    lines = remove_whitespace(&lines);
    lines = remove_comments(&lines);
    lines = remove_empty_lines(&lines);
    lines = expand_ite(&lines);
    lines = expand_push_pop(&lines);
    lines
}

#[wasm_bindgen]
pub fn assemble(text: &str) -> Vec<u8> {
    let lines = preprocess(text.to_string());
    let assembled = to_binary(&lines);
    to_memory(assembled)
}
