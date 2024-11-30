use std::io::Write;

use crate::hardware::{instruction::get_2bytes_chars, Vm};

pub fn putsp(vm: &mut Vm) {
    let mut addr = vm.register.r0;
    let mut value = vm.memory.read(addr);

    while value != 0 {
        let [c1, c2] = get_2bytes_chars(value);
        print!("{}", c1);

        if c2 != '\0' {
            print!("{}", c2);
        }

        addr += 1;
        value = vm.memory.read(addr);
    }

    std::io::stdout().flush().expect("failed to flush")
}
