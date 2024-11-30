use std::process;

use crate::hardware::Vm;

use getc::getc;
use halt::halt;
use out::out;
use puts::puts;
use putsp::putsp;
use trap_in::trap_in;

mod getc;
mod halt;
mod out;
mod puts;
mod putsp;
mod trap_in;

enum TrapCode {
    GETC = 32, // 0x20 /* get character from keyboard, not echoed onto the terminal */
    OUT,       /* output a character */
    PUTS,      /* output a word string */
    IN,        /* get character from keyboard, echoed onto the terminal */
    PUTSP,     /* output a byte string */
    HALT,      /* halt the program */
}

fn get_trap_code(instr: u16) -> Option<TrapCode> {
    match instr & 0xff {
        32 => Some(TrapCode::GETC),
        33 => Some(TrapCode::OUT),
        34 => Some(TrapCode::PUTS),
        35 => Some(TrapCode::IN),
        36 => Some(TrapCode::PUTSP),
        37 => Some(TrapCode::HALT),
        _ => None,
    }
}

/// `trap` fn allows interacting with I/O devices
/// First R7 is loaded with the incremented PC.
// (This enables a return to the instruction physically following the TRAP instruction in the original program
/// after the service routine has completed execution.)
/// Then the PC is loaded with the starting address of the system call specified by trap vector8.
/// The starting address is contained in the memory location whose address is obtained by zero-extending trap vector8 to 16 bits.
pub fn trap(instr: u16, vm: &mut Vm) {
    let trap_code = get_trap_code(instr);

    match trap_code {
        Some(TrapCode::GETC) => getc(vm),
        Some(TrapCode::OUT) => out(vm),
        Some(TrapCode::PUTS) => puts(vm),
        Some(TrapCode::IN) => trap_in(vm),
        Some(TrapCode::PUTSP) => putsp(vm),
        Some(TrapCode::HALT) => halt(),
        _ => process::exit(1),
    }
}
