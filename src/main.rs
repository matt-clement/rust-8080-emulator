use std::io::prelude::*;
use std::fs::File;

mod disassembler;
mod emulator;
mod parity;
mod sign;
mod space_invaders;
mod state_8080;

use state_8080::State8080;

fn main() {
    let cmd = std::env::args().nth(1).expect("First argument should be either diag or space-invaders");
    let file_name = std::env::args().nth(2).expect("Pass file name as second argument");
    match cmd.as_str() {
        "diag" => run_diag(&file_name),
        "space-invaders" => run_space_invaders(&file_name),
        x => {
            eprintln!("Subcommand '{}' not found.", x);
            std::process::exit(1);
        }
    }
}

fn run_space_invaders(bin_file_name: &str) {
    let mut file = File::open(&bin_file_name).expect(&format!("Unable to open file '{}'", bin_file_name));
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    while buffer.len() < 0x10000 {
        buffer.push(0);
    }
    let mut state = State8080::empty_state();
    state.memory = buffer;

    space_invaders::start(state);
}

fn run_diag(bin_file_name: &str) {
    let mut file = File::open(&bin_file_name).expect(&format!("Unable to open file '{}'", bin_file_name));
    let mut buffer: Vec<u8> = Vec::new();

    // Load code starting at 0x100
    while buffer.len() < 0x100 {
        buffer.push(0);
    }
    let _ = file.read_to_end(&mut buffer);
    // No idea how much memory this expects, hope this is enough
    while buffer.len() < 0x10000 {
        buffer.push(0);
    }

    let mut state = State8080::empty_state();
    state.memory = buffer;

    // Instructions start at 0x100
    state.set_program_counter(0x100);

    // RET to skip weird call into (seemingly) uninitialized memory?
    state.memory[0x5] = 0xc9;

    // fix stack pointer
    state.memory[368] = 0x7;

    // Skip DAA test
    state.memory[0x59c] = 0xc3; // JMP
    state.memory[0x59d] = 0xc2;
    state.memory[0x59e] = 0x05;

    emulator::run(&mut state);
}