use std::io::prelude::*;
use std::fs::File;

mod disassembler;
mod emulator;
mod state_8080;

use state_8080::State8080;

fn main() {
    let file_name = std::env::args().nth(1).expect("Pass file name as first argument");
    let mut file = File::open(&file_name).expect(&format!("Unable to open file '{}'", file_name));
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    while buffer.len() < 0x10000 {
        buffer.push(0);
    }
    let mut state = State8080::empty_state();
    state.memory = buffer;
    loop {
        emulator::emulate_8080_op(&mut state);
    }
}
