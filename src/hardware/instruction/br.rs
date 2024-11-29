use super::{safe_u16_add, sign_extend, Vm};

/// The branching operation; means to go somewhere else in the assembly code
/// depending on whether some conditions are met.
/// The condition codes specified by the state of bits [11:9] are tested.
/// If bit [11] is set, N is tested; if bit [11] is clear, N is not tested.
/// If bit [10] is set, Z is tested. If any of the condition codes tested is set,
/// the program branches to the location specified by
/// adding the sign-extended PCOffset9 field to the incremented PC.
///
/// 15           12 │11 │10 │ 9 │8                                 0
/// ┌───────────────┼───┼───┼───┼───────────────────────────────────┐
/// │      0000     │ N │ Z │ P │             PCOffset9             │
/// └───────────────┴───┴───┴───┴───────────────────────────────────┘
///
pub fn br(instr: u16, vm: &mut Vm) {
    let cond_flag = (instr >> 9) & 0x7;
    let pc_offset9 = instr & 0x1ff;

    if (vm.register.cond & cond_flag) != 0 {
        vm.register.pc = safe_u16_add(vm.register.pc, sign_extend(pc_offset9, 9));
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_match() {
        let mut vm = Vm::new();

        vm.register.pc = 97;
        vm.register.cond = 4;

        // load condition flag = 4 (=NEG), then compare to cond=4, then load pc=97, then add pc_offset9=107, save result=204 to pc
        br(0b0000_1_0_0_001101011, &mut vm);

        assert_eq!(vm.register.pc, 204);
    }

    #[test]
    fn test_mismatch() {
        let mut vm = Vm::new();

        vm.register.pc = 97;
        vm.register.cond = 2;

        // load condition flag = 4 (=NEG), then compare to cond=2
        br(0b0000_1_0_0_001101011, &mut vm);

        assert_eq!(vm.register.pc, 97);
    }
}
