#[derive(Debug)]
pub struct ConditionCodes {
    pub z: u8,
    pub s: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
    pub pad: u8,
}

pub struct State8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pc: u16,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    int_enable: u8,
}

impl State8080 {
    pub fn program_counter(&self) -> u16 {
        self.pc
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn increment_program_counter(&mut self, delta: u16) {
        self.pc += delta;
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, result: u16) {
        self.b = ((result & 0xff00) >> 8) as u8;
        self.c = (result & 0x00ff) as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_de(&mut self, result: u16) {
        self.d = ((result & 0xff00) >> 8) as u8;
        self.e = (result & 0x00ff) as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, result: u16) {
        self.h = ((result & 0xff00) >> 8) as u8;
        self.l = (result & 0x00ff) as u8;
    }

    pub fn add(&mut self, value: u8) {
        let answer: u16 = (self.a as u16) + (value as u16);
        let masked_answer: u8 = (answer & 0xff) as u8;
        self.cc.z = if masked_answer == 0 { 1 } else { 0 };
        self.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        self.cc.cy = if answer > 0xff { 1 } else { 0 };
        self.cc.p = parity(masked_answer);
        self.a = masked_answer;
    }

    pub fn addc(&mut self, value: u8) {
        let answer: u16 = (self.a as u16) + (value as u16) + (self.cc.cy as u16);
        let masked_answer: u8 = (answer & 0xff) as u8;
        self.cc.z = if masked_answer == 0 { 1 } else { 0 };
        self.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        self.cc.cy = if answer > 0xff { 1 } else { 0 };
        self.cc.p = parity(masked_answer);
        self.a = masked_answer;
    }

    pub fn sub(&mut self, value: u8) {
        let answer: u8 = self.a.wrapping_sub(value);
        self.cc.z = if answer == 0 { 1 } else { 0 };
        self.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        self.cc.cy = if self.a < value { 1 } else { 0 };
        self.cc.p = parity(answer);
        self.a = answer;
    }

    pub fn subb(&mut self, value: u8) {
        let subtrahend: u8 = value + self.cc.cy;
        let answer: u8 = self.a.wrapping_sub(subtrahend);
        self.cc.z = if answer == 0 { 1 } else { 0 };
        self.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        self.cc.cy = if self.a < subtrahend { 1 } else { 0 };
        self.cc.p = parity(answer);
        self.a = answer;
    }

    pub fn ana(&mut self, value: u8) {
        let answer: u8 = self.a & value;
        self.cc.z = if answer == 0 { 1 } else { 0 };
        self.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        self.cc.cy = 0;
        self.cc.p = parity(answer);
        self.a = answer;
    }

    pub fn xra(&mut self, value: u8) {
        let answer: u8 = self.a ^ value;
        self.cc.z = if answer == 0 { 1 } else { 0 };
        self.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        self.cc.cy = 0;
        self.cc.p = parity(answer);
        self.a = answer;
    }

    pub fn ora(&mut self, value: u8) {
        let answer: u8 = self.a | value;
        self.cc.z = if answer == 0 { 1 } else { 0 };
        self.cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        self.cc.cy = 0;
        self.cc.p = parity(answer);
        self.a = answer;
    }

    // I'm not sure how to implement this as a method on the state object and
    // also use it for multiple registers without resorting to something that
    // would add a lot of complexity (interior mutability? enum of all
    // regitsers with a match?), which is why it's a function.
    pub fn decrement_register(register_value: &mut u8, cc: &mut ConditionCodes) {
        let answer: u8 = register_value.wrapping_sub(1);
        cc.z = if answer == 0 { 1 } else { 0 };
        cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        cc.p = parity(answer);
        *register_value = answer;
    }

    pub fn increment_register(register_value: &mut u8, cc: &mut ConditionCodes) {
        let answer: u16 = (register_value.clone() as u16) + 1;
        let masked_answer: u8 = (answer & 0xff) as u8;
        cc.z = if masked_answer == 0 { 1 } else { 0 };
        cc.s = if (answer & 0x80) == 0x80 { 1 } else { 0 };
        cc.p = parity(masked_answer);
        *register_value = masked_answer;
    }

    pub fn interrupt_enabled(&self) -> bool {
        self.int_enable != 0
    }

    pub fn enable_interrupt(&mut self) {
        self.int_enable = 1;
    }

    pub fn disable_interrupt(&mut self) {
        self.int_enable = 0;
    }

    pub fn generate_interrupt(&mut self, interrupt_num: u16) {
        let high = ((self.pc & 0xff00) >> 8) as u8;
        let low = (self.pc & 0xff) as u8;
        self.memory[self.sp as usize - 1] = high;
        self.memory[self.sp as usize - 2] = low;
        self.sp -= 2;
        self.pc = 8 * interrupt_num;
    }

    pub fn read_memory(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn write_memory(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn push(&mut self, high: u8, low: u8) {
        self.write_memory(self.sp as usize - 1, high);
        self.write_memory(self.sp as usize - 2, low);
        self.sp -= 2;
    }

    pub fn pop(&mut self) -> (u8, u8) {
        let low = self.read_memory(self.sp as usize);
        let high = self.read_memory(self.sp as usize + 1);
        self.sp += 2;
        (high, low)
    }

    pub fn empty_state() -> State8080 {
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

// Returns 1 for even parity, 0 for odd
fn parity(x: u8) -> u8 {
    let mut p: u8 = x ^ x.checked_shr(1).unwrap_or(0);
    p ^= p.checked_shr(2).unwrap_or(0);
    p ^= p.checked_shr(4).unwrap_or(0);
    p ^= p.checked_shr(8).unwrap_or(0);
    if (p & 0x01) == 1 { 0 } else { 1 }
}
