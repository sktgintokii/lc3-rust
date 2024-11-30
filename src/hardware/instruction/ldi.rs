use crate::hardware::Vm;

use super::{get_cond_flag, sign_extend};

/// Load indirect
/// An address is computed by sign-extending bits [8:0] to 16 bits and adding this
/// value to the incremented PC. What is stored in memory at this address is the address
/// of the data to be loaded into DR. The condition codes are set, based on whether the
// value loaded is negative, zero, or positive.
///
/// Encoding:
///
///  15           12 11        9 8                                 0
/// ┌───────────────┬───────────┬───────────────────────────────────┐
/// │      1010     │     DR    │               PCOffset9           │
/// └───────────────┴───────────┴───────────────────────────────────┘
///
pub fn ldi(instr: u16, vm: &mut Vm) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset9 = sign_extend(instr & 0x1ff, 9);

    let first_read_addr = vm.register.pc + pc_offset9;
    let addr = vm.memory.read(first_read_addr);
    let value = vm.memory.read(addr);

    vm.register.update(dr, value);
    vm.register.cond = get_cond_flag(value);
}

#[cfg(test)]
mod test {
    use crate::hardware::instruction::ConditionFlag;

    use super::*;

    #[test]
    fn test() {
        let mut vm = Vm::new();

        vm.register.pc = 1095;
        vm.memory.write(1095 + 33, 458);
        vm.memory.write(458, 101);

        // load memory at addr = pc+33, then save to r3
        ldi(0b1010_011_000100001, &mut vm);

        assert_eq!(vm.register.r3, 101);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }
}
