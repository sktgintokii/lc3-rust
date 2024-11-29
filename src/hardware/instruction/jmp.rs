use super::Vm;

/// The program unconditionally jumps to the location specified by the contents of the base register.
/// Bits [8:6] identify the base register. `RET` is listed as a separate instruction
/// in the specification, since it is a different keyword in assembly.
/// However, it is actually a special case of JMP. RET happens whenever R1 is 7.
///
///  15           12│11        9│8         6│ 5                    0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      1100     │    000    │   BaseR   │       000000          │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
///  15           12│11        9│8         6│ 5                    0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      1100     │    000    │    111    │       000000          │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
///
pub fn jmp(instr: u16, vm: &mut Vm) {
    let r1 = (instr >> 6) & 0x7;
    vm.register.pc = vm.register.get(r1);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut vm = Vm::new();

        vm.register.r5 = 16;

        // load r5=16, then write to pc
        jmp(0b1100_000_101_000000, &mut vm);

        assert_eq!(vm.register.pc, 16);
    }
}
