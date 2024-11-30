use crate::hardware::Vm;

use super::{safe_u16_add, sign_extend};

/// The contents of the register specified by SR are stored in the memory location
/// whose address is obtained as follows: Bits [8:0] are sign-extended to 16 bits and added to the incremented PC.
/// What is in memory at this address is the address of the location to which the data in SR is stored.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      1011     │     SR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
pub fn sti(instr: u16, vm: &mut Vm) {
    let pc_offset9 = sign_extend(instr & 0x1ff, 9);
    let sr = (instr >> 9) & 0x7;

    let value = vm.register.get(sr);
    let addr = vm.memory.read(safe_u16_add(vm.register.pc, pc_offset9));

    vm.memory.write(addr, value);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut vm = Vm::new();

        vm.register.pc = 17;
        vm.memory.write(30, 98);
        vm.register.r2 = 1320;

        // load memory=98 at pc+pc_offset9=17+13=30, then load r2=1320, then write 1320 to memory at 98
        sti(0b1011_010_000001101, &mut vm);

        assert_eq!(vm.memory.read(98), 1320);
    }
}
