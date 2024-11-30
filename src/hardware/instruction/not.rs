use super::{get_cond_flag, Vm};

/// Simple binary negation.
/// 15           12 │11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      1001     │     DR    │     SR    │ 1 │      11111        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘
///
pub fn not(instr: u16, vm: &mut Vm) {
    let dr = (instr >> 9) & 0x7;
    let sr = (instr >> 6) & 0x7;

    let value = !vm.register.get(sr);
    vm.register.update(dr, value);

    let cond_flag = get_cond_flag(value);
    vm.register.cond = cond_flag;
}

#[cfg(test)]
mod test {

    use crate::hardware::instruction::ConditionFlag;

    use super::*;

    #[test]
    fn test() {
        let mut vm = Vm::new();

        vm.register.r5 = 0b1101_1011_1110_0011;

        // load r5, then negate the value, then write result to r4
        not(0b1001_100_101_1_11111, &mut vm);

        assert_eq!(vm.register.r4, 0b0010_0100_0001_1100);
        assert_eq!(vm.register.cond, ConditionFlag::POS as u16);
    }
}
