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

    pub fn enable_interrupt(&mut self) {
        self.int_enable = 1;
    }

    pub fn disable_interrupt(&mut self) {
        self.int_enable = 0;
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
