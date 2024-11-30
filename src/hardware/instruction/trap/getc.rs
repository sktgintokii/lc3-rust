use std::io::Read;

use super::super::Vm;

pub fn getc(vm: &mut Vm) {
    let mut buf = [0; 1];
    std::io::stdin().read_exact(&mut buf).unwrap();

    vm.register.r0 = buf[0] as u16;
}
