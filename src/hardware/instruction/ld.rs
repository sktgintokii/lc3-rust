use crate::hardware::Vm;

use super::{get_cond_flag, safe_u16_add, sign_extend};

/// An address is computed by sign-extending bits [8:0] to 16 bits and
/// adding this value to the incremented PC.
/// The contents of memory at this address are loaded into DR.
/// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      0010     │     DR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
///
pub fn ld(instr: u16, vm: &mut Vm) {
    let pc_offset9 = instr & 0x1ff;
    let dr = (instr >> 9) & 0x7;

    let addr = safe_u16_add(vm.register.pc, sign_extend(pc_offset9, 9));
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

        vm.memory.write(69, 132);
        vm.register.pc = 35;

        // load pc=35, then add pc_offset9=34, then load memory at addr=69, then write 132 to r3
        ld(0b0010_011_000100010, &mut vm);

        assert_eq!(vm.register.r3, 132);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }
}
