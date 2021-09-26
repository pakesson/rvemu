mod emulator;
mod types;

use emulator::{Emulator};

fn main() {
    let code = vec![
        0x13, 0x05, 0x60, 0x00, // addi a0,x0,6 (li a0,6)
        0x93, 0x05, 0x40, 0x00, // addi a1,x0,4 (li a1,4)
        0x33, 0x05, 0xb5, 0x00, // add a0,a0,a1
    ];

    let mut emu = Emulator::new(code);
    emu.run();
}
