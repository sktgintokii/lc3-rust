use crate::hardware::Vm;

use super::{safe_u16_add, sign_extend};

/// The contents of the register specified by SR are stored in the memory location
/// whose address is computed by sign-extending bits [5:0] to 16 bits
/// and adding this value to the contents of the register specified by bits [8:6].
///
///  15           12│11        9│8         6│                      0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      0111     │     SR    │   BaseR   │        Offset6        │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
///
pub fn str(instr: u16, vm: &mut Vm) {
    let offset6 = sign_extend(instr & 0x3f, 6);
    let sr_base = (instr >> 6) & 0x7;
    let sr = (instr >> 9) & 0x7;

    let value = vm.register.get(sr);
    let addr = safe_u16_add(vm.register.get(sr_base), offset6);
    vm.memory.write(addr, value);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut vm = Vm::new();

        vm.register.r0 = 17;
        vm.register.r2 = 8901;

        // load r0=17, then load r2=8901, then write r2 to memory at r0+offset6=17+29=46
        str(0b0111_010_000_011101, &mut vm);

        assert_eq!(vm.memory.read(46), 8901);
    }
}
