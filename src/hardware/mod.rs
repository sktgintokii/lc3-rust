pub mod instruction;
pub mod memory;
pub mod register;

use std::{fs::File, io::BufReader, path::Path};

use byteorder::{BigEndian, ReadBytesExt};
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

    pub fn load_image_from_file<P: AsRef<Path>>(&mut self, file_path: P) {
        let f = File::open(file_path).expect("couldn't open file");
        let mut f = BufReader::new(f);

        let pc_addr = f.read_u16::<BigEndian>().expect("fail to read file");

        let mut addr = pc_addr as u16;
        loop {
            match f.read_u16::<BigEndian>() {
                Ok(instr) => {
                    self.memory.write(addr, instr);
                    addr += 1;
                }
                Err(err) => {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof {
                        println!("Successfully loaded image from file!");
                    } else {
                        eprint!("{err}");
                    }
                    break;
                }
            }
        }
    }

    pub fn launch(&mut self) {
        loop {
            let instr = self.memory.read(self.register.pc);

            self.register.pc += 1;
            instruction::execute_instruction(instr, self);
        }
    }
}
