use super::Vm;

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

    println!("debug");

    if imm_flag == 1 {
        let imm5 = super::sign_extend(instr & 0x1F, 5);
        let sum = (vm.register.get(sr1) as u32 + imm5 as u32) as u16; // cast to u32 to prevent overflow
        vm.register.update(dr, sum);
    } else {
        let sr2 = instr & 0x7;
        let sum = (vm.register.get(sr1) as u32 + vm.register.get(sr2) as u32) as u16; // cast to u32 to prevent overflow

        vm.register.update(dr, sum);
    }

    vm.register.update_r_cond(dr);
}

#[cfg(test)]
mod tests {
    use crate::hardware::register::{FLG_NEG, FLG_POS, FLG_ZRO};

    use super::*;

    #[test]
    fn test_register_mode() {
        let mut vm = Vm::new();

        let v1: u16 = 4917;
        let v2: u16 = 98;

        // set register
        vm.register.update(1, v1);
        vm.register.update(2, v2);

        // execute instruction
        add(0b_0001_000_001_0_00_010, &mut vm);

        assert_eq!(vm.register.r0, v1 + v2);
        assert_eq!(vm.register.cond, FLG_ZRO);
    }

    #[test]
    fn test_register_mode_with_negative_num() {
        let mut vm = Vm::new();

        let v1: u16 = 105;
        let v2: u16 = 64549; // 65536 - 987 = 64549

        // set register
        vm.register.update(1, v1);
        vm.register.update(2, v2);

        // execute instruction
        add(0b_0001_000_001_0_00_010, &mut vm);

        assert_eq!(vm.register.r0, v1 + v2);
        assert_eq!(vm.register.cond, FLG_NEG);
    }

    #[test]
    fn test_immediate_mode() {
        let mut vm = Vm::new();

        let v1: u16 = 105;

        // set register
        vm.register.update(1, v1);

        // execute instruction
        add(0b_0001_000_001_1_00111, &mut vm);

        assert_eq!(vm.register.r0, v1 + 7);
        assert_eq!(vm.register.cond, FLG_POS);
    }
}
