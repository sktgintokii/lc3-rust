pub mod instruction;
pub mod memory;
pub mod register;

use memory::Memory;
use register::Register;

pub struct Vm {
    register: Register,
    memory: Memory,
}

impl Vm {
    pub fn new() -> Self {
        let register = Register::new();
        let memory: Memory = Memory::new();

        Self { register, memory }
    }

    pub fn launch(&mut self) {
        loop {
            // TODO: fetch instr

            // TODO: execute instr

            instruction::execute_instruction(0x0000, self);
            println!("running");
        }
    }
}
