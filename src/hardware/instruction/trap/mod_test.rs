use crate::hardware::Vm;

use std::io;
use std::io::Read;
use std::io::Write;

use std::process;

pub fn trap(instruction: u16, vm: &mut Vm) {
    match instruction & 0xFF {
        0x20 => {
            // Get character
            let mut buffer = [0; 1];
            std::io::stdin().read_exact(&mut buffer).unwrap();
            vm.register.r0 = buffer[0] as u16;
        }
        0x21 => {
            // Write out character
            let c = vm.register.r0 as u8;
            print!("{}", c as char);
            // println!("[*] OUT");
        }
        0x22 => {
            // Puts
            let mut index = vm.register.r0;
            let mut c = vm.memory.read(index);
            while c != 0x0000 {
                print!("{}", (c as u8) as char);
                index += 1;
                c = vm.memory.read(index);
            }
            io::stdout().flush().expect("failed to flush");
        }
        0x23 => {
            // In, Print a prompt on the screen and read a single character from the keyboard. The character is echoed onto the console monitor, and its ASCII code is copied into R0.The high eight bits of R0 are cleared.
            print!("Enter a  character : ");
            io::stdout().flush().expect("failed to flush");
            let char = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u16)
                .unwrap();
            vm.register.update(0, char);
        }
        0x24 => {
            // Putsp
            let mut index = vm.register.r0;
            let mut c = vm.memory.read(index);
            while c != 0x0000 {
                let c1 = ((c & 0xFF) as u8) as char;
                print!("{}", c1);
                let c2 = ((c >> 8) as u8) as char;
                if c2 != '\0' {
                    print!("{}", c2);
                }
                index += 1;
                c = vm.memory.read(index);
            }
            io::stdout().flush().expect("failed to flush");
        }
        0x25 => {
            println!("HALT detected");
            io::stdout().flush().expect("failed to flush");
            process::exit(1);
        }
        _ => {
            process::exit(1);
        }
    }
}
