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

fn unimplemented_instruction(state: &State8080) -> ! {
    // Subtracting one from the program counter is a workaround because we
    // increment it at the start of the `emulate_8080_op` function.
    let actual_pc = state.program_counter() as usize - 1;
    let (opcode_description, _) = disassembler::disassemble_opcode(&state.memory, actual_pc);
    let opcode = state.read_memory(actual_pc);
    eprintln!("Error: Unimplimented instruction: {} ({:02x})", opcode_description, opcode);
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
    let opcode: u8 = state.read_memory(program_counter);
    // TODO: How expensive is the following env var fetch and check? Does it
    // need to be moved outside this function?
    /*
    if let Ok(val) = std::env::var("DEBUG_PRINT_INSTRUCTIONS") {
        match val.as_ref() {
            "1" => {
                let (opcode_description, _) = disassembler::disassemble_opcode(&state.memory, program_counter);
                println!("{}\t| {:#02x} | {:x?}", opcode_description, opcode, state);
            },
            _ => {},
        }
    }
    */

    state.increment_program_counter(1);

    match opcode {
        0x00 => {}, // NOP
        0x01 => { // LXI B, D16
            state.c = state.read_memory(program_counter + 1);
            state.b = state.read_memory(program_counter + 2);
            state.increment_program_counter(2);
        },
        0x02 => { // STAX B
            let address = state.bc();
            state.write_memory(address as usize, state.a);
        },
        0x03 => { // INX B
            state.set_bc(state.bc().wrapping_add(1));
        },
        0x04 => { // INR B
            let answer: u16 = (state.b as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.b = masked_answer;
        },
        0x05 => { // DCR B
            let answer: u8 = state.b.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.b = answer;
        },
        0x06 => { // MVI B, D8
            state.b = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x07 => { // RLC
            state.cc.cy = if (state.a & 0x80) == 0x80 { 1 } else { 0 };
            state.a = state.a.rotate_left(1);
        },
        0x08 => {}, // -
        0x09 => { // DAD B
            let result: u32 = state.hl() as u32 + state.bc() as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.set_hl(result as u16);
        },
        0x0a => { // LDAX B
            state.a = state.read_memory(state.bc() as usize);
        },
        0x0b => { // DCX B
            state.set_bc(state.bc().wrapping_sub(1));
        },
        0x0c => { // INR C
            let answer: u16 = (state.c as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.c = masked_answer;
        },
        0x0d => { // DCR C
            let answer: u8 = state.c.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.c = answer;
        },
        0x0e => { // MVI C, D8
            state.c = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x0f => { // RRC
            let low_bit = state.a & 0x01;
            state.a = (state.a >> 1) | (low_bit << 7);
            state.cc.cy = low_bit;
        },
        0x10 => unimplemented_instruction(state), // -
        0x11 => { // LXI D, D16
            state.e = state.read_memory(program_counter + 1);
            state.d = state.read_memory(program_counter + 2);
            state.increment_program_counter(2);
        },
        0x12 => { // STAX D
            let address = state.de();
            state.write_memory(address as usize, state.a);
        },
        0x13 => { // INX D
            state.set_de(state.de().wrapping_add(1));
        }
        0x14 => { // INR D
            let answer: u16 = (state.d as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.d = masked_answer;
        },
        0x15 => { // DCR D
            let answer: u8 = state.d.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.d = answer;
        },
        0x16 => { // MVI D, D8
            state.d = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x17 => { // RAL
            let carry = state.cc.cy;
            let register_a_high_bit = (state.a & 0x80) >> 7;
            state.cc.cy = register_a_high_bit;
            state.a = (state.a << 1) | (carry & 0x01);
        },
        0x18 => {}, // -
        0x19 => { // DAD D
            let result: u32 = state.hl() as u32 + state.de() as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.set_hl(result as u16);
        },
        0x1a => { // LDAX D
            state.a = state.read_memory(state.de() as usize);
        },
        0x1b => { // DCX D
            state.set_de(state.de().wrapping_sub(1));
        },
        0x1c => { // INR E
            let answer: u16 = (state.e as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.e = masked_answer;
        },
        0x1d => { // DCR E
            let answer: u8 = state.e.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.e = answer;
        },
        0x1e => { // MVI E, D8
            state.e = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x1f => { // RAR
            let carry = state.cc.cy;
            let carry_as_high_bit = (carry << 7) & 0x80;
            let register_a_low_bit = state.a & 0x01;
            state.cc.cy = register_a_low_bit;
            state.a = ((state.a & 0x7f) >> 1) | carry_as_high_bit;
        },
        0x20 => {}, // -
        0x21 => { // LXI H, D16
            state.l = state.read_memory(program_counter + 1);
            state.h = state.read_memory(program_counter + 2);
            state.increment_program_counter(2);
        },
        0x22 => { // SHLD adr
            let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
            let low_address = state.read_memory(program_counter + 1) as u16;
            let address: usize = (high_address | low_address) as usize;
            state.write_memory(address, state.l);
            state.write_memory(address + 1, state.h);
            state.increment_program_counter(2);
        },
        0x23 => { // INX H
            state.set_hl(state.hl().wrapping_add(1));
        },
        0x24 => { // INR H
            let answer: u16 = (state.h as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.h = masked_answer;
        },
        0x25 => { // DCR H
            let answer: u8 = state.h.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.h = answer;
        },
        0x26 => { // MVI H, D8
            state.h = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x27 => { // DAA
            let low: u8 = state.a & 0xf;
            if low > 9 || state.cc.ac != 0 {
                let result = low + 0x06;
                state.cc.ac = if result > 0xf { 1 } else { 0 };
                state.a = state.a.wrapping_add(0x06);
            }
            let high: u8 = (state.a & 0xf0) >> 4;
            if high > 9 || state.cc.cy != 0 {
                let result = high + 0x06;
                state.cc.cy = if result > 0xf { 1 } else { 0 };
                state.a = state.a.wrapping_add(0x60);
            }
        },
        0x28 => unimplemented_instruction(state), // -
        0x29 => { // DAD H
            let result: u32 = state.hl() as u32 + state.hl() as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.set_hl(result as u16);
        },
        0x2a => { // LHLD adr
            let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
            let low_address = state.read_memory(program_counter + 1) as u16;
            let address: usize = (high_address | low_address) as usize;
            state.l = state.read_memory(address);
            state.h = state.read_memory(address + 1);
            state.increment_program_counter(2);
        },
        0x2b => { // DCX H
            state.set_hl(state.hl().wrapping_sub(1));
        },
        0x2c => { // INR L
            let answer: u16 = (state.l as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.l = masked_answer;
        },
        0x2d => { // DCR L
            let answer: u8 = state.l.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.l = answer;
        },
        0x2e => { // MVI L, D8
            state.l = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x2f => unimplemented_instruction(state), // CMA
        0x30 => {}, // -
        0x31 => { // LXI SP, D16
            state.sp = ((state.read_memory(program_counter + 2) as u16) << 8) | state.read_memory(program_counter + 1) as u16;
            state.increment_program_counter(2);
        },
        0x32 => { // STA adr
            let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
            let low_address = state.read_memory(program_counter + 1) as u16;
            let address: usize = (high_address | low_address) as usize;
            state.write_memory(address, state.a);
            state.increment_program_counter(2);
        },
        0x33 => { // INX SP
            state.sp += 1;
        },
        0x34 => { // INR M
            let address: usize = state.hl() as usize;
            let answer: u16 = state.read_memory(address) as u16 + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.write_memory(address, masked_answer);
        },
        0x35 => { // DCR M
            let address: u16 = state.hl();
            let minuend: u8 = state.read_memory(address as usize);
            let answer: u8 = minuend.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.write_memory(address as usize, answer);
        },
        0x36 => { // MVI M, D8
            let address: usize = state.hl() as usize;
            state.write_memory(address, state.read_memory(program_counter + 1));
            state.increment_program_counter(1);
        },
        0x37 => { // STC
            state.cc.cy = 1;
        },
        0x38 => {}, // -
        0x39 => { // DAD SP
            let result: u32 = state.hl() as u32 + state.sp as u32;
            state.cc.cy = if result > 0xffff { 1 } else { 0 };
            state.set_hl(result as u16);
        },
        0x3a => { // LDA adr
            let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
            let low_address = state.read_memory(program_counter + 1) as u16;
            let address: usize = (high_address | low_address) as usize;
            state.a = state.read_memory(address);
            state.increment_program_counter(2);
        },
        0x3b => { // DCX SP
            state.sp = state.sp.wrapping_sub(1);
        },
        0x3c => { // INR A
            let answer: u16 = (state.a as u16) + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x3d => { // DCR A
            let answer: u8 = state.a.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x3e => { // MVI A, D8
            state.a = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x3f => unimplemented_instruction(state), // CMC
        0x40 => state.b = state.b, // MOV B, B
        0x41 => state.b = state.c, // MOV B, C
        0x42 => state.b = state.d, // MOV B, D
        0x43 => state.b = state.e, // MOV B, E
        0x44 => state.b = state.h, // MOV B, H
        0x45 => state.b = state.l, // MOV B, L
        0x46 => { // MOV B, M
            state.b = state.read_memory(state.hl() as usize);
        },
        0x47 => state.b = state.a, // MOV B, A
        0x48 => state.c = state.b, // MOV C, B
        0x49 => state.c = state.c, // MOV C, C
        0x4a => state.c = state.d, // MOV C, D
        0x4b => state.c = state.e, // MOV C, E
        0x4c => state.c = state.h, // MOV C, H
        0x4d => state.c = state.l, // MOV C, L
        0x4e => { // MOV C, M
            state.c = state.read_memory(state.hl() as usize);
        },
        0x4f => state.c = state.a, // MOV C, A
        0x50 => state.d = state.b, // MOV D, B
        0x51 => state.d = state.c, // MOV D, C
        0x52 => state.d = state.d, // MOV D, D
        0x53 => state.d = state.e, // MOV D, E
        0x54 => state.d = state.h, // MOV D, H
        0x55 => state.d = state.l, // MOV D, L
        0x56 => { // MOV D, M
            state.d = state.read_memory(state.hl() as usize);
        },
        0x57 => state.d = state.a, // MOV D, A
        0x58 => state.e = state.b, // MOV E, B
        0x59 => state.e = state.c, // MOV E, C
        0x5a => state.e = state.d, // MOV E, D
        0x5b => state.e = state.e, // MOV E, E
        0x5c => state.e = state.h, // MOV E, H
        0x5d => state.e = state.l, // MOV E, L
        0x5e => { // MOV E, M
            state.e = state.read_memory(state.hl() as usize);
        },
        0x5f => state.e = state.a, // MOV E, A
        0x60 => state.h = state.b, // MOV H, B
        0x61 => state.h = state.c, // MOV H, C
        0x62 => state.h = state.d, // MOV H, D
        0x63 => state.h = state.e, // MOV H, E
        0x64 => state.h = state.h, // MOV H, H
        0x65 => state.h = state.l, // MOV H, L
        0x66 => { // MOV H, M
            state.h = state.read_memory(state.hl() as usize);
        },
        0x67 => state.h = state.a, // MOV H, A
        0x68 => state.l = state.b, // MOV L, B
        0x69 => state.l = state.c, // MOV L, C
        0x6a => state.l = state.d, // MOV L, D
        0x6b => state.l = state.e, // MOV L, E
        0x6c => state.l = state.h, // MOV L, H
        0x6d => state.l = state.l, // MOV L, L
        0x6e => { // MOV L, M
            state.l = state.read_memory(state.hl() as usize);
        },
        0x6f => state.l = state.a, // MOV L, A
        0x70 => { // MOV M, B
            let address: u16 = state.hl();
            state.write_memory(address as usize, state.b);
        }
        0x71 => { // MOV M, C
            let address: u16 = state.hl();
            state.write_memory(address as usize, state.c);
        },
        0x72 => { // MOV M, D
            let address: u16 = state.hl();
            state.write_memory(address as usize, state.d);
        },
        0x73 => { // MOV M, E
            let address: u16 = state.hl();
            state.write_memory(address as usize, state.e);
        },
        0x74 => { // MOV M, H
            let address: u16 = state.hl();
            state.write_memory(address as usize, state.h);
        },
        0x75 => { // MOV M, L
            let address: u16 = state.hl();
            state.write_memory(address as usize, state.l);
        },
        0x76 => { // HLT
            // This is a bit aggressive
            std::process::exit(0);
        },
        0x77 => { // MOV M, A
            let address: u16 = state.hl();
            state.write_memory(address as usize, state.a);
        },
        0x78 => state.a = state.b, // MOV A, B
        0x79 => state.a = state.c, // MOV A, C
        0x7a => state.a = state.d, // MOV A, D
        0x7b => state.a = state.e, // MOV A, E
        0x7c => state.a = state.h, // MOV A, H
        0x7d => state.a = state.l, // MOV A, L
        0x7e => { // MOV A, M
            state.a = state.read_memory(state.hl() as usize);
        },
        0x7f => state.a = state.a, // MOV A, A
        0x80 => { // ADD B
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
        0x81 => { // ADD C
            let answer: u16 = (state.a as u16) + (state.c as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x82 => { // ADD D
            let answer: u16 = (state.a as u16) + (state.d as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x83 => { // ADD E
            let answer: u16 = (state.a as u16) + (state.e as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x84 => { // ADD H
            let answer: u16 = (state.a as u16) + (state.h as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x85 => { // ADD L
            let answer: u16 = (state.a as u16) + (state.l as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x86 => { // ADD M
            let answer: u16 = (state.a as u16) + state.read_memory(state.hl() as usize) as u16;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x87 => { // ADD A
            let answer: u16 = (state.a as u16) + (state.a as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x88 => { // ADC B
            let answer: u16 = (state.a as u16) + (state.b as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x89 => { // ADC C
            let answer: u16 = (state.a as u16) + (state.c as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8a => { // ADC D
            let answer: u16 = (state.a as u16) + (state.d as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8b => { // ADC E
            let answer: u16 = (state.a as u16) + (state.e as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8c => { // ADC H
            let answer: u16 = (state.a as u16) + (state.h as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8d => { // ADC L
            let answer: u16 = (state.a as u16) + (state.l as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8e => { // ADC M
            let answer: u16 = (state.a as u16) + state.read_memory(state.hl() as usize) as u16 + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x8f => { // ADC A
            let answer: u16 = (state.a as u16) + (state.a as u16) + (state.cc.cy as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
        },
        0x90 => { // SUB B
            let answer: u8 = state.a.wrapping_sub(state.b);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.b { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x91 => { // SUB C
            let answer: u8 = state.a.wrapping_sub(state.c);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.c { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x92 => { // SUB D
            let answer: u8 = state.a.wrapping_sub(state.d);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.d { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x93 => { // SUB E
            let answer: u8 = state.a.wrapping_sub(state.e);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.e { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x94 => { // SUB H
            let answer: u8 = state.a.wrapping_sub(state.h);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.h { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x95 => { // SUB L
            let answer: u8 = state.a.wrapping_sub(state.l);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.l { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x96 => { // SUB M
            let subtrahend: u8 = state.read_memory(state.hl() as usize);
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x97 => { // SUB A
            let answer: u8 = state.a - state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < state.a { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x98 => { // SBB B
            let subtrahend: u8 = state.b + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x99 => { // SBB C
            let subtrahend: u8 = state.c + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9a => { // SBB D
            let subtrahend: u8 = state.d + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9b => { // SBB E
            let subtrahend: u8 = state.e + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9c => { // SBB H
            let subtrahend: u8 = state.h + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9d => { // SBB L
            let subtrahend: u8 = state.l + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9e => { // SBB M
            let subtrahend: u8 = state.read_memory(state.hl() as usize) + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0x9f => { // SBB A
            let subtrahend: u8 = state.a + state.cc.cy;
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa0 => { // ANA B
            let answer: u8 = state.a & state.b;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa1 => { // ANA C
            let answer: u8 = state.a & state.c;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa2 => { // ANA D
            let answer: u8 = state.a & state.d;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa3 => { // ANA E
            let answer: u8 = state.a & state.e;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa4 => { // ANA H
            let answer: u8 = state.a & state.h;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa5 => { // ANA L
            let answer: u8 = state.a & state.l;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa6 => { // ANA M
            let answer: u8 = state.a & state.read_memory(state.hl() as usize);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa7 => { // ANA A
            let answer: u8 = state.a & state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa8 => { // XRA B
            let answer: u8 = state.a ^ state.b;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xa9 => { // XRA C
            let answer: u8 = state.a ^ state.c;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xaa => { // XRA D
            let answer: u8 = state.a ^ state.d;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xab => { // XRA E
            let answer: u8 = state.a ^ state.e;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xac => { // XRA H
            let answer: u8 = state.a ^ state.h;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xad => { // XRA L
            let answer: u8 = state.a ^ state.l;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xae => { // XRA M
            let answer: u8 = state.a ^ state.read_memory(state.hl() as usize);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xaf => { // XRA A
            let answer: u8 = state.a ^ state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb0 => { // ORA B
            let answer: u8 = state.a | state.b;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb1 => { // ORA C
            let answer: u8 = state.a | state.c;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb2 => { // ORA D
            let answer: u8 = state.a | state.d;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb3 => { // ORA E
            let answer: u8 = state.a | state.e;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb4 => { // ORA H
            let answer: u8 = state.a | state.h;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb5 => { // ORA L
            let answer: u8 = state.a | state.l;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb6 => { // ORA M
            let answer: u8 = state.a | state.read_memory(state.hl() as usize);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb7 => { // ORA A
            let answer: u8 = state.a | state.a;
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
        },
        0xb8 => unimplemented_instruction(state), // CMP B
        0xb9 => unimplemented_instruction(state), // CMP C
        0xba => unimplemented_instruction(state), // CMP D
        0xbb => unimplemented_instruction(state), // CMP E
        0xbc => unimplemented_instruction(state), // CMP H
        0xbd => unimplemented_instruction(state), // CMP L
        0xbe => unimplemented_instruction(state), // CMP M
        0xbf => unimplemented_instruction(state), // CMP A
        0xc0 => { // RNZ
            if state.cc.z != 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc1 => { // POP B
            let (high, low) = state.pop();
            state.b = high;
            state.c = low;

        },
        0xc2 => { // JNZ adr
            if state.cc.z == 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc3 => { // JMP adr
            let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
            let low_address = state.read_memory(program_counter + 1) as u16;
            state.set_program_counter(high_address | low_address);
        },
        0xc4 => { // CNZ adr
            if state.cc.z != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc5 => { // PUSH B
            state.push(state.b, state.c);
        },
        0xc6 => { // ADI D8
            let answer: u16 = (state.a as u16) + (state.read_memory(program_counter + 1) as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
            state.increment_program_counter(1);
        },
        0xc7 => unimplemented_instruction(state), // RST 0
        0xc8 => { // RZ
            if state.cc.z != 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xc9 => { // RET
            let low_address = state.read_memory(state.sp as usize) as u16;
            let high_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
            state.set_program_counter(high_address | low_address);
            state.sp += 2;
        },
        0xca => { // JZ adr
            if state.cc.z != 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xcb => unimplemented_instruction(state), // -
        0xcc => { // CZ adr
            if state.cc.z == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xcd => { // CALL adr
            let ret: u16 = program_counter as u16 + 3;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp = state.sp - 2;
            let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
            let low_address = state.read_memory(program_counter + 1) as u16;
            state.set_program_counter(high_address | low_address);
        },
        0xce => { // ACI D8
            let answer: u16 = (state.a as u16) + (state.read_memory(program_counter + 1) as u16) + state.cc.cy as u16;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
            state.increment_program_counter(1);
        },
        0xcf => unimplemented_instruction(state), // RST 1
        0xd0 => { // RNC
            if state.cc.cy == 0 {
                let low_address = state.read_memory(state.sp as usize) as u16;
                let high_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd1 => { // POP D
            let (high, low) = state.pop();
            state.d = high;
            state.e = low;
        },
        0xd2 => { // JNC adr
            if state.cc.cy == 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd3 => { // OUT D8
            // TODO: IO
            // This is the OUT instruction, for now just skip data byte
            state.increment_program_counter(1);
        },
        0xd4 => { // CNC adr
            if state.cc.cy == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd5 => { // PUSH D
            state.push(state.d, state.e);
        },
        0xd6 => { // SUI D8
            let subtrahend: u8 = state.read_memory(program_counter + 1);
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
            state.increment_program_counter(1);
        },
        0xd7 => unimplemented_instruction(state), // RST 2
        0xd8 => { // RC
            if state.cc.cy != 0 {
                let low_address = state.read_memory(state.sp as usize) as u16;
                let high_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xd9 => unimplemented_instruction(state), // -
        0xda => { // JC adr
            if state.cc.cy != 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xdb => { // IN D8
            // TODO: IO
            // This is the IN instruction, for now just skip data byte
            state.increment_program_counter(1);
        },
        0xdc => { // CC adr
            if state.cc.cy != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xdd => unimplemented_instruction(state), // -
        0xde => { // SBI D8
            let subtrahend: u8 = state.read_memory(program_counter + 1).wrapping_add(state.cc.cy);
            let answer: u8 = state.a.wrapping_sub(subtrahend);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = parity(answer);
            state.a = answer;
            state.increment_program_counter(1);
        }
        0xdf => unimplemented_instruction(state), // RST 3
        0xe0 => { // RPO
            if state.cc.p == 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe1 => { // POP H
            let (high, low) = state.pop();
            state.h = high;
            state.l = low;
        },
        0xe2 => { // JPO adr
            if state.cc.p == 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe3 => { // XTHL
            let new_l = state.read_memory(state.sp as usize);
            let new_h = state.read_memory(state.sp as usize + 1);
            state.write_memory(state.sp as usize, state.l);
            state.write_memory(state.sp as usize + 1, state.h);
            state.h = new_h;
            state.l = new_l;
        },
        0xe4 => { // CPO adr
            if state.cc.p == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe5 => { // PUSH H
            state.push(state.h, state.l);
        },
        0xe6 => { // ANI D8
            let answer: u8 = state.a & state.read_memory(program_counter + 1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(answer);
            state.a = answer;
            state.increment_program_counter(1);
        },
        0xe7 => unimplemented_instruction(state), // RST 4
        0xe8 => { // RPE
            if state.cc.p != 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xe9 => unimplemented_instruction(state), // PCHL
        0xea => { // JPE adr
            if state.cc.p != 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xeb => { // XCHG
            let new_h = state.d;
            let new_l = state.e;
            state.d = state.h;
            state.e = state.l;
            state.h = new_h;
            state.l = new_l;
        },
        0xec => { // CPE adr
            if state.cc.p != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xed => unimplemented_instruction(state), // -
        0xee => { // XRI D8
            let answer: u16 = (state.a as u16) ^ (state.read_memory(program_counter + 1) as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
            state.increment_program_counter(1);
        },
        0xef => unimplemented_instruction(state), // RST 5
        0xf0 => { // RP
            if state.cc.s == 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xf1 => { // POP PSW
            let (high, psw) = state.pop();
            state.a = high;
            state.cc.cy = if 0x01 == (psw & 0x01) { 1 } else { 0 };
            state.cc.p = if 0x04 == (psw & 0x04) { 1 } else { 0 };
            state.cc.ac = if 0x10 == (psw & 0x10) { 1 } else { 0 };
            state.cc.z = if 0x40 == (psw & 0x40) { 1 } else { 0 };
            state.cc.s = if 0x80 == (psw & 0x80) { 1 } else { 0 };
        },
        0xf2 => { // JP adr
            if state.cc.s == 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xf3 => { // DI
            state.disable_interrupt();
        },
        0xf4 => { // CP adr
            if state.cc.s == 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.set_program_counter(2);
            }
        },
        0xf5 => { // PUSH PSW
            let cc = &state.cc;
            let psw: u8 = cc.cy | cc.p << 2 | cc.ac << 4 | cc.z << 6 | cc.s << 7;
            state.push(state.a, psw);
        },
        0xf6 => { // ORI D8
            let answer: u16 = (state.a as u16) | (state.read_memory(program_counter + 1) as u16);
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.cy = 0;
            state.cc.p = parity(masked_answer);
            state.a = masked_answer;
            state.increment_program_counter(1);
        },
        0xf7 => { // RST 6
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x30);
        },
        0xf8 => { // RM
            if state.cc.s != 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            } else {
                state.increment_program_counter(2);
            }
        },
        0xf9 => { // SPHL
            state.sp = state.hl();
        },
        0xfa => { // JM adr
            if state.cc.s != 0 {
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xfb => { // EI
            state.enable_interrupt();
        },
        0xfc => { // CM adr
            if state.cc.s != 0 {
                let ret: u16 = program_counter as u16 + 2;
                state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
                state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
                state.sp = state.sp - 2;
                let high_address = (state.read_memory(program_counter + 2) as u16) << 8;
                let low_address = state.read_memory(program_counter + 1) as u16;
                state.set_program_counter(high_address | low_address);
            } else {
                state.increment_program_counter(2);
            }
        },
        0xfd => unimplemented_instruction(state), // -
        0xfe => { // CPI D8
            /*
            let acc = state.a;
            let immediate_data = state.read_memory(program_counter + 1);
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
            let answer: u8 = state.a.wrapping_sub(state.read_memory(program_counter + 1));
            let masked_answer: u8 = answer & 0xff;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
            state.cc.p = parity(masked_answer);
            state.cc.cy = if state.a < state.read_memory(program_counter + 1) {
                1
            } else {
                0
            };
            state.increment_program_counter(1);
        },
        0xff => unimplemented_instruction(state), // RST 7
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
        assert_eq!(state.read_memory(2), 0xff);
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
        assert_eq!(state.read_memory(0x01), 0xde);
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
        assert_eq!(state.read_memory(0x0003), 0x09);
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
        assert_eq!(state.read_memory(0x01), 0x45);
        assert_eq!(state.read_memory(0x02), 0x47);
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
        assert_eq!(state.read_memory(0x02), 0x45);
        assert_eq!(state.read_memory(0x03), 0x47);
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
        assert_eq!(state.read_memory(0x02), 0x8f);
        assert_eq!(state.read_memory(0x01), 0x9d);
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
        assert_eq!(state.read_memory(0x01), 0x3c);
        assert_eq!(state.read_memory(0x02), 0x0b);
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
        assert_eq!(state.program_counter(), 2);
    }

    #[test]
    fn test_ani() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xe6, 0x0f];
        state.a = 0x3a;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x0a);
    }

    #[test]
    fn test_adi() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xc6, 0x42, 0xc6, 0xbe];
        state.a = 0x14;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x56);
        assert_eq!(state.cc.p, 1);
        assert_eq!(state.cc.cy, 0);
        // TODO: assert_eq!(state.cc.ac, 0);
        assert_eq!(state.cc.z, 0);
        assert_eq!(state.cc.s, 0);

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x14);
        assert_eq!(state.cc.p, 1);
        assert_eq!(state.cc.cy, 1);
        // TODO: assert_eq!(state.cc.ac, 1);
        assert_eq!(state.cc.z, 0);
        assert_eq!(state.cc.s, 0);
    }

    #[test]
    fn test_aci() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xce, 0xbe, 0xce, 0x42];
        state.a = 0x56;

        assert_eq!(state.cc.cy, 0);
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x14);
        assert_eq!(state.cc.cy, 1);

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x57);
        assert_eq!(state.cc.cy, 0);
    }

    #[test]
    fn test_sui() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xd6, 0x01];
        state.a = 0x00;
        state.cc.cy = 0;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xff);
        assert_eq!(state.cc.cy, 1);
    }

    #[test]
    fn test_sbi() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xde, 0x01];
        state.a = 0x00;
        state.cc.cy = 0;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xff);
        assert_eq!(state.cc.cy, 1);
        assert_eq!(state.cc.s, 1);
        assert_eq!(state.cc.p, 1);
        assert_eq!(state.cc.z, 0);
        // TODO: assert_eq!(state.cc.ac, 0);
    }

    #[test]
    fn test_sbi_with_carry_bit() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xde, 0x01];
        state.a = 0x00;
        state.cc.cy = 1;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xfe);
        assert_eq!(state.cc.cy, 1);
        assert_eq!(state.cc.s, 1);
        assert_eq!(state.cc.p, 0);
        assert_eq!(state.cc.z, 0);
        // TODO: assert_eq!(state.cc.ac, 0);
    }

    #[test]
    fn test_ori() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xf6, 0x0f];
        state.a = 0xb5;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xbf);
    }

    #[test]
    fn test_xri() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xee, 0x81];
        state.a = 0x3b;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xba);
    }

    #[test]
    fn test_xchg() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xeb];
        state.d = 0x12;
        state.e = 0x34;
        state.h = 0x56;
        state.l = 0x78;

        emulate_8080_op(&mut state);
        assert_eq!(state.d, 0x56);
        assert_eq!(state.e, 0x78);
        assert_eq!(state.h, 0x12);
        assert_eq!(state.l, 0x34);
    }

    #[test]
    fn test_rrc() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x0f];
        state.a = 0xf2;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x79);
        assert_eq!(state.cc.cy, 0);
    }

    #[test]
    fn test_rrc_carry() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x0f];
        state.a = 0x01;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x80);
        assert_eq!(state.cc.cy, 1);
    }

    #[test]
    fn test_rlc() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x07];
        state.a = 0xf2;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xe5);
        assert_eq!(state.cc.cy, 1);
    }

    #[test]
    fn test_rar() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x1f];
        state.a = 0x6a;
        state.cc.cy = 1;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xb5);
        assert_eq!(state.cc.cy, 0);
    }

    #[test]
    fn test_daa() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x27];
        state.a = 0x9b;
        state.cc.cy = 0;
        state.cc.ac = 0;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x01);
        assert_eq!(state.cc.cy, 1);
        assert_eq!(state.cc.ac, 1);
    }

    #[test]
    fn test_shld() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x22, 0x03, 0x00, 0x00, 0x00];
        state.h = 0xae;
        state.l = 0x29;

        emulate_8080_op(&mut state);
        assert_eq!(state.read_memory(0x03), 0x29);
        assert_eq!(state.read_memory(0x04), 0xae);
        assert_eq!(state.program_counter(), 3);
    }

    #[test]
    fn test_lhld() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x2a, 0x03, 0x00, 0xff, 0x03];
        state.l = 0x00;
        state.h = 0x00;

        emulate_8080_op(&mut state);
        assert_eq!(state.l, 0xff);
        assert_eq!(state.h, 0x03);
        assert_eq!(state.program_counter(), 3);
    }

    #[test]
    fn test_lhld_high() {
        let mut state = State8080::empty_state();
        state.memory = vec![0; 0x2ff];
        state.write_memory(0x00, 0x2a);
        state.write_memory(0x01, 0xab);
        state.write_memory(0x02, 0x02);
        state.write_memory(0x2ab, 0xff);
        state.write_memory(0x2ac, 0x03);

        state.l = 0x00;
        state.h = 0x00;

        emulate_8080_op(&mut state);
        assert_eq!(state.l, 0xff);
        assert_eq!(state.h, 0x03);
        assert_eq!(state.program_counter(), 3);
    }

    #[test]
    fn test_ral() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x17];
        state.a = 0xb5;
        state.cc.cy = 0;

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x6a);
        assert_eq!(state.cc.cy, 1);
    }

    #[test]
    fn test_inr() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x0c];
        state.c = 0x99;
        emulate_8080_op(&mut state);
        assert_eq!(state.c, 0x9a);
    }

    #[test]
    fn test_inr_mem() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x34, 0x99];
        state.set_hl(0x01);
        emulate_8080_op(&mut state);
        assert_eq!(state.read_memory(0x01), 0x9a);
    }

    #[test]
    fn test_stax() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x00; 0x4000];
        state.write_memory(0x00, 0x02);
        state.a = 0xde;
        state.b = 0x3f;
        state.c = 0x16;
        emulate_8080_op(&mut state);
        assert_eq!(state.read_memory(0x3f16), 0xde);
    }
}
