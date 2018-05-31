use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests;

pub struct Mmu {
    small_ram: [u8; 128],
    ram: [u8; 8192],
    cartridge: [u8; 32768],
    video_ram: [u8; 8192],
    oam: [u8; 160],
    bios: [u8; 256],
    io_ports: [u8; 0x4C],
    bios_mapped: u8,

    // Interrupts enabled reg
    ie: u8
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu{
            ram: [0; 8192],
            cartridge: [0xFF; 32768],
            small_ram: [0; 128],
            video_ram: [0; 8192],
            oam: [0; 160],
            bios: [0; 256],

            io_ports: [0; 0x4C],
            bios_mapped: 0,

            ie: 0,
        }
    }

    pub fn get_mem_u8(&self, location: usize) -> u8 {
        match location {
            0 ... 0xFF => { if self.bios_mapped == 0 {
                    return self.bios[location];
                }
                else {
                    return self.cartridge[location];
                }
            },
            0x100 ... 0x7FFF => return self.cartridge[location],
            0x8000 ... 0x9FFF => return self.video_ram[location - 0x8000],
            0xC000 ... 0xDFFF => return self.ram[location - 0xC000],
            0xE000 ... 0xFDFF => return self.ram[location - 0xE000],
            0xFE00 ... 0xFE9F => return self.oam[location - 0xFE00],
            0xFF00 ... 0xFF4B => return self.io_ports[location - 0xFF00],
            0xFF50 => return self.bios_mapped,
            0xFF80 ... 0xFFFE => return self.small_ram[location - 0xFF80],
            0xFFFF => return self.ie,
            _ => 0xFF,
        }
    }

    pub fn set_mem_u8(&mut self, location: usize, val: u8) {
        match location {
            0x0000 ... 0x7FFF => {},
            0x8000 ... 0x9FFF => self.video_ram[location - 0x8000] = val,
            0xC000 ... 0xDFFF => self.ram[location - 0xC000] = val,
            0xE000 ... 0xFDFF => self.ram[location - 0xE000] = val,
            0xFE00 ... 0xFE9F => self.oam[location - 0xFE00] = val,
            0xFEA0 ... 0xFEFF => {},
            0xFF00 ... 0xFF4B => self.io_ports[location - 0xFF00] = val,
            0xFF50 => self.bios_mapped = val,
            0xFE51 ... 0xFF7F => {},
            0xFF80 ... 0xFFFE => self.small_ram[location - 0xFF80] = val,
            0xFFFF => self.ie = val,
            _ => println!("set mem u8 that mmu cant handle, location: {:x}", location),
        }
    }

    // z80 is little endian in ram
    pub fn get_mem_u16(&self, location: usize) -> u16 {
        match location {
            0 ... 0xFF => { if self.bios_mapped == 0 {
                    return (self.bios[location] as u16) + ((self.bios[location + 1] as u16) << 8);
                }
                else {
                    return (self.cartridge[location] as u16) + ((self.cartridge[location + 1] as u16) << 8);
                }
            },
            0x100 ... 0x7FFF => return (self.cartridge[location] as u16) + ((self.cartridge[location + 1] as u16) << 8),
            0xC000 ... 0xDFFF => return (self.ram[location - 0xC000] as u16) + ((self.ram[location + 1 - 0xC000] as u16) << 8),
            0xE000 ... 0xFDFF => return (self.ram[location - 0xE000] as u16) + ((self.ram[location + 1 - 0xE000] as u16) << 8),
            0x8000 ... 0x9FFF => return (self.video_ram[location - 0x8000] as u16) + ((self.video_ram[location + 1 - 0x8000] as u16) << 8),
            0xFE00 ... 0xFE9F => return (self.oam[location - 0xFE00] as u16) + ((self.oam[location + 1 - 0xFE00] as u16) << 8),
            0xFF00 ... 0xFF4B => return (self.io_ports[location - 0xFF00] as u16) + ((self.io_ports[location - 0xFF00] as u16) << 8),
            0xFF80 ... 0xFFFE => return (self.small_ram[location - 0xFF80] as u16) + ((self.small_ram[location + 1 - 0xFF80] as u16) << 8),
            _ => 0
        }
    }

    // z80 is little endian in ram
    pub fn set_mem_u16(&mut self, location: usize, val: u16) {
        match location {
            0 ... 0x7FFF => { self.cartridge[location] = val as u8; self.cartridge[location + 1] = (val >> 8) as u8; },
            0xC000 ... 0xDFFF => { self.ram[location - 0xC000] = val as u8; self.ram[location + 1 - 0xC000] = (val >> 8) as u8; },
            0xE000 ... 0xFDFF => { self.ram[location - 0xE000] = val as u8; self.ram[location + 1 - 0xE000] = (val >> 8) as u8; },
            0x8000 ... 0x9FFF => { self.video_ram[location - 0x8000] = val as u8; self.video_ram[location + 1 - 0x8000] = (val >> 8) as u8; },
            0xFE00 ... 0xFE9F => { self.oam[location - 0xFE00] = val as u8; self.oam[location + 1 - 0xFE00] = (val >> 8) as u8; },
            0xFF00 ... 0xFF4B => { self.io_ports[location - 0xFF00] = val as u8; self.io_ports[location - 0xFF00] = (val >> 8) as u8; },
            0xFF80 ... 0xFFFE => { self.small_ram[location - 0xFF80] = val as u8; self.small_ram[location + 1 - 0xFF80] = (val >> 8) as u8; },
            _ => println!("set mem u16 that mmu cant handle, location: {:x}", location),
        }
    }


    // bios goes from 0x0000 -> 0x00FF, overlays onto cartridge space
    pub fn load_bios(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).expect("rom not found");

        let mut contents: Vec<u8> = vec![];
        f.read_to_end(&mut contents).unwrap();
        for (index, i) in contents.iter().enumerate() {
            self.bios[index] = *i;
        }
    }

    // Cartridge goes from 0x0000 -> 0x7FFF
    pub fn load_rom(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).expect("rom not found");

        let mut contents: Vec<u8> = vec![];
        f.read_to_end(&mut contents).unwrap();
        for (index, i) in contents.iter().enumerate() {
            self.cartridge[index] = *i;
        }
    }

    pub fn push_u16(&mut self, sp: &mut u16, val: u16) {
        match *sp {
            0xC000 ... 0xDFFF => { *sp = *sp - 1; self.ram[(*sp - 0xC000) as usize] = (val >> 8) as u8; *sp = *sp - 1; self.ram[(*sp - 0xC000) as usize] = val as u8; },
            0x8000 ... 0x9FFF => { *sp = *sp - 1; self.video_ram[(*sp - 0x8000) as usize] = (val >> 8) as u8; *sp = *sp - 1; self.video_ram[(*sp - 0x8000) as usize] = val as u8; },
            0xFF80 ... 0xFFFE => { *sp = *sp - 1; self.small_ram[(*sp - 0xFF80) as usize] = (val >> 8) as u8; *sp = *sp - 1; self.small_ram[(*sp - 0xFF80) as usize] = val as u8; },
            _ => println!("push to mem u16 that mmu cant handle, location: {:x}", *sp),
        }
    }

    pub fn pop_u16(&self, sp: &mut u16) -> u16{
        let mut val: u16 = 0;
        match *sp {
            0xC000 ... 0xDFFF => { val = self.ram[(*sp - 0xC000) as usize] as u16; *sp = *sp + 1; val = val + ((self.ram[(*sp - 0xC000) as usize] as u16) << 8); *sp = *sp + 1; },
            0x8000 ... 0x9FFF => { val = self.video_ram[(*sp - 0x8000) as usize] as u16; *sp = *sp + 1; val = val + ((self.video_ram[(*sp - 0x8000) as usize] as u16) << 8); *sp = *sp + 1; },
            0xFF80 ... 0xFFFE => { val = self.small_ram[(*sp - 0xFF80) as usize] as u16; *sp = *sp + 1; val = val + ((self.small_ram[(*sp - 0xFF80) as usize] as u16) << 8); *sp = *sp + 1; },
            _ => println!("pop mem u16 that mmu cant handle, location: {:x}", *sp),
        }
        val
    }
}
