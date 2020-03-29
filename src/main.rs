use std::io::prelude::*;
use std::fs::File;

struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
}

struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: Vec<u8>,
    cc: ConditionCodes,
    int_enable: u8,
}

fn unimplemented_instruction(state: &State8080) -> ! {
    eprintln!("Error: Unimplimented instruction");
    std::process::exit(1);
}

fn emulate_8080_op(state: &mut State8080) {
    let opcode: u8 = state.memory[state.pc as usize];
    match opcode {
        0x00 => {},
        0x01 => {
            state.c = state.memory[state.pc as usize + 1];
            state.b = state.memory[state.pc as usize + 2];
            state.pc += 2;
        },
        0x02 => unimplemented_instruction(state),
        0x03 => unimplemented_instruction(state),
        0x04 => unimplemented_instruction(state),
        0x05 => unimplemented_instruction(state),
        0x06 => unimplemented_instruction(state),
        0x07 => unimplemented_instruction(state),
        0x08 => unimplemented_instruction(state),
        0x09 => unimplemented_instruction(state),
        0x0a => unimplemented_instruction(state),
        0x0b => unimplemented_instruction(state),
        0x0c => unimplemented_instruction(state),
        0x0d => unimplemented_instruction(state),
        0x0e => unimplemented_instruction(state),
        0x0f => unimplemented_instruction(state),
        0x10 => unimplemented_instruction(state),
        0x11 => unimplemented_instruction(state),
        0x12 => unimplemented_instruction(state),
        0x13 => unimplemented_instruction(state),
        0x14 => unimplemented_instruction(state),
        0x15 => unimplemented_instruction(state),
        0x16 => unimplemented_instruction(state),
        0x17 => unimplemented_instruction(state),
        0x18 => unimplemented_instruction(state),
        0x19 => unimplemented_instruction(state),
        0x1a => unimplemented_instruction(state),
        0x1b => unimplemented_instruction(state),
        0x1c => unimplemented_instruction(state),
        0x1d => unimplemented_instruction(state),
        0x1e => unimplemented_instruction(state),
        0x1f => unimplemented_instruction(state),
        0x20 => unimplemented_instruction(state),
        0x21 => unimplemented_instruction(state),
        0x22 => unimplemented_instruction(state),
        0x23 => unimplemented_instruction(state),
        0x24 => unimplemented_instruction(state),
        0x25 => unimplemented_instruction(state),
        0x26 => unimplemented_instruction(state),
        0x27 => unimplemented_instruction(state),
        0x28 => unimplemented_instruction(state),
        0x29 => unimplemented_instruction(state),
        0x2a => unimplemented_instruction(state),
        0x2b => unimplemented_instruction(state),
        0x2c => unimplemented_instruction(state),
        0x2d => unimplemented_instruction(state),
        0x2e => unimplemented_instruction(state),
        0x2f => unimplemented_instruction(state),
        0x30 => unimplemented_instruction(state),
        0x31 => unimplemented_instruction(state),
        0x32 => unimplemented_instruction(state),
        0x33 => unimplemented_instruction(state),
        0x34 => unimplemented_instruction(state),
        0x35 => unimplemented_instruction(state),
        0x36 => unimplemented_instruction(state),
        0x37 => unimplemented_instruction(state),
        0x38 => unimplemented_instruction(state),
        0x39 => unimplemented_instruction(state),
        0x3a => unimplemented_instruction(state),
        0x3b => unimplemented_instruction(state),
        0x3c => unimplemented_instruction(state),
        0x3d => unimplemented_instruction(state),
        0x3e => unimplemented_instruction(state),
        0x3f => unimplemented_instruction(state),
        0x40 => unimplemented_instruction(state),
        0x41 => state.b = state.c,
        0x42 => state.b = state.d,
        0x43 => unimplemented_instruction(state),
        0x44 => unimplemented_instruction(state),
        0x45 => unimplemented_instruction(state),
        0x46 => unimplemented_instruction(state),
        0x47 => unimplemented_instruction(state),
        0x48 => unimplemented_instruction(state),
        0x49 => unimplemented_instruction(state),
        0x4a => unimplemented_instruction(state),
        0x4b => unimplemented_instruction(state),
        0x4c => unimplemented_instruction(state),
        0x4d => unimplemented_instruction(state),
        0x4e => unimplemented_instruction(state),
        0x4f => unimplemented_instruction(state),
        0x50 => unimplemented_instruction(state),
        0x51 => unimplemented_instruction(state),
        0x52 => unimplemented_instruction(state),
        0x53 => unimplemented_instruction(state),
        0x54 => unimplemented_instruction(state),
        0x55 => unimplemented_instruction(state),
        0x56 => unimplemented_instruction(state),
        0x57 => unimplemented_instruction(state),
        0x58 => unimplemented_instruction(state),
        0x59 => unimplemented_instruction(state),
        0x5a => unimplemented_instruction(state),
        0x5b => unimplemented_instruction(state),
        0x5c => unimplemented_instruction(state),
        0x5d => unimplemented_instruction(state),
        0x5e => unimplemented_instruction(state),
        0x5f => unimplemented_instruction(state),
        0x60 => unimplemented_instruction(state),
        0x61 => unimplemented_instruction(state),
        0x62 => unimplemented_instruction(state),
        0x63 => unimplemented_instruction(state),
        0x64 => unimplemented_instruction(state),
        0x65 => unimplemented_instruction(state),
        0x66 => unimplemented_instruction(state),
        0x67 => unimplemented_instruction(state),
        0x68 => unimplemented_instruction(state),
        0x69 => unimplemented_instruction(state),
        0x6a => unimplemented_instruction(state),
        0x6b => unimplemented_instruction(state),
        0x6c => unimplemented_instruction(state),
        0x6d => unimplemented_instruction(state),
        0x6e => unimplemented_instruction(state),
        0x6f => unimplemented_instruction(state),
        0x70 => unimplemented_instruction(state),
        0x71 => unimplemented_instruction(state),
        0x72 => unimplemented_instruction(state),
        0x73 => unimplemented_instruction(state),
        0x74 => unimplemented_instruction(state),
        0x75 => unimplemented_instruction(state),
        0x76 => unimplemented_instruction(state),
        0x77 => unimplemented_instruction(state),
        0x78 => unimplemented_instruction(state),
        0x79 => unimplemented_instruction(state),
        0x7a => unimplemented_instruction(state),
        0x7b => unimplemented_instruction(state),
        0x7c => unimplemented_instruction(state),
        0x7d => unimplemented_instruction(state),
        0x7e => unimplemented_instruction(state),
        0x7f => unimplemented_instruction(state),
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
        0xa0 => unimplemented_instruction(state),
        0xa1 => unimplemented_instruction(state),
        0xa2 => unimplemented_instruction(state),
        0xa3 => unimplemented_instruction(state),
        0xa4 => unimplemented_instruction(state),
        0xa5 => unimplemented_instruction(state),
        0xa6 => unimplemented_instruction(state),
        0xa7 => unimplemented_instruction(state),
        0xa8 => unimplemented_instruction(state),
        0xa9 => unimplemented_instruction(state),
        0xaa => unimplemented_instruction(state),
        0xab => unimplemented_instruction(state),
        0xac => unimplemented_instruction(state),
        0xad => unimplemented_instruction(state),
        0xae => unimplemented_instruction(state),
        0xaf => unimplemented_instruction(state),
        0xb0 => unimplemented_instruction(state),
        0xb1 => unimplemented_instruction(state),
        0xb2 => unimplemented_instruction(state),
        0xb3 => unimplemented_instruction(state),
        0xb4 => unimplemented_instruction(state),
        0xb5 => unimplemented_instruction(state),
        0xb6 => unimplemented_instruction(state),
        0xb7 => unimplemented_instruction(state),
        0xb8 => unimplemented_instruction(state),
        0xb9 => unimplemented_instruction(state),
        0xba => unimplemented_instruction(state),
        0xbb => unimplemented_instruction(state),
        0xbc => unimplemented_instruction(state),
        0xbd => unimplemented_instruction(state),
        0xbe => unimplemented_instruction(state),
        0xbf => unimplemented_instruction(state),
        0xc0 => unimplemented_instruction(state),
        0xc1 => unimplemented_instruction(state),
        0xc2 => unimplemented_instruction(state),
        0xc3 => unimplemented_instruction(state),
        0xc4 => unimplemented_instruction(state),
        0xc5 => unimplemented_instruction(state),
        0xc6 => unimplemented_instruction(state),
        0xc7 => unimplemented_instruction(state),
        0xc8 => unimplemented_instruction(state),
        0xc9 => unimplemented_instruction(state),
        0xca => unimplemented_instruction(state),
        0xcb => unimplemented_instruction(state),
        0xcc => unimplemented_instruction(state),
        0xcd => unimplemented_instruction(state),
        0xce => unimplemented_instruction(state),
        0xcf => unimplemented_instruction(state),
        0xd0 => unimplemented_instruction(state),
        0xd1 => unimplemented_instruction(state),
        0xd2 => unimplemented_instruction(state),
        0xd3 => unimplemented_instruction(state),
        0xd4 => unimplemented_instruction(state),
        0xd5 => unimplemented_instruction(state),
        0xd6 => unimplemented_instruction(state),
        0xd7 => unimplemented_instruction(state),
        0xd8 => unimplemented_instruction(state),
        0xd9 => unimplemented_instruction(state),
        0xda => unimplemented_instruction(state),
        0xdb => unimplemented_instruction(state),
        0xdc => unimplemented_instruction(state),
        0xdd => unimplemented_instruction(state),
        0xde => unimplemented_instruction(state),
        0xdf => unimplemented_instruction(state),
        0xe0 => unimplemented_instruction(state),
        0xe1 => unimplemented_instruction(state),
        0xe2 => unimplemented_instruction(state),
        0xe3 => unimplemented_instruction(state),
        0xe4 => unimplemented_instruction(state),
        0xe5 => unimplemented_instruction(state),
        0xe6 => unimplemented_instruction(state),
        0xe7 => unimplemented_instruction(state),
        0xe8 => unimplemented_instruction(state),
        0xe9 => unimplemented_instruction(state),
        0xea => unimplemented_instruction(state),
        0xeb => unimplemented_instruction(state),
        0xec => unimplemented_instruction(state),
        0xed => unimplemented_instruction(state),
        0xee => unimplemented_instruction(state),
        0xef => unimplemented_instruction(state),
        0xf0 => unimplemented_instruction(state),
        0xf1 => unimplemented_instruction(state),
        0xf2 => unimplemented_instruction(state),
        0xf3 => unimplemented_instruction(state),
        0xf4 => unimplemented_instruction(state),
        0xf5 => unimplemented_instruction(state),
        0xf6 => unimplemented_instruction(state),
        0xf7 => unimplemented_instruction(state),
        0xf8 => unimplemented_instruction(state),
        0xf9 => unimplemented_instruction(state),
        0xfa => unimplemented_instruction(state),
        0xfb => unimplemented_instruction(state),
        0xfc => unimplemented_instruction(state),
        0xfd => unimplemented_instruction(state),
        0xfe => unimplemented_instruction(state),
        0xff => unimplemented_instruction(state),
    }
    state.pc += 1;
}

fn parity(x: u8) -> u8 {
    unimplemented!()
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Pass file name as first argument");
    let mut file = File::open(&file_name).expect(&format!("Unable to open file '{}'", file_name));
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    let mut pc: usize = 0;
    while pc < buffer.len() {
        pc += disassemble_opcode(&buffer, pc);
    }
}

// 8080 disassembler
fn disassemble_opcode(src: &[u8], pc: usize) -> usize {
    let mut opbytes = 1;
    print!("{:04x}\t", pc);
    let code = src[pc];
    match code {
        0x00 => print!("NOP"),
        0x01 => { print!("LXI\tB,#${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3; },
        0x02 => print!("STAX\tB"),
        0x03 => print!("INX\tB"),
        0x04 => print!("INR\tB"),
        0x05 => print!("DCR\tB"),
        0x06 => { print!("MVI\tB,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x07 => print!("RLC"),
        0x08 => print!("NOP\tB"),
        0x09 => print!("DAD\tB"),
        0x0a => print!("LDAX\tB"),
        0x0b => print!("DCX\tB"),
        0x0c => print!("INR\tC"),
        0x0d => print!("DCR\tC"),
        0x0e => { print!("MVI\tC,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x0f => print!("RRC"),
        0x10 => print!("NOP"),
        0x11 => { print!("LXI\tD,#${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3; },
        0x12 => print!("STAX\tD"),
        0x13 => print!("INX\tD"),
        0x14 => print!("INR\tD"),
        0x15 => print!("DCR\tD"),
        0x16 => { print!("MVI\tD,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x17 => print!("RAL"),
        0x18 => print!("NOP"),
        0x19 => print!("DAD\tD"),
        0x1a => print!("LDAX\tD"),
        0x1b => print!("DCX\tD"),
        0x1c => print!("INR\tE"),
        0x1d => print!("DCR\tE"),
        0x1e => { print!("MVI\tE,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x1f => print!("RAR"),
        0x20 => print!("NOP"),
        0x21 => { print!("LXI\tH,#${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3; },
        0x22 => { print!("SHLD\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0x23 => print!("INX\tH"),
        0x24 => print!("INR\tH"),
        0x25 => print!("DCR\tH"),
        0x26 => { print!("MVI\tH,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x27 => print!("DAA"),
        0x28 => print!("NOP"),
        0x29 => print!("DAD\tH"),
        0x2a => { print!("LHLD\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0x2b => print!("DCX\tH"),
        0x2c => print!("INR\tL"),
        0x2d => print!("DCR\tL"),
        0x2e => { print!("MVI\tL,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x2f => print!("CMA"),
        0x30 => print!("NOP"),
        0x31 => { print!("LXI\tSP,#${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3; },
        0x32 => { print!("STA\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0x33 => print!("INX\tSP"),
        0x34 => print!("INR\tM"),
        0x35 => print!("DCR\tM"),
        0x36 => { print!("MVI\tM,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x37 => print!("STC"),
        0x38 => print!("NOP"),
        0x39 => print!("DAD\tSP"),
        0x3a => { print!("LDA\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0x3b => print!("DCX\tSP"),
        0x3c => print!("INR\tA"),
        0x3d => print!("DCR\tA"),
        0x3e => { print!("MVI\tA,#${:02x}", src[pc + 1]); opbytes = 2; },
        0x3f => print!("CMC"),
        0x40 => print!("MOV\tB,B"),
        0x41 => print!("MOV\tB,C"),
        0x42 => print!("MOV\tB,D"),
        0x43 => print!("MOV\tB,E"),
        0x44 => print!("MOV\tB,H"),
        0x45 => print!("MOV\tB,L"),
        0x46 => print!("MOV\tB,M"),
        0x47 => print!("MOV\tB,A"),
        0x48 => print!("MOV\tC,B"),
        0x49 => print!("MOV\tC,C"),
        0x4a => print!("MOV\tC,D"),
        0x4b => print!("MOV\tC,E"),
        0x4c => print!("MOV\tC,H"),
        0x4d => print!("MOV\tC,L"),
        0x4e => print!("MOV\tC,M"),
        0x4f => print!("MOV\tC,A"),
        0x50 => print!("MOV\tD,B"),
        0x51 => print!("MOV\tD,C"),
        0x52 => print!("MOV\tD,D"),
        0x53 => print!("MOV\tD,E"),
        0x54 => print!("MOV\tD,H"),
        0x55 => print!("MOV\tD,L"),
        0x56 => print!("MOV\tD,M"),
        0x57 => print!("MOV\tD,A"),
        0x58 => print!("MOV\tE,B"),
        0x59 => print!("MOV\tE,C"),
        0x5a => print!("MOV\tE,D"),
        0x5b => print!("MOV\tE,E"),
        0x5c => print!("MOV\tE,H"),
        0x5d => print!("MOV\tE,L"),
        0x5e => print!("MOV\tE,M"),
        0x5f => print!("MOV\tE,A"),
        0x60 => print!("MOV\tH,B"),
        0x61 => print!("MOV\tH,C"),
        0x62 => print!("MOV\tH,D"),
        0x63 => print!("MOV\tH,E"),
        0x64 => print!("MOV\tH,H"),
        0x65 => print!("MOV\tH,L"),
        0x66 => print!("MOV\tH,M"),
        0x67 => print!("MOV\tH,A"),
        0x68 => print!("MOV\tL,B"),
        0x69 => print!("MOV\tL,C"),
        0x6a => print!("MOV\tL,D"),
        0x6b => print!("MOV\tL,E"),
        0x6c => print!("MOV\tL,H"),
        0x6d => print!("MOV\tL,L"),
        0x6e => print!("MOV\tL,M"),
        0x6f => print!("MOV\tL,A"),
        0x70 => print!("MOV\tM,B"),
        0x71 => print!("MOV\tM,C"),
        0x72 => print!("MOV\tM,D"),
        0x73 => print!("MOV\tM,E"),
        0x74 => print!("MOV\tM,H"),
        0x75 => print!("MOV\tM,L"),
        0x76 => print!("HLT"),
        0x77 => print!("MOV\tM,A"),
        0x78 => print!("MOV\tA,B"),
        0x79 => print!("MOV\tA,C"),
        0x7a => print!("MOV\tA,D"),
        0x7b => print!("MOV\tA,E"),
        0x7c => print!("MOV\tA,H"),
        0x7d => print!("MOV\tA,L"),
        0x7e => print!("MOV\tA,M"),
        0x7f => print!("MOV\tA,A"),
        0x80 => print!("ADD\tB"),
        0x81 => print!("ADD\tC"),
        0x82 => print!("ADD\tD"),
        0x83 => print!("ADD\tE"),
        0x84 => print!("ADD\tH"),
        0x85 => print!("ADD\tL"),
        0x86 => print!("ADD\tM"),
        0x87 => print!("ADD\tA"),
        0x88 => print!("ADC\tB"),
        0x89 => print!("ADC\tC"),
        0x8a => print!("ADC\tD"),
        0x8b => print!("ADC\tE"),
        0x8c => print!("ADC\tH"),
        0x8d => print!("ADC\tL"),
        0x8e => print!("ADC\tM"),
        0x8f => print!("ADC\tA"),
        0x90 => print!("SUB\tB"),
        0x91 => print!("SUB\tC"),
        0x92 => print!("SUB\tD"),
        0x93 => print!("SUB\tE"),
        0x94 => print!("SUB\tH"),
        0x95 => print!("SUB\tL"),
        0x96 => print!("SUB\tM"),
        0x97 => print!("SUB\tA"),
        0x98 => print!("SBB\tB"),
        0x99 => print!("SBB\tC"),
        0x9a => print!("SBB\tD"),
        0x9b => print!("SBB\tE"),
        0x9c => print!("SBB\tH"),
        0x9d => print!("SBB\tL"),
        0x9e => print!("SBB\tM"),
        0x9f => print!("SBB\tA"),
        0xa0 => print!("ANA\tB"),
        0xa1 => print!("ANA\tC"),
        0xa2 => print!("ANA\tD"),
        0xa3 => print!("ANA\tE"),
        0xa4 => print!("ANA\tH"),
        0xa5 => print!("ANA\tL"),
        0xa6 => print!("ANA\tM"),
        0xa7 => print!("ANA\tA"),
        0xa8 => print!("XRA\tB"),
        0xa9 => print!("XRA\tC"),
        0xaa => print!("XRA\tD"),
        0xab => print!("XRA\tE"),
        0xac => print!("XRA\tH"),
        0xad => print!("XRA\tL"),
        0xae => print!("XRA\tM"),
        0xaf => print!("XRA\tA"),
        0xb0 => print!("ORA\tB"),
        0xb1 => print!("ORA\tC"),
        0xb2 => print!("ORA\tD"),
        0xb3 => print!("ORA\tE"),
        0xb4 => print!("ORA\tH"),
        0xb5 => print!("ORA\tL"),
        0xb6 => print!("ORA\tM"),
        0xb7 => print!("ORA\tA"),
        0xb8 => print!("CMP\tB"),
        0xb9 => print!("CMP\tC"),
        0xba => print!("CMP\tD"),
        0xbb => print!("CMP\tE"),
        0xbc => print!("CMP\tH"),
        0xbd => print!("CMP\tL"),
        0xbe => print!("CMP\tM"),
        0xbf => print!("CMP\tA"),
        0xc0 => print!("RNZ"),
        0xc1 => print!("POP\tB"),
        0xc2 => { print!("JNZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xc3 => { print!("JMP\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xc4 => { print!("CNZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xc5 => print!("PUSH\tB"),
        0xc6 => { print!("ADI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xc7 => print!("RST 0"),
        0xc8 => print!("RZ"),
        0xc9 => print!("RET"),
        0xca => { print!("JZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xcb => print!("NOP"),
        0xcc => { print!("CZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xcd => { print!("CALL\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xce => { print!("ACI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xcf => print!("RST\t1"),
        0xd0 => print!("RNC"),
        0xd1 => print!("POP\tD"),
        0xd2 => { print!("JNC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xd3 => { print!("OUT\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xd4 => { print!("CNC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xd5 => print!("PUSH\tD"),
        0xd6 => { print!("SUI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xd7 => print!("RST\t2"),
        0xd8 => print!("RC"),
        0xd9 => print!("NOP"),
        0xda => { print!("JC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xdb => { print!("IN\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xdc => { print!("CC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xdd => print!("NOP"),
        0xde => { print!("SBI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xdf => print!("RST\t3"),
        0xe0 => print!("RPO"),
        0xe1 => print!("POP\tH"),
        0xe2 => { print!("JPO\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xe3 => print!("XHTL"),
        0xe4 => { print!("CPO\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xe5 => print!("PUSH\tH"),
        0xe6 => { print!("ANI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xe7 => print!("RST\t4"),
        0xe8 => print!("RPE"),
        0xe9 => print!("PCHL"),
        0xea => { print!("JPE\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xeb => print!("XCHG"),
        0xec => { print!("CPE\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xed => print!("NOP"),
        0xee => { print!("XRI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xef => print!("RST\t5"),
        0xf0 => print!("RP"),
        0xf1 => print!("POP\tPSW"),
        0xf2 => { print!("JP\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xf3 => print!("DI"),
        0xf4 => { print!("CP\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xf5 => print!("PUSH\tPSW"),
        0xf6 => { print!("ORI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xf7 => print!("RST\t6"),
        0xf8 => print!("RM"),
        0xf9 => print!("SPHL"),
        0xfa => { print!("JM\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xfb => print!("EI"),
        0xfc => { print!("CM\t${:02x}{:02x}", src[pc + 2], src[pc + 1]); opbytes = 3 },
        0xfd => print!("NOP"),
        0xfe => { print!("CPI\t#${:02x}", src[pc + 1]); opbytes = 2 },
        0xff => print!("RST\t7"),
    };
    println!();
    opbytes
}
