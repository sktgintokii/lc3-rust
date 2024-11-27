use add::add;

use super::Vm;

mod add;

pub enum OpCode {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
}

pub fn get_op_code(instr: u16) -> Option<OpCode> {
    match instr >> 12 {
        0 => Some(OpCode::BR),
        1 => Some(OpCode::ADD),
        2 => Some(OpCode::LD),
        3 => Some(OpCode::ST),
        4 => Some(OpCode::JSR),
        5 => Some(OpCode::AND),
        6 => Some(OpCode::LDR),
        7 => Some(OpCode::STR),
        8 => Some(OpCode::RTI),
        9 => Some(OpCode::NOT),
        10 => Some(OpCode::LDI),
        11 => Some(OpCode::STI),
        12 => Some(OpCode::JMP),
        13 => Some(OpCode::RES),
        14 => Some(OpCode::LEA),
        15 => Some(OpCode::TRAP),
        _ => None,
    }
}

pub fn execute_instruction(instr: u16, vm: &mut Vm) {
    let op_code = get_op_code(instr);

    match op_code {
        // Some(OpCode::BR) => (),
        Some(OpCode::ADD) => add(instr, vm),
        // Some(OpCode::LD) => (),
        // Some(OpCode::ST) => (),
        // Some(OpCode::JSR) => (),
        // Some(OpCode::AND) => (),
        // Some(OpCode::LDR) => (),
        // Some(OpCode::STR) => (),
        Some(OpCode::RTI) => panic!(),
        // Some(OpCode::NOT) => (),
        // Some(OpCode::LDI) => (),
        // Some(OpCode::STI) => (),
        // Some(OpCode::JMP) => (),
        Some(OpCode::RES) => panic!(),
        // Some(OpCode::LEA) => (),
        // Some(OpCode::TRAP) => (),
        _ => panic!(),
    }
}

pub fn sign_extend(x: u16, bit_count: u8) -> u16 {
    let first_bit = (x >> (bit_count - 1)) & 1;
    if first_bit == 1 {
        return x & (0xFFFF << bit_count);
    }
    x
}
