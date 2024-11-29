use super::Vm;

use add::add;
use and::and;
use br::br;
use jmp::jmp;
use jsr::jsr;
use ld::ld;
use ldi::ldi;
use ldr::ldr;
use not::not;
use st::st;
use str::str;

mod add;
mod and;
mod br;
mod jmp;
mod jsr;
mod ld;
mod ldi;
mod ldr;
mod not;
mod st;
mod str;

pub enum ConditionFlag {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}

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

pub fn get_cond_flag(x: u16) -> u16 {
    if x == 0 {
        ConditionFlag::ZRO as u16
    } else if x >> 15 == 1 {
        ConditionFlag::NEG as u16
    } else {
        ConditionFlag::POS as u16
    }
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
        Some(OpCode::BR) => br(instr, vm),
        Some(OpCode::ADD) => add(instr, vm),
        Some(OpCode::LD) => ld(instr, vm),
        Some(OpCode::ST) => st(instr, vm),
        Some(OpCode::JSR) => jsr(instr, vm),
        Some(OpCode::AND) => and(instr, vm),
        Some(OpCode::LDR) => ldr(instr, vm),
        Some(OpCode::STR) => str(instr, vm),
        Some(OpCode::RTI) => panic!(),
        Some(OpCode::NOT) => not(instr, vm),
        Some(OpCode::LDI) => ldi(instr, vm),
        // Some(OpCode::STI) => (),
        Some(OpCode::JMP) => jmp(instr, vm),
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

pub fn safe_u16_add(x: u16, y: u16) -> u16 {
    (x as u32 + y as u32) as u16
}
