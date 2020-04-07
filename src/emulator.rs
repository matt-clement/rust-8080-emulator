use crate::state_8080::State8080;
use crate::disassembler;

// Returns 1 for even parity, 0 for odd
fn parity(x: u8) -> u8 {
    let mut p: u8 = x ^ x.checked_shr(1).unwrap_or(0);
    p ^= p.checked_shr(2).unwrap_or(0);
    p ^= p.checked_shr(4).unwrap_or(0);
    p ^= p.checked_shr(8).unwrap_or(0);
    if (p & 0x01) == 1 { 0 } else { 1 }
}

fn unimplemented_instruction(_state: &State8080) -> ! {
    eprintln!("Error: Unimplimented instruction");
    std::process::exit(1);
}

#[allow(dead_code)]
pub fn run(state: &mut State8080) {
    loop {
        emulate_8080_op(state);
    }
}

const CYCLES: [u32; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4,
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4,
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4,
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4,

    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,

    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,

    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11,
    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11,
    11, 10, 10, 18, 17, 11, 7, 11, 11, 5, 10, 5, 17, 17, 7, 11,
    11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11,
];

pub fn emulate_8080_op(state: &mut State8080) -> u32 {
    let program_counter: usize = state.program_counter() as usize;
    let opcode: u8 = state.memory[program_counter];
    let (opcode_description, _) = disassembler::disassemble_opcode(&state.memory, program_counter);
    println!("{}\t| {:#02x} | {:x?}", opcode_description, opcode, state);

    state.increment_program_counter(1);

    match opcode {
        0x00 => {},
        0x01 => {
            state.c = state.memory[program_counter + 1];
            state.b = state.memory[program_counter + 2];
            state.increment_program_counter(2);
        },
        0x02 => unimplemented_instruction(state),
        0x03 => {
            let result = (((state.b as u16) << 8) | state.c as u16) + 1;
            state.b = ((result & 0xff00) >> 8) as u8;
            state.c = (result & 0x00ff) as u8;
        },
        0x04 => {
            let answer: u16 = (state.b as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x05 => {
            let answer: u8 = state.b.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.b = answer;
        },
        0x06 => {
            state.b = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x07 => unimplemented_instruction(state),
        0x08 => unimplemented_instruction(state),
        0x09 => {
            let bc: u16 = ((state.b as u16) << 8) | state.c as u16;
            let hl: u16 = ((state.h as u16) << 8) | state.l as u16;
            let result: u32 = hl as u32 + bc as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.h = ((result & 0xff00) >> 8) as u8;
            state.l = (result & 0x00ff) as u8;
        },
        0x0a => {
            let address = ((state.b as u16) << 8) | state.c as u16;
            state.a = state.memory[address as usize];
        },
        0x0b => {
            let result = (((state.b as u16) << 8) | state.c as u16).wrapping_sub(1);
            state.b = ((result & 0xff00) >> 8) as u8;
            state.c = (result & 0x00ff) as u8;
        },
        0x0c => {
            let answer: u16 = (state.c as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x0d => {
            let answer: u8 = state.c.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.c = answer;
        },
        0x0e => {
            state.c = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x0f => unimplemented_instruction(state),
        0x10 => unimplemented_instruction(state),
        0x11 => {
            state.e = state.memory[program_counter + 1];
            state.d = state.memory[program_counter + 2];
            state.increment_program_counter(2);
        },
        0x12 => unimplemented_instruction(state),
        0x13 => {
            let result = (((state.d as u16) << 8) | state.e as u16) + 1;
            state.d = ((result & 0xff00) >> 8) as u8;
            state.e = (result & 0x00ff) as u8;
        }
        0x14 => {
            let answer: u16 = (state.d as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x15 => {
            let answer: u8 = state.d.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.d = answer;
        },
        0x16 => {
            state.d = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x17 => unimplemented_instruction(state),
        0x18 => unimplemented_instruction(state),
        0x19 => {
            let de: u16 = ((state.d as u16) << 8) | state.e as u16;
            let hl: u16 = ((state.h as u16) << 8) | state.l as u16;
            let result: u32 = hl as u32 + de as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.h = ((result & 0xff00) >> 8) as u8;
            state.l = (result & 0x00ff) as u8;
        },
        0x1a => {
            let address = ((state.d as u16) << 8) | state.e as u16;
            state.a = state.memory[address as usize];
        },
        0x1b => {
            let result = (((state.d as u16) << 8) | state.e as u16).wrapping_sub(1);
            state.d = ((result & 0xff00) >> 8) as u8;
            state.e = (result & 0x00ff) as u8;
        },
        0x1c => {
            let answer: u16 = (state.e as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x1d => {
            let answer: u8 = state.e.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.e = answer;
        },
        0x1e => {
            state.e = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x1f => unimplemented_instruction(state),
        0x20 => unimplemented_instruction(state),
        0x21 => {
            state.l = state.memory[program_counter + 1];
            state.h = state.memory[program_counter + 2];
            state.increment_program_counter(2);
        },
        0x22 => unimplemented_instruction(state),
        0x23 => {
            let result = (((state.h as u16) << 8) | state.l as u16) + 1;
            state.h = ((result & 0xff00) >> 8) as u8;
            state.l = (result & 0x00ff) as u8;
        },
        0x24 => {
            let answer: u16 = (state.h as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x25 => {
            let answer: u8 = state.h.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.h = answer;
        },
        0x26 => {
            state.h = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x27 => unimplemented_instruction(state),
        0x28 => unimplemented_instruction(state),
        0x29 => {
            let hl: u16 = ((state.h as u16) << 8) | state.l as u16;
            let result: u32 = hl as u32 + hl as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.h = ((result & 0xff00) >> 8) as u8;
            state.l = (result & 0x00ff) as u8;
        },
        0x2a => unimplemented_instruction(state),
        0x2b => {
            let result = (((state.h as u16) << 8) | state.l as u16).wrapping_sub(1);
            state.h = ((result & 0xff00) >> 8) as u8;
            state.l = (result & 0x00ff) as u8;
        },
        0x2c => {
            let answer: u16 = (state.l as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x2d => {
            let answer: u8 = state.l.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.l = answer;
        },
        0x2e => {
            state.l = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x2f => unimplemented_instruction(state),
        0x30 => unimplemented_instruction(state),
        0x31 => {
            state.sp = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16;
            state.increment_program_counter(2);
        },
        0x32 => {
            let high_address = (state.memory[program_counter + 2] as u16) << 8;
            let low_address = state.memory[program_counter + 1] as u16;
            let address: usize = (high_address | low_address) as usize;
            state.memory[address] = state.a;
            state.increment_program_counter(2);
        },
        0x33 => {
            state.sp += 1;
        },
        0x34 => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let answer: u16 = state.memory[offset as usize] as u16 + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x35 => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let minuend: u8 = state.memory[offset as usize];
            let answer: u8 = minuend.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.memory[offset as usize] = answer;
        },
        0x36 => {
            let offset: usize = ((state.h as usize) << 8 ) | state.l as usize;
            state.memory[offset] = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x37 => unimplemented_instruction(state),
        0x38 => unimplemented_instruction(state),
        0x39 => {
            let hl: u16 = ((state.h as u16) << 8) | state.l as u16;
            let result: u32 = hl as u32 + state.sp as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.h = ((result & 0xff00) >> 8) as u8;
            state.l = (result & 0x00ff) as u8;
        },
        0x3a => {
            let high_address = (state.memory[program_counter + 2] as u16) << 8;
            let low_address = state.memory[program_counter + 1] as u16;
            let address: usize = (high_address | low_address) as usize;
            state.a = state.memory[address];
            state.increment_program_counter(2);
        },
        0x3b => {
            state.sp = state.sp.wrapping_sub(1);
        },
        0x3c => {
            let answer: u16 = (state.a as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x3d => {
            let answer: u8 = state.a.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x3e => {
            state.a = state.memory[program_counter + 1];
            state.increment_program_counter(1);
        },
        0x3f => unimplemented_instruction(state),
        0x40 => state.b = state.b,
        0x41 => state.b = state.c,
        0x42 => state.b = state.d,
        0x43 => state.b = state.e,
        0x44 => state.b = state.h,
        0x45 => state.b = state.l,
        0x46 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.b = state.memory[address as usize];
        },
        0x47 => state.c = state.a,
        0x48 => state.c = state.b,
        0x49 => state.c = state.c,
        0x4a => state.c = state.d,
        0x4b => state.c = state.e,
        0x4c => state.c = state.h,
        0x4d => state.c = state.l,
        0x4e => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.c = state.memory[address as usize];
        },
        0x4f => state.c = state.a,
        0x50 => state.d = state.b,
        0x51 => state.d = state.c,
        0x52 => state.d = state.d,
        0x53 => state.d = state.e,
        0x54 => state.d = state.h,
        0x55 => state.d = state.l,
        0x56 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.d = state.memory[address as usize];
        },
        0x57 => state.d = state.a,
        0x58 => state.e = state.b,
        0x59 => state.e = state.c,
        0x5a => state.e = state.d,
        0x5b => state.e = state.e,
        0x5c => state.e = state.h,
        0x5d => state.e = state.l,
        0x5e => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.e = state.memory[address as usize];
        },
        0x5f => state.e = state.a,
        0x60 => state.h = state.b,
        0x61 => state.h = state.c,
        0x62 => state.h = state.d,
        0x63 => state.h = state.e,
        0x64 => state.h = state.h,
        0x65 => state.h = state.l,
        0x66 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.h = state.memory[address as usize];
        },
        0x67 => state.h = state.a,
        0x68 => state.l = state.b,
        0x69 => state.l = state.c,
        0x6a => state.l = state.d,
        0x6b => state.l = state.e,
        0x6c => state.l = state.h,
        0x6d => state.l = state.l,
        0x6e => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.l = state.memory[address as usize];
        },
        0x6f => state.l = state.a,
        0x70 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.memory[address as usize] = state.b;
        }
        0x71 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.memory[address as usize] = state.c;
        },
        0x72 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.memory[address as usize] = state.d;
        },
        0x73 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.memory[address as usize] = state.e;
        },
        0x74 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.memory[address as usize] = state.h;
        },
        0x75 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.memory[address as usize] = state.l;
        },
        0x76 => {
            std::process::exit(0);
        },
        0x77 => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.memory[address as usize] = state.a;
        },
        0x78 => state.a = state.b,
        0x79 => state.a = state.c,
        0x7a => state.a = state.d,
        0x7b => state.a = state.e,
        0x7c => state.a = state.h,
        0x7d => state.a = state.l,
        0x7e => {
            let address: u16 = ((state.h as u16) << 8) | state.l as u16;
            state.a = state.memory[address as usize];
        },
        0x7f => state.a = state.a,
        0x80 => {
            let answer: u16 = state.a as u16 + state.b as u16;
            let masked_answer: u8 = (answer & 0xff) as u8;

            // Zero flag: if the result is zero set the flag, else clear it
            if masked_answer == 0 {
                state.cc.z = 1;
            } else {
                state.cc.z = 0;
            }

            // Sign flag: if bit 7 is 1 set the flag, else clear it
            if (answer & 0x80) == 0x80 {
                state.cc.s = 1;
            } else {
                state.cc.s = 0;
            }

            // Carry flag
            if answer > 0xff {
                state.cc.cy = 1;
            } else {
                state.cc.cy = 0;
            }

            state.cc.p = parity(masked_answer);

            state.a = masked_answer;
        },
        0x81 => {
            let answer: u16 = (state.a as u16) + (state.c as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x82 => {
            let answer: u16 = (state.a as u16) + (state.d as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x83 => {
            let answer: u16 = (state.a as u16) + (state.e as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x84 => {
            let answer: u16 = (state.a as u16) + (state.h as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x85 => {
            let answer: u16 = (state.a as u16) + (state.l as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x86 => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let answer: u16 = (state.a as u16) + state.memory[offset as usize] as u16;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x87 => {
            let answer: u16 = (state.a as u16) + (state.a as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x88 => {
            let answer: u16 = (state.a as u16) + (state.b as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x89 => {
            let answer: u16 = (state.a as u16) + (state.c as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8a => {
            let answer: u16 = (state.a as u16) + (state.d as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8b => {
            let answer: u16 = (state.a as u16) + (state.e as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8c => {
            let answer: u16 = (state.a as u16) + (state.h as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8d => {
            let answer: u16 = (state.a as u16) + (state.l as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8e => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let answer: u16 = (state.a as u16) + state.memory[offset as usize] as u16 + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8f => {
            let answer: u16 = (state.a as u16) + (state.a as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x90 => {
            let answer: u8 = state.a - state.b;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.b { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x91 => {
            let answer: u8 = state.a - state.c;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.c { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x92 => {
            let answer: u8 = state.a - state.d;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.d { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x93 => {
            let answer: u8 = state.a - state.e;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.e { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x94 => {
            let answer: u8 = state.a - state.h;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.h { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x95 => {
            let answer: u8 = state.a - state.l;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.l { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x96 => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let subtrahend: u8 = state.memory[offset as usize];
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x97 => {
            let answer: u8 = state.a - state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.a { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x98 => {
            let subtrahend: u8 = state.b + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x99 => {
            let subtrahend: u8 = state.c + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9a => {
            let subtrahend: u8 = state.d + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9b => {
            let subtrahend: u8 = state.e + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9c => {
            let subtrahend: u8 = state.h + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9d => {
            let subtrahend: u8 = state.l + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9e => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let subtrahend: u8 = state.memory[offset as usize] + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9f => {
            let subtrahend: u8 = state.a + state.cc.cy;
            let answer: u8 = state.a - subtrahend;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa0 => {
            let answer: u8 = state.a & state.b;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa1 => {
            let answer: u8 = state.a & state.c;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa2 => {
            let answer: u8 = state.a & state.d;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa3 => {
            let answer: u8 = state.a & state.e;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa4 => {
            let answer: u8 = state.a & state.h;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa5 => {
            let answer: u8 = state.a & state.l;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa6 => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let answer: u8 = state.a & state.memory[offset as usize];
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa7 => {
            let answer: u8 = state.a & state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa8 => {
            let answer: u8 = state.a ^ state.b;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa9 => {
            let answer: u8 = state.a ^ state.c;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xaa => {
            let answer: u8 = state.a ^ state.d;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xab => {
            let answer: u8 = state.a ^ state.e;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xac => {
            let answer: u8 = state.a ^ state.h;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xad => {
            let answer: u8 = state.a ^ state.l;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xae => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let answer: u8 = state.a ^ state.memory[offset as usize];
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xaf => {
            let answer: u8 = state.a ^ state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb0 => {
            let answer: u8 = state.a | state.b;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb1 => {
            let answer: u8 = state.a | state.c;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb2 => {
            let answer: u8 = state.a | state.d;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb3 => {
            let answer: u8 = state.a | state.e;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb4 => {
            let answer: u8 = state.a | state.h;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb5 => {
            let answer: u8 = state.a | state.l;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb6 => {
            let offset: u16 = ((state.h as u16) << 8 ) | state.l as u16;
            let answer: u8 = state.a | state.memory[offset as usize];
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb7 => {
            let answer: u8 = state.a | state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb8 => unimplemented_instruction(state),
        0xb9 => unimplemented_instruction(state),
        0xba => unimplemented_instruction(state),
        0xbb => unimplemented_instruction(state),
        0xbc => unimplemented_instruction(state),
        0xbd => unimplemented_instruction(state),
        0xbe => unimplemented_instruction(state),
        0xbf => unimplemented_instruction(state),
        0xc0 => {
            if state.cc.z != 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc1 => {
            state.c = state.memory[state.sp as usize];
            state.b = state.memory[state.sp as usize + 1];
            state.sp += 2;
        },
        0xc2 => {
            if state.cc.z == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc3 => {
            let high_address = (state.memory[program_counter + 2] as u16) << 8;
            let low_address = state.memory[program_counter + 1] as u16;
            state.set_program_counter(high_address | low_address);
        },
        0xc4 => {
            if state.cc.z != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc5 => {
            state.memory[state.sp as usize - 1] = state.b;
            state.memory[state.sp as usize - 2] = state.c;
            state.sp -= 2;
        },
        0xc6 => unimplemented_instruction(state),
        0xc7 => unimplemented_instruction(state),
        0xc8 => {
            if state.cc.z == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc9 => {
            let high_address = state.memory[state.sp as usize] as u16;
            let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
            state.set_program_counter(high_address | low_address);
            state.sp += 2;
        },
        0xca => {
            if state.cc.z != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xcb => unimplemented_instruction(state),
        0xcc => {
            if state.cc.z == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xcd => {
            let ret: u16 = program_counter as u16 + 3;
            state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
            state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
            state.sp = state.sp - 2;
            let high_address = (state.memory[program_counter + 2] as u16) << 8;
            let low_address = state.memory[program_counter + 1] as u16;
            state.set_program_counter(high_address | low_address);
        },
        0xce => unimplemented_instruction(state),
        0xcf => unimplemented_instruction(state),
        0xd0 => {
            if state.cc.cy == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd1 => {
            state.e = state.memory[state.sp as usize];
            state.d = state.memory[state.sp as usize + 1];
            state.sp += 2;
        },
        0xd2 => {
            if state.cc.cy == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd3 => {
            // TODO: IO
            // This is the OUT instruction, for now just skip data byte
            state.increment_program_counter(1);
        },
        0xd4 => {
            if state.cc.cy != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd5 => {
            state.memory[state.sp as usize - 1] = state.d;
            state.memory[state.sp as usize - 2] = state.e;
            state.sp -= 2;
        },
        0xd6 => unimplemented_instruction(state),
        0xd7 => unimplemented_instruction(state),
        0xd8 => {
            if state.cc.cy != 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd9 => unimplemented_instruction(state),
        0xda => {
            if state.cc.cy != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xdb => {
            // TODO: IO
            // This is the IN instruction, for now just skip data byte
            state.increment_program_counter(1);
        },
        0xdc => {
            if state.cc.cy == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xdd => unimplemented_instruction(state),
        0xde => unimplemented_instruction(state),
        0xdf => unimplemented_instruction(state),
        0xe0 => {
            if state.cc.p == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe1 => {
            state.l = state.memory[state.sp as usize];
            state.h = state.memory[state.sp as usize + 1];
            state.sp += 2;
        },
        0xe2 => {
            if state.cc.p == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe3 => {
            let new_l = state.memory[state.sp as usize];
            let new_h = state.memory[state.sp as usize + 1];
            state.memory[state.sp as usize] = state.l;
            state.memory[state.sp as usize + 1] = state.h;
            state.h = new_h;
            state.l = new_l;
        },
        0xe4 => {
            if state.cc.p == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe5 => {
            state.memory[state.sp as usize - 1] = state.h;
            state.memory[state.sp as usize - 2] = state.l;
            state.sp -= 2;
        },
        0xe6 => unimplemented_instruction(state),
        0xe7 => unimplemented_instruction(state),
        0xe8 => {
            if state.cc.p != 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe9 => unimplemented_instruction(state),
        0xea => {
            if state.cc.p != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xeb => unimplemented_instruction(state),
        0xec => {
            if state.cc.p != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xed => unimplemented_instruction(state),
        0xee => unimplemented_instruction(state),
        0xef => unimplemented_instruction(state),
        0xf0 => {
            if state.cc.s == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xf1 => {
            state.a = state.memory[state.sp as usize + 1];
            let psw: u8 = state.memory[state.sp as usize];
            state.cc.cy = if 0x01 == (psw & 0x01) { 1 } else { 0 };
            state.cc.p = if 0x04 == (psw & 0x04) { 1 } else { 0 };
            state.cc.ac = if 0x10 == (psw & 0x10) { 1 } else { 0 };
            state.cc.z = if 0x40 == (psw & 0x40) { 1 } else { 0 };
            state.cc.s = if 0x80 == (psw & 0x80) { 1 } else { 0 };
            state.sp += 2;
        },
        0xf2 => {
            if state.cc.s == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xf3 => {
            state.disable_interrupt();
        },
        0xf4 => {
            if state.cc.s == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.set_program_counter(2);
            }
        },
        0xf5 => {
            state.memory[state.sp as usize -1] = state.a;
            let cc = &state.cc;
            let psw: u8 = cc.cy | cc.p << 2 | cc.ac << 4 | cc.z << 6 | cc.s << 7;
            state.memory[state.sp as usize - 2] = psw;
            state.sp -= 2;
        },
        0xf6 => unimplemented_instruction(state),
        0xf7 => unimplemented_instruction(state),
        0xf8 => {
            if state.cc.s != 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xf9 => {
            state.sp = ((state.h as u16) << 8) | (state.l as u16);
        },
        0xfa => {
            if state.cc.s != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xfb => {
            state.enable_interrupt();
        },
        0xfc => {
            if state.cc.s != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xfd => unimplemented_instruction(state),
        0xfe => {
            /*
            let acc = state.a;
            let immediate_data = state.memory[program_counter + 1];
            let answer: u16 = (acc as u16).wrapping_sub(immediate_data as u16);
            let masked_answer: u8 = answer as u8 & 0xff;
            let same_sign = acc & 0x80 == immediate_data & 0x80;
            let (carry_positive, carry_negative) = if same_sign { (1, 0) } else { (1, 0) };
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { carry_positive } else { carry_negative };
            state.cc.p = parity(masked_answer);
            state.increment_program_counter(1);
            */

            // The following aligns with implementations I've seen
            // I still have to convince myself that it's right.
            let answer: u8 = state.a.wrapping_sub(state.memory[program_counter + 1]);
            let masked_answer: u8 = answer & 0xff;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.cc.cy = if state.a < state.memory[program_counter + 1] {
                1
            } else {
                0
            };
        },
        0xff => unimplemented_instruction(state),
    }
    CYCLES[opcode as usize]
}

mod test {
    #[allow(unused)] use super::*;

    #[test]
    fn dothething() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x87];
        emulate_8080_op(&mut state);
        assert_eq!(state.cc.z, 1);
        assert_eq!(state.cc.p, 1);
    }

    #[test]
    fn test_parity() {
        assert_eq!(parity(0), 1);
        assert_eq!(parity(1), 0);
        assert_eq!(parity(2), 0);
        assert_eq!(parity(3), 1);
        assert_eq!(parity(4), 0);
        assert_eq!(parity(5), 1);
        assert_eq!(parity(8), 0);
        assert_eq!(parity(16), 0);
        assert_eq!(parity(127), 0);
        assert_eq!(parity(128), 0);
        assert_eq!(parity(129), 1);
        assert_eq!(parity(254), 0);
        assert_eq!(parity(255), 1);
    }

    #[test]
    fn test_inx_d_low() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x13];
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0);
        assert_eq!(state.e, 1);
    }

    #[test]
    fn test_inx_d_rollover() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x13];
        state.d = 0x38;
        state.e = 0xff;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0x39);
        assert_eq!(state.e, 0x00);
    }

    #[test]
    fn test_dcr_b_underflow() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x05];
        state.b = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0xff);
    }

    #[test]
    fn test_dcx_d_underflow() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x1b];
        state.d = 0x00;
        state.e = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0xff);
        assert_eq!(state.e, 0xff);
    }

    #[test]
    fn test_dcx_d_low() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x1b];
        state.d = 0x00;
        state.e = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0x00);
        assert_eq!(state.e, 0x00);
    }

    #[test]
    fn test_dcx_d_mid() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x1b];
        state.d = 0x00;
        state.e = 0xff;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0x00);
        assert_eq!(state.e, 0xfe);
    }

    #[test]
    fn test_dcx_d_high() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x1b];
        state.d = 0xff;
        state.e = 0xff;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0xff);
        assert_eq!(state.e, 0xfe);
    }

    #[test]
    fn test_dcx_h() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x2b];
        state.h = 0x98;
        state.l = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.h, 0x97);
        assert_eq!(state.l, 0xff);
    }

    #[test]
    fn test_dad_b() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x09];
        state.b = 0x33;
        state.c = 0x9f;
        state.h = 0xa1;
        state.l = 0x7b;
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0x33);
        assert_eq!(state.c, 0x9f);
        assert_eq!(state.h, 0xd5);
        assert_eq!(state.l, 0x1a);
        assert_eq!(state.cc.cy, 0);
    }

    #[test]
    fn test_dad_b_carry() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x09];
        state.b = 0xff;
        state.c = 0xff;
        state.h = 0x00;
        state.l = 0x02;
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0xff);
        assert_eq!(state.c, 0xff);
        assert_eq!(state.h, 0x00);
        assert_eq!(state.l, 0x01);
        assert_eq!(state.cc.cy, 1);
    }

    #[test]
    fn test_mvi_h() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x26, 0x3c];
        state.h = 0;
        emulate_8080_op(&mut state);
        assert_eq!(state.h, 0x3c);
    }

    #[test]
    fn test_mvi_l() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x2e, 0xf4];
        state.l = 0;
        emulate_8080_op(&mut state);
        assert_eq!(state.l, 0xf4);
    }

    #[test]
    fn test_mvi_m() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x36, 0xff, 0x00];
        state.h = 0x00;
        state.l = 0x02;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[2], 0xff);
    }

    #[test]
    fn test_sequential_mvi() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x06, 0xde, 0x0e, 0xad];
        state.b = 0x00;
        state.c = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0xde);
        assert_eq!(state.c, 0x00);
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0xde);
        assert_eq!(state.c, 0xad);
    }

    #[test]
    fn test_ldax_d() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x1a, 0xde, 0x0e, 0xad];
        state.a = 0x00;
        state.d = 0x00;
        state.e = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xde);
    }

    #[test]
    fn test_mov_m_a() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x77, 0x00];
        state.a = 0xde;
        state.h = 0x00;
        state.l = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x01], 0xde);
    }

    #[test]
    fn test_jnz_no_jump() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xc2, 0x00, 0x00, 0x00, 0x00, 0x00];
        state.cc.z = 1;
        emulate_8080_op(&mut state);
        assert_eq!(state.program_counter(), 0x03);
    }

    #[test]
    fn test_jnz_jump() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xc2, 0x04, 0x00, 0x00, 0x00, 0x00];
        state.cc.z = 0;
        emulate_8080_op(&mut state);
        assert_eq!(state.program_counter(), 0x04);
    }

    #[test]
    fn simple_loop_test() {
        let mut state = State8080::empty_state();
        state.memory = vec![
            0x3e, 0x03, // MVI A 0x03
            0xfa, 0x10, 0x00, // JM 0x10 0x00
            0x3d, // DCR A
            0xc3, 0x02, 0x00, // JMP 0x02 0x00
            0x00, // NOP
        ];
        emulate_8080_op(&mut state); // MVI
        assert_eq!(state.a, 0x03);
        assert_eq!(state.cc.z, 0x00);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.program_counter(), 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0x02);
        assert_eq!(state.cc.z, 0x00);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.program_counter(), 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.program_counter(), 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.z, 0x00);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.program_counter(), 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.program_counter(), 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, 0x01);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.program_counter(), 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.program_counter(), 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0xff);
        assert_eq!(state.cc.z, 0x00);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.program_counter(), 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.program_counter(), 0x10);
    }

    #[test]
    fn test_sta() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x32, 0x03, 0x00, 0x00];
        state.a = 0x09;
        state.set_program_counter(0);
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x0003], 0x09);
        assert_eq!(state.program_counter(), 0x03);
    }

    #[test]
    fn test_lda() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x3a, 0x03, 0x00, 0x09];
        state.a = 0x00;
        state.set_program_counter(0);
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x09);
        assert_eq!(state.program_counter(), 0x03);
    }

    #[test]
    fn test_pop_b() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xc1, 0xc3, 0xff];
        state.sp = 0x01;
        state.b = 0x00;
        state.c = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0xff);
        assert_eq!(state.c, 0xc3);
        assert_eq!(state.sp, 0x03);
    }

    #[test]
    fn test_pop_h() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xe1, 0x3d, 0x93];
        state.sp = 0x01;
        state.h = 0x00;
        state.l = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.h, 0x93);
        assert_eq!(state.l, 0x3d);
        assert_eq!(state.sp, 0x03);
    }

    #[test]
    fn test_pop_psw() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xf1, 0xc3, 0xff];
        state.sp = 0x01;
        state.a = 0x00;
        state.cc.cy = 0x00;
        state.cc.p = 0x00;
        state.cc.ac = 0x00;
        state.cc.z = 0x00;
        state.cc.s = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xff);
        assert_eq!(state.cc.cy, 0x01);
        assert_eq!(state.cc.p, 0x00);
        assert_eq!(state.cc.ac, 0x00);
        assert_eq!(state.cc.z, 0x01);
        assert_eq!(state.cc.s, 0x01);
        assert_eq!(state.sp, 0x03);
    }

    #[test]
    fn test_push_psw() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xf5, 0x00, 0x00];
        state.sp = 0x03;
        state.a = 0x47;
        state.cc.cy = 0x01;
        state.cc.p = 0x01;
        state.cc.ac = 0x00;
        state.cc.z = 0x01;
        state.cc.s = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x01], 0x45);
        assert_eq!(state.memory[0x02], 0x47);
        assert_eq!(state.sp, 0x01);
    }

    #[test]
    fn test_push_pop_psw() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xf5, 0xf1, 0x00, 0x00];
        state.sp = 0x04;
        state.a = 0x47;
        state.cc.cy = 0x01;
        state.cc.p = 0x01;
        state.cc.ac = 0x00;
        state.cc.z = 0x01;
        state.cc.s = 0x00;

        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x02], 0x45);
        assert_eq!(state.memory[0x03], 0x47);
        assert_eq!(state.sp, 0x02);

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x47);
        assert_eq!(state.cc.cy, 0x01);
        assert_eq!(state.cc.p, 0x01);
        assert_eq!(state.cc.ac, 0x00);
        assert_eq!(state.cc.z, 0x01);
        assert_eq!(state.cc.s, 0x00);
        assert_eq!(state.sp, 0x04);
    }

    #[test]
    fn test_push_d() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xd5, 0x00, 0x00];
        state.sp = 0x03;
        state.d = 0x8f;
        state.e = 0x9d;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x02], 0x8f);
        assert_eq!(state.memory[0x01], 0x9d);
        assert_eq!(state.sp, 0x01);
    }

    #[test]
    fn test_sphl() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xf9];
        state.sp = 0x00;
        state.h = 0x50;
        state.l = 0x6c;
        emulate_8080_op(&mut state);
        assert_eq!(state.sp, 0x506c);
    }

    #[test]
    fn test_xthl() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xe3, 0xf0, 0x0d];
        state.sp = 0x01;
        state.h = 0x0b;
        state.l = 0x3c;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x01], 0x3c);
        assert_eq!(state.memory[0x02], 0x0b);
        assert_eq!(state.h, 0x0d);
        assert_eq!(state.l, 0xf0);
        assert_eq!(state.sp, 0x01);
    }

    #[test]
    fn test_add_d() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x82];
        state.a = 0x6c;
        state.d = 0x2e;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x9a);
        assert_eq!(state.cc.z, 0);
        assert_eq!(state.cc.cy, 0);
        assert_eq!(state.cc.p, 1);
        assert_eq!(state.cc.s, 1);
        // TODO: assert_eq!(state.cc.ac, 1);
    }

    #[test]
    fn test_add_a() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x87];
        state.a = 0x04;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x08);
    }

    #[test]
    fn test_subroutine() {
        let mut state = State8080::empty_state();
        state.memory = vec![
            0x31, 0x00, 0x10, // Setup stack pointer
            0xcd, 0x10, 0x00, // Call
            0x06, 0x12, // Set register b
            0x00, 0x00, // Padding
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x0e, 0x24, // Set register c
            0xc9, // Return
        ];
        while state.memory.len() < 4096 {
            state.memory.push(0);
        }
        assert_eq!(state.b, 0x00);
        emulate_8080_op(&mut state);
        assert_eq!(state.sp, 0x1000);
        emulate_8080_op(&mut state);
        assert_eq!(state.sp, 0x0ffe);
        assert_eq!(state.program_counter(), 0x10);
        assert_eq!(state.c, 0x00);
        emulate_8080_op(&mut state);
        assert_eq!(state.c, 0x24);
        emulate_8080_op(&mut state);
        assert_eq!(state.program_counter(), 0x06);
        assert_eq!(state.b, 0x00);
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0x12);
        assert_eq!(state.sp, 0x1000);
    }

    #[test]
    fn test_cpi() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xfe, 0xc0];
        state.a = 0x4a;
        emulate_8080_op(&mut state);
        // TODO: assert_eq!(state.cc.cy, 0);
        assert_eq!(state.cc.z, 0);
    }
}
