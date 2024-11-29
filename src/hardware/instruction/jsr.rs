use super::{safe_u16_add, sign_extend, Vm};

/// First, the incremented PC is saved in R7.
/// This is the linkage back to the calling routine.
/// Then the PC is loaded with the address of the first instruction of the subroutine,
/// causing an unconditional jump to that address.
/// The address of the subroutine is obtained from the base register (if bit [11] is 0),
/// or the address is computed by sign-extending bits [10:0] and adding this value to the incremented PC (if bit [11] is 1).
///
///  15           12│11 │10
/// ┌───────────────┼───┼───────────────────────────────────────────┐
/// │      0100     │ 1 │                PCOffset11                 │
/// └───────────────┴───┴───────────────────────────────────────────┘
///  15           12│11 │10    9│8     6│5                         0
/// ┌───────────────┼───┼───────┼───────┼───────────────────────────┐
/// │      0100     │ 0 │   00  │ BaseR │           000000          │
/// └───────────────┴───┴───────┴───────┴───────────────────────────┘
///
pub fn jsr(instr: u16, vm: &mut Vm) {
    let long_flag = (instr >> 11) & 1;

    vm.register.r7 = vm.register.pc;
    if long_flag == 1 {
        // JSR
        let pc_offset11 = instr & 0x7ff;
        vm.register.pc = safe_u16_add(vm.register.pc, sign_extend(pc_offset11, 11));
    } else {
        // JSRR
        let sr1 = (instr >> 6) & 0x7;
        vm.register.pc = sr1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut vm = Vm::new();

        vm.register.pc = 98; // write 98 to pc

        // load pc=98, then write to r7, then add pc_offset11=860, write result=958 to pc
        jsr(0b_0100_1_01101011100, &mut vm);

        assert_eq!(vm.register.pc, 958);
        assert_eq!(vm.register.r7, 98);
    }

    #[test]
    fn test_register_mode() {
        let mut vm = Vm::new();

        vm.register.pc = 98; // write 98 to pc
        vm.register.r3 = 1092;

        // load pc=98, then write to r7, then load r3=1092, then write r3=1092 to pc
        jsr(0b_0100_0_00_011_000000, &mut vm);

        assert_eq!(vm.register.r3, 1092);
    }
}
