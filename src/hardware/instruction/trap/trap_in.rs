use std::io::{Read, Write};

use crate::hardware::{instruction::get_cond_flag, Vm};

pub fn trap_in(vm: &mut Vm) {
    print!("Enter a  character : ");
    std::io::stdout().flush().expect("failed to flush");

    let mut buf = [0; 1];
    std::io::stdin().read_exact(&mut buf).unwrap();

    let char = buf[0];
    print!("{}", char as char);
    std::io::stdout().flush().expect("failed to flush");

    vm.register.r0 = char as u16;
    vm.register.cond = get_cond_flag(char as u16);
}
