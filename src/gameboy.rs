#[derive(Clone, Copy)]
struct reg_8_bit {
    lower: u8,
    upper: u8,
}

#[derive(Clone, Copy)]
union reg_16_bit {
    split: reg_8_bit,
    whole: u16,
}

pub struct cpu {
    AF: reg_16_bit,
    BC: reg_16_bit,
    DE: reg_16_bit,
    HL: reg_16_bit,
    SP: reg_16_bit,
    PC: reg_16_bit,
    mem: Memory,
}

impl cpu {
    pub fn new() -> cpu {
        let AF = reg_16_bit{ whole: 0 };
        let BC = reg_16_bit{ whole: 0 };
        let DE = reg_16_bit{ whole: 0 };
        let HL = reg_16_bit{ whole: 0 };
        let SP = reg_16_bit{ whole: 0 };
        let PC = reg_16_bit{ whole: 0 };
        let mem = Memory{ ram: [0; 8192] };
        cpu {AF, BC, DE, HL, SP, PC, mem}
    }
    pub fn set_A(&mut self, val: u8) {
        unsafe {
            self.AF.split.upper = val;
        }
    }
    pub fn set_F(&mut self, val: u8) {
        unsafe {
            self.AF.split.lower = val;
        }
    }
    pub fn set_AF(&mut self, val: u16) {
        unsafe {
            self.AF.whole = val;
        }
    }
    pub fn set_B(&mut self, val: u8) {
        unsafe {
            self.BC.split.upper = val;
        }
    }
    pub fn set_C(&mut self, val: u8) {
        unsafe {
            self.BC.split.lower = val;
        }
    }
    pub fn set_BC(&mut self, val: u16) {
        unsafe {
            self.BC.whole = val;
        }
    }
    pub fn set_D(&mut self, val: u8) {
        unsafe {
            self.DE.split.upper = val;
        }
    }
    pub fn set_E(&mut self, val: u8) {
        unsafe {
            self.DE.split.lower = val;
        }
    }
    pub fn set_DE(&mut self, val: u16) {
        unsafe {
            self.DE.whole = val;
        }
    }
    pub fn set_H(&mut self, val: u8) {
        unsafe {
            self.HL.split.upper = val;
        }
    }
    pub fn set_L(&mut self, val: u8) {
        unsafe {
            self.HL.split.lower = val;
        }
    }
    pub fn set_HL(&mut self, val: u16) {
        unsafe {
            self.HL.whole = val;
        }
    }
    pub fn get_A(&self) -> u8{
        unsafe {
            self.AF.split.upper
        }
    }
    pub fn get_F(&self) -> u8{
        unsafe {
            self.AF.split.lower
        }
    }
    pub fn get_AF(&self) -> u16{
        unsafe {
            self.AF.whole
        }
    }
}

struct Memory {
    ram: [u8; 8192],
    cartridge: [u8; 32768],
}
