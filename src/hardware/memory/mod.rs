use std::io::Read;

const MAX_SIZE: usize = 65536; // 16 bit word size

enum MemoryMappedRegister {
    MrKbsr = 0xfe00, // keyboard status
    MrKbdr = 0xfe02, // keyboard data
}

pub struct Memory([u16; MAX_SIZE]);

impl Memory {
    pub fn new() -> Self {
        Self([0; MAX_SIZE])
    }

    fn handle_keyboard(&mut self) {
        let mut buf = [0; 1];
        std::io::stdin().read_exact(&mut buf).unwrap();

        if buf[0] != 0 {
            self.write(MemoryMappedRegister::MrKbsr as u16, 1 << 15);
            self.write(MemoryMappedRegister::MrKbdr as u16, buf[0] as u16);
        } else {
            self.write(MemoryMappedRegister::MrKbsr as u16, 0);
        }
    }

    pub fn read(&mut self, addr: u16) -> u16 {
        if addr == MemoryMappedRegister::MrKbsr as u16 {
            self.handle_keyboard();
        }

        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u16) {
        self.0[addr as usize] = value;
    }
}
