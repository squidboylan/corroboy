use std::fs::File;
use std::io::prelude::*;

pub struct Mmu {
    ram: [u8; 8192],
    cartridge: [u8; 32768],
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu{ ram: [0; 8192], cartridge: [0; 32768] }
    }

    pub fn get_mem_u8(&self, location: usize) -> u8 {
        if location < 0x4000 {
            return self.cartridge[location];
        }
        if location < 0xE000 && location >= 0xC000 {
            return self.ram[location - 0xC000];
        }
        0
    }

    pub fn set_mem_u8(&mut self, location: usize, val: u8) {
        if location < 0x4000 {
            self.cartridge[location] = val;
        }
        if location < 0xE000 && location >= 0xC000 {
            self.ram[location - 0xC000] = val;
        }
    }

    // z80 is little endian in ram
    pub fn get_mem_u16(&self, location: usize) -> u16 {
        if location < 0x4000 {
            return (self.cartridge[location] as u16) + ((self.cartridge[location + 1] as u16) << 8);
        }
        if location < 0xE000 && location >= 0xC000 {
            return (self.ram[location - 0xC000] as u16) + ((self.ram[location + 1 - 0xC000] as u16) << 8);
        }

        0
    }

    // z80 is little endian in ram
    pub fn set_mem_u16(&mut self, location: usize, val: u16) {
        if location < 0x4000 {
            self.cartridge[location] = val as u8;
            self.cartridge[location + 1] = (val >> 8) as u8;
        }
        if location < 0xE000 && location >= 0xC000 {
            self.ram[location - 0xC000] = val as u8;
            self.ram[location + 1 - 0xC000] = (val >> 8) as u8;
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).expect("rom not found");

        let mut contents: Vec<u8> = vec![];
        f.read_to_end(&mut contents);
        for (index, i) in contents.iter().enumerate() {
            self.cartridge[index] = *i;
        }
    }

    pub fn push_u8(&mut self, sp: &mut u16, val: u8) {
        if *sp < 0xE000 && *sp >= 0xC000 {
            *sp = *sp - 1;
            self.ram[(*sp - 0xC000) as usize] = val;
        }
    }

    pub fn pop_u8(&mut self, sp: &mut u16) -> u8{
        let mut val: u8 = 0;
        if *sp < 0xE000 && *sp >= 0xC000 {
            val = self.ram[(*sp - 0xC000) as usize];
            *sp = *sp + 1;
        }

        val
    }

    pub fn push_u16(&mut self, sp: &mut u16, val: u16) {
        if *sp < 0xE000 && *sp >= 0xC000 {
            *sp = *sp - 1;
            self.ram[(*sp - 0xC000) as usize] = (val >> 8) as u8;
            *sp = *sp - 1;
            self.ram[(*sp - 0xC000) as usize] = val as u8;
        }
    }

    pub fn pop_u16(&mut self, sp: &mut u16) -> u16{
        let mut val: u16 = 0;
        if *sp < 0xE000 && *sp >= 0xC000 {
            val = (self.ram[(*sp - 0xC000) as usize] as u16);
            *sp = *sp + 1;
            val = val + ((self.ram[(*sp - 0xC000) as usize] as u16) << 8);
            *sp = *sp + 1;
        }

        val
    }
}

#[test]
fn test_mmu_ram() {
    let mut derp = Mmu::new();
    derp.set_mem_u8(0xC000, 255);
    assert_eq!(derp.get_mem_u8(0xC000), 255);

    derp.set_mem_u8(0xC001, 10);
    assert_eq!(derp.get_mem_u8(0xC001), 10);

    derp.set_mem_u8(0xC002, 1);
    assert_eq!(derp.get_mem_u8(0xC002), 1);

    derp.set_mem_u16(0xC003, 0x1234);
    assert_eq!(derp.get_mem_u8(0xC003), 0x34);
    assert_eq!(derp.get_mem_u8(0xC004), 0x12);
    assert_eq!(derp.get_mem_u16(0xC003), 0x1234);
}

#[test]
fn test_stack_functions() {
    let mut derp = Mmu::new();
    let mut sp: u16 = 0xDFFF;

    derp.push_u16(&mut sp, 0x3210);
    derp.push_u8(&mut sp, 0x01);
    derp.push_u8(&mut sp, 0xF1);
    derp.push_u8(&mut sp, 0x0F);
    assert_eq!(derp.pop_u8(&mut sp), 0x0F);
    assert_eq!(derp.pop_u8(&mut sp), 0xF1);
    assert_eq!(derp.pop_u8(&mut sp), 0x01);
    assert_eq!(derp.pop_u16(&mut sp), 0x3210);

}
