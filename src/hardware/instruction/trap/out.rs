use std::io::Write;

use crate::hardware::Vm;

pub fn out(vm: &mut Vm) {
    print!("{}", vm.register.r0 as u8 as char);
    std::io::stdout().flush().expect("failed to flush");
}
