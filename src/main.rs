use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
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

impl std::fmt::Debug for State8080 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State8080")
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .field("d", &self.d)
            .field("e", &self.e)
            .field("h", &self.h)
            .field("l", &self.l)
            .field("sp", &self.sp)
            .field("pc", &self.pc)
            .field("cc", &self.cc)
            .finish()
    }
}

fn unimplemented_instruction(_state: &State8080) -> ! {
    eprintln!("Error: Unimplimented instruction");
    std::process::exit(1);
}

fn emulate_8080_op(state: &mut State8080) {
    let program_counter: usize = state.pc as usize;
    let opcode: u8 = state.memory[program_counter];
    let (opcode_description, _) = disassemble_opcode(&state.memory, program_counter);
    println!("{}\t| {:#02x} | {:x?}", opcode_description, opcode, state);

    state.pc += 1;

    match opcode {
        0x00 => {},
        0x01 => {
            state.c = state.memory[program_counter + 1];
            state.b = state.memory[program_counter + 2];
            state.pc += 2;
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
            state.pc += 1;
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
            state.pc += 1;
        },
        0x0f => unimplemented_instruction(state),
        0x10 => unimplemented_instruction(state),
        0x11 => {
            state.d = state.memory[program_counter + 1];
            state.e = state.memory[program_counter + 2];
            state.pc += 2;
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
            state.pc += 1;
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
            state.pc += 1;
        },
        0x1f => unimplemented_instruction(state),
        0x20 => unimplemented_instruction(state),
        0x21 => {
            state.h = state.memory[program_counter + 1];
            state.l = state.memory[program_counter + 2];
            state.pc += 2;
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
            state.pc += 1;
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
            state.pc += 1;
        },
        0x2f => unimplemented_instruction(state),
        0x30 => unimplemented_instruction(state),
        0x31 => {
            state.sp = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16;
            state.pc += 2;
        },
        0x32 => {
            let high_address = (state.memory[program_counter + 2] as u16) << 8;
            let low_address = state.memory[program_counter + 1] as u16;
            let address: usize = (high_address | low_address) as usize;
            state.memory[address] = state.a;
            state.pc += 2;
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
            state.pc += 1;
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
        0x3a => unimplemented_instruction(state),
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
            state.pc += 1;
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
        0x76 => unimplemented_instruction(state),
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
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xc1 => unimplemented_instruction(state),
        0xc2 => {
            if state.cc.z == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xc3 => {
            let high_address = (state.memory[program_counter + 2] as u16) << 8;
            let low_address = state.memory[program_counter + 1] as u16;
            state.pc = high_address | low_address;
        },
        0xc4 => {
            if state.cc.z != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xc5 => unimplemented_instruction(state),
        0xc6 => unimplemented_instruction(state),
        0xc7 => unimplemented_instruction(state),
        0xc8 => {
            if state.cc.z == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xc9 => {
            let high_address = state.memory[state.sp as usize] as u16;
            let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
            state.pc = high_address | low_address;
            state.sp += 2;
        },
        0xca => {
            if state.cc.z != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xcb => unimplemented_instruction(state),
        0xcc => {
            if state.cc.z == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xcd => {
            let ret: u16 = program_counter as u16 + 2;
            state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
            state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
            state.sp = state.sp - 2;
            state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
        },
        0xce => unimplemented_instruction(state),
        0xcf => unimplemented_instruction(state),
        0xd0 => {
            if state.cc.cy == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xd1 => unimplemented_instruction(state),
        0xd2 => {
            if state.cc.cy == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xd3 => unimplemented_instruction(state),
        0xd4 => {
            if state.cc.cy != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xd5 => unimplemented_instruction(state),
        0xd6 => unimplemented_instruction(state),
        0xd7 => unimplemented_instruction(state),
        0xd8 => {
            if state.cc.cy != 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xd9 => unimplemented_instruction(state),
        0xda => {
            if state.cc.cy != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xdb => unimplemented_instruction(state),
        0xdc => {
            if state.cc.cy == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xdd => unimplemented_instruction(state),
        0xde => unimplemented_instruction(state),
        0xdf => unimplemented_instruction(state),
        0xe0 => {
            if state.cc.p == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xe1 => unimplemented_instruction(state),
        0xe2 => {
            if state.cc.p == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xe3 => unimplemented_instruction(state),
        0xe4 => {
            if state.cc.p == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xe5 => unimplemented_instruction(state),
        0xe6 => unimplemented_instruction(state),
        0xe7 => unimplemented_instruction(state),
        0xe8 => {
            if state.cc.p != 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xe9 => unimplemented_instruction(state),
        0xea => {
            if state.cc.p != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xeb => unimplemented_instruction(state),
        0xec => {
            if state.cc.p != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xed => unimplemented_instruction(state),
        0xee => unimplemented_instruction(state),
        0xef => unimplemented_instruction(state),
        0xf0 => {
            if state.cc.s == 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xf1 => unimplemented_instruction(state),
        0xf2 => {
            if state.cc.s == 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xf3 => unimplemented_instruction(state),
        0xf4 => {
            if state.cc.s == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xf5 => unimplemented_instruction(state),
        0xf6 => unimplemented_instruction(state),
        0xf7 => unimplemented_instruction(state),
        0xf8 => {
            if state.cc.s != 0 {
                let high_address = state.memory[state.sp as usize] as u16;
                let low_address = (state.memory[state.sp as usize + 1] as u16) << 8;
                state.pc = high_address | low_address;
                state.sp += 2;
            } else {
                state.pc += 2;
            }
        },
        0xf9 => unimplemented_instruction(state),
        0xfa => {
            if state.cc.s != 0 {
                let high_address = (state.memory[program_counter + 2] as u16) << 8;
                let low_address = state.memory[program_counter + 1] as u16;
                state.pc = high_address | low_address;
            } else {
                state.pc += 2;
            }
        },
        0xfb => unimplemented_instruction(state),
        0xfc => {
            if state.cc.s != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = ((state.memory[program_counter + 2] as u16) << 8) | state.memory[program_counter + 1] as u16
            } else {
                state.pc += 2;
            }
        },
        0xfd => unimplemented_instruction(state),
        0xfe => unimplemented_instruction(state),
        0xff => unimplemented_instruction(state),
    }
}

// Returns 1 for even parity, 0 for odd
fn parity(x: u8) -> u8 {
    let mut p: u8 = x ^ x.checked_shr(1).unwrap_or(0);
    p ^= p.checked_shr(2).unwrap_or(0);
    p ^= p.checked_shr(4).unwrap_or(0);
    p ^= p.checked_shr(8).unwrap_or(0);
    if (p & 0x01) == 1 { 0 } else { 1 }
}

fn empty_state() -> State8080 {
    State8080 {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        cc: ConditionCodes { ac: 0, cy: 0, p: 0, pad: 0, s: 0, z: 0 },
        int_enable: 0,
        memory: Vec::new(),
        sp: 0,
        pc: 0,
    }
}


fn main() {
    let file_name = std::env::args().nth(1).expect("Pass file name as first argument");
    let mut file = File::open(&file_name).expect(&format!("Unable to open file '{}'", file_name));
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    while buffer.len() < 0x10000 {
        buffer.push(0);
    }
    let mut state = empty_state();
    state.memory = buffer;
    loop {
        emulate_8080_op(&mut state);
    }
}

// 8080 disassembler
fn disassemble_opcode(src: &[u8], pc: usize) -> (String, usize) {
    let mut opbytes = 1;
    let code = src[pc];
    let opcode_description: String = match code {
        0x00 => format!("NOP"),
        0x01 => { opbytes = 3; format!("LXI\tB,#${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x02 => format!("STAX\tB"),
        0x03 => format!("INX\tB"),
        0x04 => format!("INR\tB"),
        0x05 => format!("DCR\tB"),
        0x06 => { opbytes = 2; format!("MVI\tB,#${:02x}", src[pc + 1]) },
        0x07 => format!("RLC"),
        0x08 => format!("NOP\tB"),
        0x09 => format!("DAD\tB"),
        0x0a => format!("LDAX\tB"),
        0x0b => format!("DCX\tB"),
        0x0c => format!("INR\tC"),
        0x0d => format!("DCR\tC"),
        0x0e => { opbytes = 2; format!("MVI\tC,#${:02x}", src[pc + 1]) },
        0x0f => format!("RRC"),
        0x10 => format!("NOP"),
        0x11 => { opbytes = 3; format!("LXI\tD,#${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x12 => format!("STAX\tD"),
        0x13 => format!("INX\tD"),
        0x14 => format!("INR\tD"),
        0x15 => format!("DCR\tD"),
        0x16 => { opbytes = 2; format!("MVI\tD,#${:02x}", src[pc + 1]) },
        0x17 => format!("RAL"),
        0x18 => format!("NOP"),
        0x19 => format!("DAD\tD"),
        0x1a => format!("LDAX\tD"),
        0x1b => format!("DCX\tD"),
        0x1c => format!("INR\tE"),
        0x1d => format!("DCR\tE"),
        0x1e => { opbytes = 2; format!("MVI\tE,#${:02x}", src[pc + 1]) },
        0x1f => format!("RAR"),
        0x20 => format!("NOP"),
        0x21 => { opbytes = 3; format!("LXI\tH,#${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x22 => { opbytes = 3; format!("SHLD\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x23 => format!("INX\tH"),
        0x24 => format!("INR\tH"),
        0x25 => format!("DCR\tH"),
        0x26 => { opbytes = 2; format!("MVI\tH,#${:02x}", src[pc + 1]) },
        0x27 => format!("DAA"),
        0x28 => format!("NOP"),
        0x29 => format!("DAD\tH"),
        0x2a => { opbytes = 3; format!("LHLD\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x2b => format!("DCX\tH"),
        0x2c => format!("INR\tL"),
        0x2d => format!("DCR\tL"),
        0x2e => { opbytes = 2; format!("MVI\tL,#${:02x}", src[pc + 1]) },
        0x2f => format!("CMA"),
        0x30 => format!("NOP"),
        0x31 => { opbytes = 3; format!("LXI\tSP,#${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x32 => { opbytes = 3; format!("STA\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x33 => format!("INX\tSP"),
        0x34 => format!("INR\tM"),
        0x35 => format!("DCR\tM"),
        0x36 => { opbytes = 2; format!("MVI\tM,#${:02x}", src[pc + 1]) },
        0x37 => format!("STC"),
        0x38 => format!("NOP"),
        0x39 => format!("DAD\tSP"),
        0x3a => { opbytes = 3; format!("LDA\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0x3b => format!("DCX\tSP"),
        0x3c => format!("INR\tA"),
        0x3d => format!("DCR\tA"),
        0x3e => { opbytes = 2; format!("MVI\tA,#${:02x}", src[pc + 1]) },
        0x3f => format!("CMC"),
        0x40 => format!("MOV\tB,B"),
        0x41 => format!("MOV\tB,C"),
        0x42 => format!("MOV\tB,D"),
        0x43 => format!("MOV\tB,E"),
        0x44 => format!("MOV\tB,H"),
        0x45 => format!("MOV\tB,L"),
        0x46 => format!("MOV\tB,M"),
        0x47 => format!("MOV\tB,A"),
        0x48 => format!("MOV\tC,B"),
        0x49 => format!("MOV\tC,C"),
        0x4a => format!("MOV\tC,D"),
        0x4b => format!("MOV\tC,E"),
        0x4c => format!("MOV\tC,H"),
        0x4d => format!("MOV\tC,L"),
        0x4e => format!("MOV\tC,M"),
        0x4f => format!("MOV\tC,A"),
        0x50 => format!("MOV\tD,B"),
        0x51 => format!("MOV\tD,C"),
        0x52 => format!("MOV\tD,D"),
        0x53 => format!("MOV\tD,E"),
        0x54 => format!("MOV\tD,H"),
        0x55 => format!("MOV\tD,L"),
        0x56 => format!("MOV\tD,M"),
        0x57 => format!("MOV\tD,A"),
        0x58 => format!("MOV\tE,B"),
        0x59 => format!("MOV\tE,C"),
        0x5a => format!("MOV\tE,D"),
        0x5b => format!("MOV\tE,E"),
        0x5c => format!("MOV\tE,H"),
        0x5d => format!("MOV\tE,L"),
        0x5e => format!("MOV\tE,M"),
        0x5f => format!("MOV\tE,A"),
        0x60 => format!("MOV\tH,B"),
        0x61 => format!("MOV\tH,C"),
        0x62 => format!("MOV\tH,D"),
        0x63 => format!("MOV\tH,E"),
        0x64 => format!("MOV\tH,H"),
        0x65 => format!("MOV\tH,L"),
        0x66 => format!("MOV\tH,M"),
        0x67 => format!("MOV\tH,A"),
        0x68 => format!("MOV\tL,B"),
        0x69 => format!("MOV\tL,C"),
        0x6a => format!("MOV\tL,D"),
        0x6b => format!("MOV\tL,E"),
        0x6c => format!("MOV\tL,H"),
        0x6d => format!("MOV\tL,L"),
        0x6e => format!("MOV\tL,M"),
        0x6f => format!("MOV\tL,A"),
        0x70 => format!("MOV\tM,B"),
        0x71 => format!("MOV\tM,C"),
        0x72 => format!("MOV\tM,D"),
        0x73 => format!("MOV\tM,E"),
        0x74 => format!("MOV\tM,H"),
        0x75 => format!("MOV\tM,L"),
        0x76 => format!("HLT"),
        0x77 => format!("MOV\tM,A"),
        0x78 => format!("MOV\tA,B"),
        0x79 => format!("MOV\tA,C"),
        0x7a => format!("MOV\tA,D"),
        0x7b => format!("MOV\tA,E"),
        0x7c => format!("MOV\tA,H"),
        0x7d => format!("MOV\tA,L"),
        0x7e => format!("MOV\tA,M"),
        0x7f => format!("MOV\tA,A"),
        0x80 => format!("ADD\tB"),
        0x81 => format!("ADD\tC"),
        0x82 => format!("ADD\tD"),
        0x83 => format!("ADD\tE"),
        0x84 => format!("ADD\tH"),
        0x85 => format!("ADD\tL"),
        0x86 => format!("ADD\tM"),
        0x87 => format!("ADD\tA"),
        0x88 => format!("ADC\tB"),
        0x89 => format!("ADC\tC"),
        0x8a => format!("ADC\tD"),
        0x8b => format!("ADC\tE"),
        0x8c => format!("ADC\tH"),
        0x8d => format!("ADC\tL"),
        0x8e => format!("ADC\tM"),
        0x8f => format!("ADC\tA"),
        0x90 => format!("SUB\tB"),
        0x91 => format!("SUB\tC"),
        0x92 => format!("SUB\tD"),
        0x93 => format!("SUB\tE"),
        0x94 => format!("SUB\tH"),
        0x95 => format!("SUB\tL"),
        0x96 => format!("SUB\tM"),
        0x97 => format!("SUB\tA"),
        0x98 => format!("SBB\tB"),
        0x99 => format!("SBB\tC"),
        0x9a => format!("SBB\tD"),
        0x9b => format!("SBB\tE"),
        0x9c => format!("SBB\tH"),
        0x9d => format!("SBB\tL"),
        0x9e => format!("SBB\tM"),
        0x9f => format!("SBB\tA"),
        0xa0 => format!("ANA\tB"),
        0xa1 => format!("ANA\tC"),
        0xa2 => format!("ANA\tD"),
        0xa3 => format!("ANA\tE"),
        0xa4 => format!("ANA\tH"),
        0xa5 => format!("ANA\tL"),
        0xa6 => format!("ANA\tM"),
        0xa7 => format!("ANA\tA"),
        0xa8 => format!("XRA\tB"),
        0xa9 => format!("XRA\tC"),
        0xaa => format!("XRA\tD"),
        0xab => format!("XRA\tE"),
        0xac => format!("XRA\tH"),
        0xad => format!("XRA\tL"),
        0xae => format!("XRA\tM"),
        0xaf => format!("XRA\tA"),
        0xb0 => format!("ORA\tB"),
        0xb1 => format!("ORA\tC"),
        0xb2 => format!("ORA\tD"),
        0xb3 => format!("ORA\tE"),
        0xb4 => format!("ORA\tH"),
        0xb5 => format!("ORA\tL"),
        0xb6 => format!("ORA\tM"),
        0xb7 => format!("ORA\tA"),
        0xb8 => format!("CMP\tB"),
        0xb9 => format!("CMP\tC"),
        0xba => format!("CMP\tD"),
        0xbb => format!("CMP\tE"),
        0xbc => format!("CMP\tH"),
        0xbd => format!("CMP\tL"),
        0xbe => format!("CMP\tM"),
        0xbf => format!("CMP\tA"),
        0xc0 => format!("RNZ"),
        0xc1 => format!("POP\tB"),
        0xc2 => { opbytes = 3; format!("JNZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xc3 => { opbytes = 3; format!("JMP\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xc4 => { opbytes = 3; format!("CNZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xc5 => format!("PUSH\tB"),
        0xc6 => { opbytes = 2; format!("ADI\t#${:02x}", src[pc + 1]) },
        0xc7 => format!("RST 0"),
        0xc8 => format!("RZ"),
        0xc9 => format!("RET"),
        0xca => { opbytes = 3; format!("JZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xcb => format!("NOP"),
        0xcc => { opbytes = 3; format!("CZ\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xcd => { opbytes = 3; format!("CALL\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xce => { opbytes = 2; format!("ACI\t#${:02x}", src[pc + 1]) },
        0xcf => format!("RST\t1"),
        0xd0 => format!("RNC"),
        0xd1 => format!("POP\tD"),
        0xd2 => { opbytes = 3; format!("JNC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xd3 => { opbytes = 2; format!("OUT\t#${:02x}", src[pc + 1]) },
        0xd4 => { opbytes = 3; format!("CNC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xd5 => format!("PUSH\tD"),
        0xd6 => { opbytes = 2; format!("SUI\t#${:02x}", src[pc + 1]) },
        0xd7 => format!("RST\t2"),
        0xd8 => format!("RC"),
        0xd9 => format!("NOP"),
        0xda => { opbytes = 3; format!("JC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xdb => { opbytes = 2; format!("IN\t#${:02x}", src[pc + 1]) },
        0xdc => { opbytes = 3; format!("CC\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xdd => format!("NOP"),
        0xde => { opbytes = 2; format!("SBI\t#${:02x}", src[pc + 1]) },
        0xdf => format!("RST\t3"),
        0xe0 => format!("RPO"),
        0xe1 => format!("POP\tH"),
        0xe2 => { opbytes = 3; format!("JPO\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xe3 => format!("XHTL"),
        0xe4 => { opbytes = 3; format!("CPO\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xe5 => format!("PUSH\tH"),
        0xe6 => { opbytes = 2; format!("ANI\t#${:02x}", src[pc + 1]) },
        0xe7 => format!("RST\t4"),
        0xe8 => format!("RPE"),
        0xe9 => format!("PCHL"),
        0xea => { opbytes = 3; format!("JPE\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xeb => format!("XCHG"),
        0xec => { opbytes = 3; format!("CPE\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xed => format!("NOP"),
        0xee => { opbytes = 2; format!("XRI\t#${:02x}", src[pc + 1]) },
        0xef => format!("RST\t5"),
        0xf0 => format!("RP"),
        0xf1 => format!("POP\tPSW"),
        0xf2 => { opbytes = 3; format!("JP\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xf3 => format!("DI"),
        0xf4 => { opbytes = 3; format!("CP\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xf5 => format!("PUSH\tPSW"),
        0xf6 => { opbytes = 2; format!("ORI\t#${:02x}", src[pc + 1]) },
        0xf7 => format!("RST\t6"),
        0xf8 => format!("RM"),
        0xf9 => format!("SPHL"),
        0xfa => { opbytes = 3; format!("JM\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xfb => format!("EI"),
        0xfc => { opbytes = 3; format!("CM\t${:02x}{:02x}", src[pc + 2], src[pc + 1]) },
        0xfd => format!("NOP"),
        0xfe => { opbytes = 2; format!("CPI\t#${:02x}", src[pc + 1]) },
        0xff => format!("RST\t7"),
    };
    (format!("{:04x}\t{}", pc, opcode_description), opbytes)
}

mod test {
    #[allow(unused)] use super::*;

    #[test]
    fn dothething() {
        let mut state = empty_state();
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
        let mut state = empty_state();
        state.memory = vec![0x13];
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0);
        assert_eq!(state.e, 1);
    }

    #[test]
    fn test_inx_d_rollover() {
        let mut state = empty_state();
        state.memory = vec![0x13];
        state.d = 0x38;
        state.e = 0xff;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0x39);
        assert_eq!(state.e, 0x00);
    }

    #[test]
    fn test_dcr_b_underflow() {
        let mut state = empty_state();
        state.memory = vec![0x05];
        state.b = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.b, 0xff);
    }

    #[test]
    fn test_dcx_d_underflow() {
        let mut state = empty_state();
        state.memory = vec![0x1b];
        state.d = 0x00;
        state.e = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0xff);
        assert_eq!(state.e, 0xff);
    }

    #[test]
    fn test_dcx_d_low() {
        let mut state = empty_state();
        state.memory = vec![0x1b];
        state.d = 0x00;
        state.e = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0x00);
        assert_eq!(state.e, 0x00);
    }

    #[test]
    fn test_dcx_d_mid() {
        let mut state = empty_state();
        state.memory = vec![0x1b];
        state.d = 0x00;
        state.e = 0xff;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0x00);
        assert_eq!(state.e, 0xfe);
    }

    #[test]
    fn test_dcx_d_high() {
        let mut state = empty_state();
        state.memory = vec![0x1b];
        state.d = 0xff;
        state.e = 0xff;
        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0xff);
        assert_eq!(state.e, 0xfe);
    }

    #[test]
    fn test_dcx_h() {
        let mut state = empty_state();
        state.memory = vec![0x2b];
        state.h = 0x98;
        state.l = 0x00;
        emulate_8080_op(&mut state);
        assert_eq!(state.h, 0x97);
        assert_eq!(state.l, 0xff);
    }

    #[test]
    fn test_dad_b() {
        let mut state = empty_state();
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
        let mut state = empty_state();
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
        let mut state = empty_state();
        state.memory = vec![0x26, 0x3c];
        state.h = 0;
        emulate_8080_op(&mut state);
        assert_eq!(state.h, 0x3c);
    }

    #[test]
    fn test_mvi_l() {
        let mut state = empty_state();
        state.memory = vec![0x2e, 0xf4];
        state.l = 0;
        emulate_8080_op(&mut state);
        assert_eq!(state.l, 0xf4);
    }

    #[test]
    fn test_mvi_m() {
        let mut state = empty_state();
        state.memory = vec![0x36, 0xff, 0x00];
        state.h = 0x00;
        state.l = 0x02;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[2], 0xff);
    }

    #[test]
    fn test_sequential_mvi() {
        let mut state = empty_state();
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
        let mut state = empty_state();
        state.memory = vec![0x1a, 0xde, 0x0e, 0xad];
        state.a = 0x00;
        state.d = 0x00;
        state.e = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xde);
    }

    #[test]
    fn test_mov_m_a() {
        let mut state = empty_state();
        state.memory = vec![0x77, 0x00];
        state.a = 0xde;
        state.h = 0x00;
        state.l = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x01], 0xde);
    }

    #[test]
    fn test_jnz_no_jump() {
        let mut state = empty_state();
        state.memory = vec![0xc2, 0x00, 0x00, 0x00, 0x00, 0x00];
        state.cc.z = 1;
        emulate_8080_op(&mut state);
        assert_eq!(state.pc, 0x03);
    }

    #[test]
    fn test_jnz_jump() {
        let mut state = empty_state();
        state.memory = vec![0xc2, 0x04, 0x00, 0x00, 0x00, 0x00];
        state.cc.z = 0;
        emulate_8080_op(&mut state);
        assert_eq!(state.pc, 0x04);
    }

    #[test]
    fn simple_loop_test() {
        let mut state = empty_state();
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
        assert_eq!(state.pc, 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0x02);
        assert_eq!(state.cc.z, 0x00);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.pc, 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.pc, 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.z, 0x00);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.pc, 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.pc, 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, 0x01);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.pc, 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.pc, 0x05);
        emulate_8080_op(&mut state); // DCR A
        assert_eq!(state.a, 0xff);
        assert_eq!(state.cc.z, 0x00);
        emulate_8080_op(&mut state); // JMP
        assert_eq!(state.pc, 0x02);
        emulate_8080_op(&mut state); // JM
        assert_eq!(state.pc, 0x10);
    }

    #[test]
    fn test_sta() {
        let mut state = empty_state();
        state.memory = vec![0x32, 0x03, 0x00, 0x00];
        state.a = 0x09;
        state.pc = 0;
        emulate_8080_op(&mut state);
        assert_eq!(state.memory[0x0003], 0x09);
        assert_eq!(state.pc, 0x03);
    }
}
