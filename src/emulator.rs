use crate::state_8080::State8080;
use crate::disassembler;
use crate::parity::Parity;
use crate::sign::Sign;

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
            state.b = state.read_memory(program_counter + 2);
            state.c = state.read_memory(program_counter + 1);
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
            State8080::increment_register(&mut state.b, &mut state.cc);
        },
        0x05 => { // DCR B
            State8080::decrement_register(&mut state.b, &mut state.cc);
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
            State8080::increment_register(&mut state.c, &mut state.cc);
        },
        0x0d => { // DCR C
            State8080::decrement_register(&mut state.c, &mut state.cc);
        },
        0x0e => { // MVI C, D8
            state.c = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x0f => { // RRC
            state.cc.cy = state.a & 0x01;
            state.a = state.a.rotate_right(1);
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
            State8080::increment_register(&mut state.d, &mut state.cc);
        },
        0x15 => { // DCR D
            State8080::decrement_register(&mut state.d, &mut state.cc);
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
            State8080::increment_register(&mut state.e, &mut state.cc);
        },
        0x1d => { // DCR E
            State8080::decrement_register(&mut state.e, &mut state.cc);
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
            State8080::increment_register(&mut state.h, &mut state.cc);
        },
        0x25 => { // DCR H
            State8080::decrement_register(&mut state.h, &mut state.cc);
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
            State8080::increment_register(&mut state.l, &mut state.cc);
        },
        0x2d => { // DCR L
            State8080::decrement_register(&mut state.l, &mut state.cc);
        },
        0x2e => { // MVI L, D8
            state.l = state.read_memory(program_counter + 1);
            state.increment_program_counter(1);
        },
        0x2f => { // CMA
            state.a = !state.a;
        },
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
            let answer: u16 = state.m() as u16 + 1;
            let masked_answer: u8 = (answer & 0xff) as u8;
            state.cc.z = if masked_answer == 0 { 1 } else { 0 };
            state.cc.s = Sign::get_sign(masked_answer);
            state.cc.p = Parity::from(masked_answer);
            state.set_m(masked_answer);
        },
        0x35 => { // DCR M
            let minuend: u8 = state.m();
            let answer: u8 = minuend.wrapping_sub(1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = Sign::get_sign(answer);
            state.cc.p = Parity::from(answer);
            state.set_m(answer);
        },
        0x36 => { // MVI M, D8
            state.set_m(state.read_memory(program_counter + 1));
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
            State8080::increment_register(&mut state.a, &mut state.cc);
        },
        0x3d => { // DCR A
            State8080::decrement_register(&mut state.a, &mut state.cc);
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
        0x46 => { state.b = state.m(); }, // MOV B, M
        0x47 => state.b = state.a, // MOV B, A
        0x48 => state.c = state.b, // MOV C, B
        0x49 => state.c = state.c, // MOV C, C
        0x4a => state.c = state.d, // MOV C, D
        0x4b => state.c = state.e, // MOV C, E
        0x4c => state.c = state.h, // MOV C, H
        0x4d => state.c = state.l, // MOV C, L
        0x4e => { state.c = state.m(); }, // MOV C, M
        0x4f => state.c = state.a, // MOV C, A
        0x50 => state.d = state.b, // MOV D, B
        0x51 => state.d = state.c, // MOV D, C
        0x52 => state.d = state.d, // MOV D, D
        0x53 => state.d = state.e, // MOV D, E
        0x54 => state.d = state.h, // MOV D, H
        0x55 => state.d = state.l, // MOV D, L
        0x56 => { state.d = state.m(); }, // MOV D, M
        0x57 => state.d = state.a, // MOV D, A
        0x58 => state.e = state.b, // MOV E, B
        0x59 => state.e = state.c, // MOV E, C
        0x5a => state.e = state.d, // MOV E, D
        0x5b => state.e = state.e, // MOV E, E
        0x5c => state.e = state.h, // MOV E, H
        0x5d => state.e = state.l, // MOV E, L
        0x5e => { state.e = state.m(); }, // MOV E, M
        0x5f => state.e = state.a, // MOV E, A
        0x60 => state.h = state.b, // MOV H, B
        0x61 => state.h = state.c, // MOV H, C
        0x62 => state.h = state.d, // MOV H, D
        0x63 => state.h = state.e, // MOV H, E
        0x64 => state.h = state.h, // MOV H, H
        0x65 => state.h = state.l, // MOV H, L
        0x66 => { state.h = state.m(); }, // MOV H, M
        0x67 => state.h = state.a, // MOV H, A
        0x68 => state.l = state.b, // MOV L, B
        0x69 => state.l = state.c, // MOV L, C
        0x6a => state.l = state.d, // MOV L, D
        0x6b => state.l = state.e, // MOV L, E
        0x6c => state.l = state.h, // MOV L, H
        0x6d => state.l = state.l, // MOV L, L
        0x6e => { state.l = state.m(); }, // MOV L, M
        0x6f => state.l = state.a, // MOV L, A
        0x70 => { state.set_m(state.b); } // MOV M, B
        0x71 => { state.set_m(state.c); }, // MOV M, C
        0x72 => { state.set_m(state.d); }, // MOV M, D
        0x73 => { state.set_m(state.e); }, // MOV M, E
        0x74 => { state.set_m(state.h); }, // MOV M, H
        0x75 => { state.set_m(state.l); }, // MOV M, L
        0x76 => { // HLT
            // This is a bit aggressive
            std::process::exit(0);
        },
        0x77 => { state.set_m(state.a); }, // MOV M, A
        0x78 => state.a = state.b, // MOV A, B
        0x79 => state.a = state.c, // MOV A, C
        0x7a => state.a = state.d, // MOV A, D
        0x7b => state.a = state.e, // MOV A, E
        0x7c => state.a = state.h, // MOV A, H
        0x7d => state.a = state.l, // MOV A, L
        0x7e => { state.a = state.m(); }, // MOV A, M
        0x7f => state.a = state.a, // MOV A, A
        0x80 => { state.add(state.b); }, // ADD B
        0x81 => { state.add(state.c); }, // ADD C
        0x82 => { state.add(state.d); }, // ADD D
        0x83 => { state.add(state.e); }, // ADD E
        0x84 => { state.add(state.h); }, // ADD H
        0x85 => { state.add(state.l); }, // ADD L
        0x86 => { state.add(state.m()); }, // ADD M
        0x87 => { state.add(state.a); }, // ADD A
        0x88 => { state.adc(state.b); }, // ADC B
        0x89 => { state.adc(state.c); }, // ADC C
        0x8a => { state.adc(state.d); }, // ADC D
        0x8b => { state.adc(state.e); }, // ADC E
        0x8c => { state.adc(state.h); }, // ADC H
        0x8d => { state.adc(state.l); }, // ADC L
        0x8e => { state.adc(state.m()); }, // ADC M
        0x8f => { state.adc(state.a); }, // ADC A
        0x90 => { state.sub(state.b); }, // SUB B
        0x91 => { state.sub(state.c); }, // SUB C
        0x92 => { state.sub(state.d); }, // SUB D
        0x93 => { state.sub(state.e); }, // SUB E
        0x94 => { state.sub(state.h); }, // SUB H
        0x95 => { state.sub(state.l); }, // SUB L
        0x96 => { state.sub(state.m()); }, // SUB M
        0x97 => { state.sub(state.a); }, // SUB A
        0x98 => { state.sbb(state.b); }, // SBB B
        0x99 => { state.sbb(state.c); }, // SBB C
        0x9a => { state.sbb(state.d); }, // SBB D
        0x9b => { state.sbb(state.e); }, // SBB E
        0x9c => { state.sbb(state.h); }, // SBB H
        0x9d => { state.sbb(state.l); }, // SBB L
        0x9e => { state.sbb(state.m()); }, // SBB M
        0x9f => { state.sbb(state.a); }, // SBB A
        0xa0 => { state.ana(state.b); }, // ANA B
        0xa1 => { state.ana(state.c); }, // ANA C
        0xa2 => { state.ana(state.d); }, // ANA D
        0xa3 => { state.ana(state.e); }, // ANA E
        0xa4 => { state.ana(state.h); }, // ANA H
        0xa5 => { state.ana(state.l); }, // ANA L
        0xa6 => { state.ana(state.m()); }, // ANA M
        0xa7 => { state.ana(state.a); }, // ANA A
        0xa8 => { state.xra(state.b); }, // XRA B
        0xa9 => { state.xra(state.c); }, // XRA C
        0xaa => { state.xra(state.d); }, // XRA D
        0xab => { state.xra(state.e); }, // XRA E
        0xac => { state.xra(state.h); }, // XRA H
        0xad => { state.xra(state.l); }, // XRA L
        0xae => { state.xra(state.m()); }, // XRA M
        0xaf => { state.xra(state.a); }, // XRA A
        0xb0 => { state.ora(state.b); }, // ORA B
        0xb1 => { state.ora(state.c); }, // ORA C
        0xb2 => { state.ora(state.d); }, // ORA D
        0xb3 => { state.ora(state.e); }, // ORA E
        0xb4 => { state.ora(state.h); }, // ORA H
        0xb5 => { state.ora(state.l); }, // ORA L
        0xb6 => { state.ora(state.m()); }, // ORA M
        0xb7 => { state.ora(state.a); }, // ORA A
        0xb8 => { state.cmp(state.b); }, // CMP B
        0xb9 => { state.cmp(state.c); }, // CMP C
        0xba => { state.cmp(state.d); }, // CMP D
        0xbb => { state.cmp(state.e); }, // CMP E
        0xbc => { state.cmp(state.h); }, // CMP H
        0xbd => { state.cmp(state.l); }, // CMP L
        0xbe => { state.cmp(state.m()); }, // CMP M
        0xbf => { state.cmp(state.a); }, // CMP A
        0xc0 => { // RNZ
            if state.cc.z != 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
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
            state.cc.s = Sign::get_sign(masked_answer);
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = Parity::from(masked_answer);
            state.a = masked_answer;
            state.increment_program_counter(1);
        },
        0xc7 => { // RST 0
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x00);
        },
        0xc8 => { // RZ
            if state.cc.z != 0 {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
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
            state.cc.s = Sign::get_sign(masked_answer);
            state.cc.cy = if answer > 0xff { 1 } else { 0 };
            state.cc.p = Parity::from(masked_answer);
            state.a = masked_answer;
            state.increment_program_counter(1);
        },
        0xcf => { // RST 1
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x08);
        }
        0xd0 => { // RNC
            if state.cc.cy == 0 {
                let low_address = state.read_memory(state.sp as usize) as u16;
                let high_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
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
            state.cc.s = Sign::get_sign(answer);
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = Parity::from(answer);
            state.a = answer;
            state.increment_program_counter(1);
        },
        0xd7 => { // RST 2
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x10);
        },
        0xd8 => { // RC
            if state.cc.cy != 0 {
                let low_address = state.read_memory(state.sp as usize) as u16;
                let high_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
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
            state.cc.s = Sign::get_sign(answer);
            state.cc.cy = if state.a < subtrahend { 1 } else { 0 };
            state.cc.p = Parity::from(answer);
            state.a = answer;
            state.increment_program_counter(1);
        }
        0xdf => { // RST 3
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x18);
        }
        0xe0 => { // RPO
            if state.cc.p == Parity::Odd {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            }
        },
        0xe1 => { // POP H
            let (high, low) = state.pop();
            state.h = high;
            state.l = low;
        },
        0xe2 => { // JPO adr
            if state.cc.p == Parity::Odd {
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
            if state.cc.p == Parity::Odd {
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
            state.cc.s = Sign::get_sign(answer);
            state.cc.cy = 0;
            state.cc.p = Parity::from(answer);
            state.a = answer;
            state.increment_program_counter(1);
        },
        0xe7 => { // RST 4
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x20);
        }
        0xe8 => { // RPE
            if state.cc.p == Parity::Even {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            }
        },
        0xe9 => { // PCHL
            state.set_program_counter(state.hl());
        },
        0xea => { // JPE adr
            if state.cc.p == Parity::Even {
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
            if state.cc.p == Parity::Even {
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
            state.cc.s = Sign::get_sign(masked_answer);
            state.cc.cy = 0;
            state.cc.p = Parity::from(masked_answer);
            state.a = masked_answer;
            state.increment_program_counter(1);
        },
        0xef => { // RST 5
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x28);
        }
        0xf0 => { // RP
            if state.cc.s == Sign::Positive {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            }
        },
        0xf1 => { // POP PSW
            let (high, psw) = state.pop();
            state.a = high;
            state.cc.cy = if 0x01 == (psw & 0x01) { 1 } else { 0 };
            state.cc.p = if 0x04 == (psw & 0x04) { Parity::Even } else { Parity::Odd };
            state.cc.ac = if 0x10 == (psw & 0x10) { 1 } else { 0 };
            state.cc.z = if 0x40 == (psw & 0x40) { 1 } else { 0 };
            state.cc.s = Sign::get_sign(psw);
        },
        0xf2 => { // JP adr
            if state.cc.s == Sign::Positive {
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
            if state.cc.s == Sign::Positive {
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
        0xf5 => { // PUSH PSW
            let cc = &state.cc;
            let psw: u8 = cc.cy | Into::<u8>::into(cc.p) << 2 | cc.ac << 4 | cc.z << 6 | Into::<u8>::into(cc.s) << 7;
            state.push(state.a, psw);
        },
        0xf6 => { // ORI D8
            let answer: u8 = state.a | state.read_memory(program_counter + 1);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = Sign::get_sign(answer);
            state.cc.cy = 0;
            state.cc.p = Parity::from(answer);
            state.a = answer;
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
            if state.cc.s == Sign::Negative {
                let high_address = state.read_memory(state.sp as usize) as u16;
                let low_address = (state.read_memory(state.sp as usize + 1) as u16) << 8;
                state.set_program_counter(high_address | low_address);
                state.sp += 2;
            }
        },
        0xf9 => { // SPHL
            state.sp = state.hl();
        },
        0xfa => { // JM adr
            if state.cc.s == Sign::Negative {
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
            if state.cc.s == Sign::Negative {
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
            let immediate_data = state.read_memory(program_counter + 1);
            let answer: u8 = state.a.wrapping_sub(immediate_data);
            state.cc.z = if answer == 0 { 1 } else { 0 };
            state.cc.s = Sign::get_sign(answer);
            state.cc.p = Parity::from(answer);
            state.cc.cy = if state.a < immediate_data { 1 } else { 0 };
            state.increment_program_counter(1);
        },
        0xff => { // RST 7
            let ret: u16 = state.program_counter() + 2;
            state.write_memory(state.sp as usize - 1, ((ret >> 8) & 0xff) as u8);
            state.write_memory(state.sp as usize - 2, (ret & 0xff) as u8);
            state.sp -= 2;
            state.set_program_counter(0x38);
        },
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
        assert_eq!(state.cc.p, Parity::Even);
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
        state.cc.p = Parity::Odd;
        state.cc.ac = 0x00;
        state.cc.z = 0x00;
        state.cc.s = Sign::Positive;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xff);
        assert_eq!(state.cc.cy, 0x01);
        assert_eq!(state.cc.p, Parity::Odd);
        assert_eq!(state.cc.ac, 0x00);
        assert_eq!(state.cc.z, 0x01);
        assert_eq!(state.cc.s, Sign::Negative);
        assert_eq!(state.sp, 0x03);
    }

    #[test]
    fn test_push_psw() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xf5, 0x00, 0x00];
        state.sp = 0x03;
        state.a = 0x47;
        state.cc.cy = 0x01;
        state.cc.p = Parity::Even;
        state.cc.ac = 0x00;
        state.cc.z = 0x01;
        state.cc.s = Sign::Positive;
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
        state.cc.p = Parity::Even;
        state.cc.ac = 0x00;
        state.cc.z = 0x01;
        state.cc.s = Sign::Positive;

        emulate_8080_op(&mut state);
        assert_eq!(state.read_memory(0x02), 0x45);
        assert_eq!(state.read_memory(0x03), 0x47);
        assert_eq!(state.sp, 0x02);

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x47);
        assert_eq!(state.cc.cy, 0x01);
        assert_eq!(state.cc.p, Parity::Even);
        assert_eq!(state.cc.ac, 0x00);
        assert_eq!(state.cc.z, 0x01);
        assert_eq!(state.cc.s, Sign::Positive);
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
        assert_eq!(state.cc.p, Parity::Even);
        assert_eq!(state.cc.s, Sign::Negative);
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
        state.memory = vec![0xfe, 0x40];
        state.a = 0x4a;
        state.cc.cy = 1;
        state.cc.z = 1;
        emulate_8080_op(&mut state);
        assert_eq!(state.cc.cy, 0);
        assert_eq!(state.cc.z, 0);
        assert_eq!(state.program_counter(), 2);
    }

    #[test]
    fn test_cpi_equal() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xfe, 0x4a];
        state.a = 0x4a;
        emulate_8080_op(&mut state);
        assert_eq!(state.cc.cy, 0);
        assert_eq!(state.cc.z, 1);
        assert_eq!(state.program_counter(), 2);
    }

    #[test]
    fn test_cpi_greater() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xfe, 0x4b];
        state.a = 0x4a;
        emulate_8080_op(&mut state);
        assert_eq!(state.cc.cy, 1);
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
        assert_eq!(state.cc.p, Parity::Even);
        assert_eq!(state.cc.cy, 0);
        // TODO: assert_eq!(state.cc.ac, 0);
        assert_eq!(state.cc.z, 0);
        assert_eq!(state.cc.s, Sign::Positive);

        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x14);
        assert_eq!(state.cc.p, Parity::Even);
        assert_eq!(state.cc.cy, 1);
        // TODO: assert_eq!(state.cc.ac, 1);
        assert_eq!(state.cc.z, 0);
        assert_eq!(state.cc.s, Sign::Positive);
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
        assert_eq!(state.cc.s, Sign::Negative);
        assert_eq!(state.cc.p, Parity::Even);
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
        assert_eq!(state.cc.s, Sign::Negative);
        assert_eq!(state.cc.p, Parity::Odd);
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

    #[test]
    fn test_cmp() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xbb];
        state.a = 0x0a;
        state.e = 0x05;
        state.cc.cy = 0x01;
        state.cc.z = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x0a);
        assert_eq!(state.e, 0x05);
        assert_eq!(state.cc.cy, 0x00);
        assert_eq!(state.cc.z, 0x00);
    }

    #[test]
    fn test_cmp_2() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xbb];
        state.a = 0x02;
        state.e = 0x05;
        state.cc.cy = 0x00;
        state.cc.z = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0x02);
        assert_eq!(state.e, 0x05);
        assert_eq!(state.cc.cy, 0x01);
        assert_eq!(state.cc.z, 0x00);
    }

    #[test]
    fn test_cmp_3() {
        let mut state = State8080::empty_state();
        state.memory = vec![0xbb];
        state.a = 0xe5;
        state.e = 0x05;
        state.cc.cy = 0x01;
        state.cc.z = 0x01;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xe5);
        assert_eq!(state.e, 0x05);
        assert_eq!(state.cc.cy, 0x00);
        assert_eq!(state.cc.z, 0x00);
    }

    #[test]
    fn test_cma() {
        let mut state = State8080::empty_state();
        state.memory = vec![0x2f];
        state.a = 0x51;
        emulate_8080_op(&mut state);
        assert_eq!(state.a, 0xae);
    }
}
