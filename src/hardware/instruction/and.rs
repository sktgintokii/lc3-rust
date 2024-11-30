use super::{get_cond_flag, sign_extend, Vm};

/// Your good old logical `and` function. Two operation modes, immediate or passing a register.
///
/// 15           12 │11        9│8         6│ 5 │4     3│2         0
/// ┌───────────────┼───────────┼───────────┼───┼───────┼───────────┐
/// │      0101     │     DR    │  SR1      │ 0 │  00   │    SR2    │
/// └───────────────┴───────────┴───────────┴───┴───────┴───────────┘

///  15           12│11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      0101     │     DR    │  SR1      │ 1 │       IMM5        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘
///
pub fn and(instr: u16, vm: &mut Vm) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let value = vm.register.get(sr1) & imm5;
        vm.register.update(dr, value);
        vm.register.cond = get_cond_flag(value);
    } else {
        let sr2 = instr & 0x7;
        let value = vm.register.get(sr1) & vm.register.get(sr2);

        vm.register.update(dr, value);
        vm.register.cond = get_cond_flag(value);
    }
}

#[cfg(test)]
mod test {
    use crate::hardware::instruction::ConditionFlag;

    use super::*;

    #[test]
    fn test_register_mode() {
        let mut vm = Vm::new();

        vm.register.r1 = 4917;
        vm.register.r2 = 98;

        // load r1=105 and r2=-987, then compute bitwise AND result=32, then write to r0
        and(0b_0001_000_001_0_00_010, &mut vm);

        assert_eq!(vm.register.r0, 32);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }

    #[test]
    fn test_immediate_mode() {
        let mut vm = Vm::new();

        vm.register.r1 = 105;

        // load r1=105, then add sr2=7, then write result=1 to r0
        and(0b_0001_000_001_1_00111, &mut vm);

        assert_eq!(vm.register.r0, 1);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }
}
