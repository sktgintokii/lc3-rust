use crate::hardware::Vm;

use super::{get_cond_flag, safe_u16_add, sign_extend};

/// An address is computed by sign-extending bits [8:0] to 16 bits and adding
/// this value to the incremented PC.
/// This address is loaded into DR. The condition codes are set, based on whether the
/// value loaded is negative, zero, or positive.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      1110     │     DR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
pub fn lea(instr: u16, vm: &mut Vm) {
    let pc_offset9 = sign_extend(instr & 0x1ff, 9);
    let dr = (instr >> 9) & 0x7;

    let value = safe_u16_add(vm.register.pc, pc_offset9);
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

        vm.register.pc = 17;

        // compute pc+pc_offset9=17+13=30, then write to r2
        lea(0b1110_010_000001101, &mut vm);

        assert_eq!(vm.register.r2, 30);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }
}
