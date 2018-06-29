use std::fs::File;
use std::io::prelude::*;

mod cartridge;

#[cfg(test)]
mod tests;

pub struct Mmu {
    small_ram: [u8; 128],
    ram: [u8; 8192],
    cart: Option<Box<cartridge::Cartridge>>,
    video_ram: [u8; 8192],
    oam: [u8; 160],
    bios: [u8; 256],
    io_registers: [u8; 0x4D],
    bios_mapped: u8,

    // Interrupts enabled reg
    ie: u8,

    save_file: Option<String>,
}

impl Mmu {
    pub fn new(save_file: Option<String>) -> Mmu {
        Mmu{
            ram: [0; 8192],
            cart: None,
            small_ram: [0; 128],
            video_ram: [0; 8192],
            oam: [0; 160],
            bios: [0; 256],

            io_registers: [0; 0x4D],
            bios_mapped: 0,

            ie: 0,
            save_file,
        }
    }

    pub fn skip_bios(&mut self) {
        self.bios_mapped = 1;
        // Zero out vram
        for i in 0x8000..0xA000 {
            self.set_mem_u8(i, 0);
        }

        // Set io register values
        self.io_registers[0x05] = 0;
        self.io_registers[0x06] = 0;
        self.io_registers[0x07] = 0;
        self.io_registers[0x10] = 0x80;
        self.io_registers[0x11] = 0xBF;
        self.io_registers[0x12] = 0xF3;
        self.io_registers[0x14] = 0xBF;
        self.io_registers[0x16] = 0x3F;
        self.io_registers[0x17] = 0;
        self.io_registers[0x19] = 0xBF;
        self.io_registers[0x1A] = 0x7F;
        self.io_registers[0x1B] = 0xFF;
        self.io_registers[0x1C] = 0x9F;
        self.io_registers[0x1E] = 0xBF;
        self.io_registers[0x20] = 0xFF;
        self.io_registers[0x21] = 0;
        self.io_registers[0x22] = 0;
        self.io_registers[0x23] = 0xBF;
        self.io_registers[0x24] = 0x77;
        self.io_registers[0x25] = 0xF3;
        self.io_registers[0x26] = 0xF1;
        self.io_registers[0x40] = 0x91;
        self.io_registers[0x42] = 0;
        self.io_registers[0x43] = 0;
        self.io_registers[0x45] = 0;
        self.io_registers[0x47] = 0xFC;
        self.io_registers[0x48] = 0xFF;
        self.io_registers[0x49] = 0xFF;
        self.io_registers[0x4A] = 0;
        self.io_registers[0x4B] = 0;
        self.io_registers[0x4C] = 0;

        self.set_mem_u8(0xFFFF, 0);
    }

    pub fn get_mem_u8(&self, location: usize) -> u8 {
        match location {
            0 ... 0xFF => { if self.bios_mapped == 0 {
                    return self.bios[location];
                }
                else {
                    return self.read_cart(location).unwrap();
                }
            },
            0x100 ... 0x7FFF => return self.read_cart(location).unwrap(),
            0x8000 ... 0x9FFF => return self.video_ram[location - 0x8000],
            0xA000 ... 0xBFFF => return self.read_cart(location).unwrap(),
            0xC000 ... 0xDFFF => return self.ram[location - 0xC000],
            0xE000 ... 0xFDFF => return self.ram[location - 0xE000],
            0xFE00 ... 0xFE9F => return self.oam[location - 0xFE00],
            0xFF00 ... 0xFF4C => return self.io_registers[location - 0xFF00],
            0xFF50 => return self.bios_mapped,
            0xFF80 ... 0xFFFE => return self.small_ram[location - 0xFF80],
            0xFFFF => return self.ie,
            _ => 0xFF,
        }
    }

    pub fn set_mem_u8(&mut self, location: usize, val: u8) {
        match location {
            0x0000 ... 0x7FFF => self.write_cart(location, val),
            0x8000 ... 0x9FFF => self.video_ram[location - 0x8000] = val,
            0xA000 ... 0xBFFF => self.write_cart(location, val),
            0xC000 ... 0xDFFF => self.ram[location - 0xC000] = val,
            0xE000 ... 0xFDFF => self.ram[location - 0xE000] = val,
            0xFE00 ... 0xFE9F => self.oam[location - 0xFE00] = val,
            0xFEA0 ... 0xFEFF => {},
            0xFF00 ... 0xFF45 => self.io_registers[location - 0xFF00] = val,
            0xFF46 => self.dma_transfer(val),
            0xFF47 ... 0xFF4C => self.io_registers[location - 0xFF00] = val,
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
                    return (self.read_cart(location).unwrap() as u16) + ((self.read_cart(location + 1).unwrap() as u16) << 8);
                }
            },
            0x100 ... 0x7FFF => return (self.read_cart(location).unwrap() as u16) + ((self.read_cart(location + 1).unwrap() as u16) << 8),
            0xC000 ... 0xDFFF => return (self.ram[location - 0xC000] as u16) + ((self.ram[location + 1 - 0xC000] as u16) << 8),
            0xE000 ... 0xFDFF => return (self.ram[location - 0xE000] as u16) + ((self.ram[location + 1 - 0xE000] as u16) << 8),
            0x8000 ... 0x9FFF => return (self.video_ram[location - 0x8000] as u16) + ((self.video_ram[location + 1 - 0x8000] as u16) << 8),
            0xFE00 ... 0xFE9F => return (self.oam[location - 0xFE00] as u16) + ((self.oam[location + 1 - 0xFE00] as u16) << 8),
            0xFF00 ... 0xFF4B => return (self.io_registers[location - 0xFF00] as u16) + ((self.io_registers[location - 0xFF00] as u16) << 8),
            0xFF80 ... 0xFFFE => return (self.small_ram[location - 0xFF80] as u16) + ((self.small_ram[location + 1 - 0xFF80] as u16) << 8),
            _ => 0
        }
    }

    // z80 is little endian in ram
    pub fn set_mem_u16(&mut self, location: usize, val: u16) {
        match location {
            0 ... 0x7FFF => { self.write_cart(location, val as u8); self.write_cart(location + 1, (val >> 8) as u8); },
            0xC000 ... 0xDFFF => { self.ram[location - 0xC000] = val as u8; self.ram[location + 1 - 0xC000] = (val >> 8) as u8; },
            0xE000 ... 0xFDFF => { self.ram[location - 0xE000] = val as u8; self.ram[location + 1 - 0xE000] = (val >> 8) as u8; },
            0x8000 ... 0x9FFF => { self.video_ram[location - 0x8000] = val as u8; self.video_ram[location + 1 - 0x8000] = (val >> 8) as u8; },
            0xFE00 ... 0xFE9F => { self.oam[location - 0xFE00] = val as u8; self.oam[location + 1 - 0xFE00] = (val >> 8) as u8; },
            0xFF00 ... 0xFF4B => { self.io_registers[location - 0xFF00] = val as u8; self.io_registers[location - 0xFF00] = (val >> 8) as u8; },
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
        let cart_type = contents[0x147];
        println!("cart-type: {:x}", cart_type);
        println!("cart-size: {}B", contents.len());

        if cart_type == 0 {
            self.cart = Some(Box::new(cartridge::no_mbc::NoMbc::new(contents, 0, false, &self.save_file)));
        }
        else if cart_type == 1 {
            let mut ram_size = 0;
            if contents[0x149] == 1 {
                ram_size = 2048;
            }
            else if contents[0x149] == 2 {
                ram_size = 8192;
            }

            else if contents[0x149] == 3 {
                ram_size = 8192 * 4;
            }

            else if contents[0x149] == 4 {
                ram_size = 8192 * 8;
            }

            else if contents[0x149] == 5 {
                ram_size = 8192 * 16;
            }

            self.cart = Some(Box::new(cartridge::mbc1::Mbc1::new(contents, ram_size, false, &self.save_file)));
        }
        else if cart_type == 3 {
            let mut ram_size = 0;
            if contents[0x149] == 1 {
                ram_size = 2048;
            }
            else if contents[0x149] == 2 {
                ram_size = 8192;
            }

            else if contents[0x149] == 3 {
                ram_size = 8192 * 4;
            }

            else if contents[0x149] == 4 {
                ram_size = 8192 * 8;
            }

            else if contents[0x149] == 5 {
                ram_size = 8192 * 16;
            }

            self.cart = Some(Box::new(cartridge::mbc1::Mbc1::new(contents, ram_size, true, &self.save_file)));
        }
        else if cart_type == 8 || cart_type == 9 {
            let mut ram_size = 0;
            if contents[0x149] == 1 {
                ram_size = 2048;
            }
            else if contents[0x149] == 2 {
                ram_size = 8192;
            }

            if cart_type == 8 {
                self.cart = Some(Box::new(cartridge::no_mbc::NoMbc::new(contents, ram_size, false, &self.save_file)));
            }
            else {
                self.cart = Some(Box::new(cartridge::no_mbc::NoMbc::new(contents, ram_size, true, &self.save_file)));
            }
        }
        else {
            panic!("Cartridge uses unsupported MBC, code: {}", cart_type);
        }

    }

    pub fn push_u16(&mut self, sp: &mut u16, val: u16) {
        *sp = *sp - 1;
        self.set_mem_u8(*sp as usize, (val >> 8) as u8);
        *sp = *sp - 1;
        self.set_mem_u8(*sp as usize, val as u8);
    }

    pub fn pop_u16(&self, sp: &mut u16) -> u16{
        let mut val = self.get_mem_u8(*sp as usize) as u16;
        *sp = *sp + 1;
        val += (self.get_mem_u8(*sp as usize) as u16) << 8;
        *sp = *sp + 1;
        val
    }

    // DMA transfers data from XX00 - XX9F to FE00 - FE9F, should take 160us, but will try make it
    // immediate and see what breaks.
    pub fn dma_transfer(&mut self, ff46: u8) {
        //println!("dma_transfer");
        let start = ((ff46 & 0xF1) as usize) << 8;
        let mut i: usize = 0;
        while i < 0xA0 {
            let val = self.get_mem_u8(start + i);
            self.set_mem_u8(0xFE00 + i, val);
            i += 1;
        }
    }


    // These are functions to speed up common location accesses so they dont have to go through the
    // expensive match statement

    pub fn get_vram(&self, location: usize) -> u8 {
        return self.video_ram[location - 0x8000];
    }

    pub fn get_io_register(&self, location: usize) -> u8 {
        return self.io_registers[location - 0xFF00];
    }

    pub fn set_io_register(&mut self, location: usize, val: u8) {
        self.io_registers[location - 0xFF00] = val;
    }

    pub fn get_interrupts(&self) -> u8 {
        return self.io_registers[0xF];
    }

    pub fn set_interrupts(&mut self, num: u8) {
        self.io_registers[0xF] = num;
    }

    pub fn get_interrupts_enabled(&self) -> u8 {
        return self.ie;
    }

    pub fn get_div(&self) -> u8 {
        return self.io_registers[4];
    }

    pub fn set_div(&mut self, num: u8) {
        self.io_registers[4] = num;
    }

    pub fn get_tima(&self) -> u8 {
        return self.io_registers[5];
    }

    pub fn set_tima(&mut self, num: u8) {
        self.io_registers[5] = num;
    }

    pub fn get_tma(&self) -> u8 {
        return self.io_registers[6];
    }

    pub fn get_tac(&self) -> u8 {
        return self.io_registers[7];
    }

    pub fn read_cart(&self, location: usize) -> Option<u8> {
        if let Some(v) = self.cart.as_ref() {
            return Some(v.read(location));
        }
        None
    }

    pub fn write_cart(&mut self, location: usize, val: u8) {
        if let Some(v) = self.cart.as_mut() {
            v.write(location, val);
        }
    }

    pub fn save_cart_ram(&mut self) {
        if let Some(v) = self.cart.as_mut() {
            v.save_ram();
        }
    }
}
