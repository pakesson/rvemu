mod decoder;
mod emulator;
mod instruction;
mod types;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use emulator::Emulator;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(());
    }
    let mut file = File::open(&args[1])?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut emu = Emulator::new(data);
    emu.run();

    emu.print_state();

    Ok(())
}
