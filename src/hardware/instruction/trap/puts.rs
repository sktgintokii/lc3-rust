use std::io::Write;

use super::super::Vm;

pub fn puts(vm: &mut Vm) {
    let mut addr = vm.register.r0;
    let mut char = vm.memory.read(addr) as u8;

    while char != 0 {
        print!("{}", char as char);

        addr += 1;
        char = vm.memory.read(addr) as u8;
    }

    std::io::stdout().flush().expect("failed to flush");
}
