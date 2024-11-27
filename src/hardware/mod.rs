pub mod instruction;
pub mod register;

use register::Register;

pub struct Vm {
    register: Register,
}

impl Vm {
    pub fn new() -> Self {
        let register = Register::new();

        Self { register }
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
