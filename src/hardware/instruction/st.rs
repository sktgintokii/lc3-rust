use super::{safe_u16_add, sign_extend, Vm};

/// The contents of the register specified by SR are stored in the memory location
/// whose address is computed by sign-extending bits [8:0] to 16 bits and adding
/// this value to the incremented PC.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      0011     │     SR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
pub fn st(instr: u16, vm: &mut Vm) {
    let pc_offset9 = sign_extend(instr & 0x1ff, 9);
    let sr = (instr >> 9) & 0x7;

    let value = vm.register.get(sr);
    let addr = safe_u16_add(vm.register.pc, pc_offset9);
    vm.memory.write(addr, value);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut vm = Vm::new();

        vm.register.pc = 17;
        vm.register.r2 = 8901;

        // load r2=8901, then write to memory at pc+pc_offset6=17+157=174
        st(0b0011_010_010011101, &mut vm);

        assert_eq!(vm.memory.read(174), 8901);
    }
}
