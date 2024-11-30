use crate::hardware::Vm;

use super::{get_cond_flag, safe_u16_add, sign_extend};

/// Load base + offset
/// An address is computed by sign-extending bits [5:0] to 16 bits
/// and adding this value to the contents of the register specified by bits [8:6].
/// The contents of memory at this address are loaded into DR.
/// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
///
///  15           12│11        9│8             6│5                 0
/// ┌───────────────┼───────────┼───────────────┼───────────────────┐
/// │      1010     │     DR    │     BaseR     │       Offset6     │
/// └───────────────┴───────────┴───────────────┴───────────────────┘
///
pub fn ldr(instr: u16, vm: &mut Vm) {
    let offset6 = sign_extend(instr & 0x3f, 6);
    let sr = (instr >> 6) & 0x7;
    let dr = (instr >> 9) & 0x7;

    let addr = safe_u16_add(vm.register.get(sr), offset6);
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

        vm.memory.write(53, 132);
        vm.register.r1 = 35;

        // load r1=35, then add offset6=18, then load memory at addr=53, then write 132 to r3
        ldr(0b1010_011_001_010010, &mut vm);

        println!("{:?}", vm.register);

        assert_eq!(vm.register.r3, 132);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }
}
