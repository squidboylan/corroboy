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
    pub fn get_mem_u16(&self, location: usize) -> u16 {
        if location < 0x4000 {
            return ((self.cartridge[location] as u16) << 8) + (self.cartridge[location + 1] as u16);
        }
        if location < 0xE000 && location >= 0xC000 {
            return ((self.ram[location - 0xC000] as u16) << 8) + (self.ram[location + 1 - 0xC000] as u16);
        }

        0
    }

    pub fn set_mem_u16(&mut self, location: usize, val: u16) {
        if location < 0x4000 {
            self.cartridge[location] = (val >> 8) as u8;
            self.cartridge[location + 1] = val as u8;
        }
        if location < 0xE000 && location >= 0xC000 {
            self.ram[location - 0xC000] = (val >> 8) as u8;
            self.ram[location + 1 - 0xC000] = val as u8;
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
    assert_eq!(derp.get_mem_u8(0xC003), 0x12);
    assert_eq!(derp.get_mem_u8(0xC004), 0x34);
    assert_eq!(derp.get_mem_u16(0xC003), 0x1234);
}
