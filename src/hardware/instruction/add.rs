use super::{get_cond_flag, safe_u16_add, sign_extend, Vm};

/// ADD takes two values and stores them in a register.
/// In register mode, the second value to add is found in a register.
/// In immediate mode, the second value is embedded in the right-most 5 bits of the instruction.
/// Values which are shorter than 16 bits need to be sign extended.
/// Any time an instruction modifies a register, the condition flags need to be updated
/// If bit [5] is 0, the second source operand is obtained from SR2.
/// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
/// In both cases, the second source operand is added to the contents of SR1 and the result stored in DR.
/// The condition codes are set, based on whether the result is negative, zero, or positive.
/// Encoding:
///
/// 15           12 │11        9│8         6│ 5 │4     3│2         0
/// ┌───────────────┼───────────┼───────────┼───┼───────┼───────────┐
/// │      0001     │     DR    │  SR1      │ 0 │  00   │    SR2    │
/// └───────────────┴───────────┴───────────┴───┴───────┴───────────┘
///
///  15           12│11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      0001     │     DR    │  SR1      │ 1 │       IMM5        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘
pub fn add(instr: u16, vm: &mut Vm) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let value = safe_u16_add(vm.register.get(sr1), imm5);
        vm.register.update(dr, value);
        vm.register.cond = get_cond_flag(value);
    } else {
        let sr2 = instr & 0x7;
        let value = safe_u16_add(vm.register.get(sr1), vm.register.get(sr2));

        vm.register.update(dr, value);
        vm.register.cond = get_cond_flag(value);
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::instruction::ConditionFlag;

    use super::*;

    #[test]
    fn test_register_mode() {
        let mut vm = Vm::new();

        vm.register.r1 = 4917;
        vm.register.r2 = 98;

        // load r1=4917 and r2=98, then add the values=>5015, then write to r0
        add(0b_0001_000_001_0_00_010, &mut vm);

        assert_eq!(vm.register.r0, 5015);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }

    #[test]
    fn test_register_mode_with_negative_num() {
        let mut vm = Vm::new();

        vm.register.r1 = 105;
        vm.register.r2 = 64549; // -987

        // load r1=105 and r2=-987, then add the values=>-882 (=64654), then write to r0
        add(0b_0001_000_001_0_00_010, &mut vm);

        assert_eq!(vm.register.r0, 64654);
        assert_eq!(vm.register.cond, ConditionFlag::NEG as u16);
    }

    #[test]
    fn test_immediate_mode() {
        let mut vm = Vm::new();

        vm.register.r6 = 16384;

        // load r6=16384, then add sr2=29 (=-3), then write result=16381 to r6
        add(0b_0001_110_110_1_11101, &mut vm);

        assert_eq!(vm.register.r6, 16381);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }
}
