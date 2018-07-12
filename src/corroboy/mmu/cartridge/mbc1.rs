// Copyright (c) 2018 Caleb Boylan

use super::Cartridge;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

// This code was made using the docs that describe the different MBCs at http://gbdev.gg8.se/wiki/articles/Memory_Bank_Controllers#MBC1_.28max_2MByte_ROM_and.2For_32KByte_RAM.29
pub struct Mbc1 {
    data: Vec<u8>,
    data_bank: usize,
    ram_enabled: bool,
    ram: Vec<u8>,
    ram_bank: usize,
    batt: bool,
    save_file: Option<File>,

    // 00h = ROM Banking Mode (up to 8KByte RAM, 2MByte ROM) (default)
    // 01h = RAM Banking Mode (up to 32KByte RAM, 512KByte ROM)
    banking_mode: u8,
}

impl Mbc1 {
    pub fn new(
        data: Vec<u8>,
        ram_size: usize,
        batt: bool,
        save_file_name: &Option<String>,
    ) -> Mbc1 {
        let mut ram = Vec::with_capacity(ram_size);

        let mut save_file = None;

        // If there is a battery Load the ram from the save file
        if batt == true {
            if *save_file_name == None {
                eprintln!("No save file provided and cartridge is battery backed, save games will not work");
            } else {
                let file_name = save_file_name.as_ref().unwrap();
                save_file = Some(
                    OpenOptions::new()
                        .create(true)
                        .read(true)
                        .write(true)
                        .truncate(false)
                        .append(false)
                        .open(file_name)
                        .expect("failed to open save file"),
                );
                if let Some(f) = save_file.as_mut() {
                    f.read_to_end(&mut ram)
                        .expect("Failed to read to end of save file");
                    f.seek(SeekFrom::Start(0))
                        .expect("Failed to seek to start of save file");
                }
            }
        }

        // If the ram hasnt been populated by a save file fill it with 0xFF
        if ram.len() == 0 {
            for _i in 0..ram_size {
                ram.push(0xFF);
            }
        }

        Mbc1 {
            data,
            data_bank: 1,
            ram,
            ram_bank: 0,
            batt,
            save_file,
            ram_enabled: false,
            banking_mode: 0,
        }
    }
}

impl Cartridge for Mbc1 {
    fn read(&self, location: usize) -> u8 {
        if location <= 0x3FFF {
            return self.data[location];
        } else if location >= 0x4000 && location <= 0x7FFF {
            return self.data[(location - 0x4000) + (0x4000 * self.data_bank)];
        } else if location >= 0xA000 && location <= 0xBFFF {
            return self.ram[(location - 0xA000) + (0x2000 * self.ram_bank)];
        }
        return 0xFF;
    }

    fn write(&mut self, location: usize, val: u8) {
        if location <= 0x1FFF {
            if val & 0b00001111 == 0x0A {
                self.ram_enabled = true;
            } else {
                self.ram_enabled = false;
            }
        } else if location >= 0x2000 && location <= 0x3FFF {
            if val & 0b00011111 == 0 {
                self.data_bank = (self.data_bank & 0b11100000) | 0x01;
            } else {
                self.data_bank = (self.data_bank & 0b11100000) | (val as usize & 0b00011111);
            }
        } else if location >= 0x4000 && location <= 0x5FFF {
            if self.banking_mode == 1 {
                self.ram_bank = val as usize & 0x03;
            } else if self.banking_mode == 0 {
                self.data_bank = (self.data_bank & 0b00011111) | ((val as usize & 0x03) << 5);
            }
        } else if location >= 0x6000 && location <= 0x7FFF {
            if val == 0 {
                self.banking_mode = 0;
                self.ram_bank &= 0;
            } else if val == 1 {
                self.banking_mode = 1;
                self.data_bank &= 0b00011111;
            }
        } else if location <= 0xBFFF && location >= 0xA000 {
            if self.ram.len() - (location - 0xA000) > 0 && self.ram_enabled == true {
                self.ram[(location - 0xA000) + 0x2000 * self.ram_bank] = val;
            }
        }
    }

    fn save_ram(&mut self) {
        if self.batt == true {
            if let Some(f) = self.save_file.as_mut() {
                f.seek(SeekFrom::Start(0))
                    .expect("Failed to seek to start of save file");
                f.write(&self.ram)
                    .expect("Failed to write save data to file");
                f.flush().expect("Failed to flush save data to file");
            }
        }
    }
}

#[test]
fn test_mbc1() {
    // Fill cart data with 0 - 255
    let mut data: Vec<u8> = Vec::with_capacity(1024 * 32);
    for j in 0..8 {
        for _i in 0..16384 {
            data.push(j as u8);
        }
    }

    // Create cart with 32768B of ram, no battery and no save file
    let mut cart = Mbc1::new(data, 32768, false, &None);

    // Test that the data reads correctly
    for i in 0..16384 {
        assert_eq!(cart.read(i), 0);
    }
    for j in 1..8 {
        cart.write(0x2000, j);
        for i in 0..16384 {
            assert_eq!(cart.read(0x4000 + i), j as u8);
        }
    }

    // Enable RAM
    cart.write(0, 0xA);
    // Set to RAM banking mode
    cart.write(0x6000, 1);

    // Test RAM writes and reading and banking
    for k in 0..0xFF as u8 {
        for j in 0..4 {
            // Change ram banks
            cart.write(0x4000, j);
            for i in 0..8192 as usize {
                // Assert that the ram writes and reads correctly
                cart.write(0xA000 + i, k);
                assert_eq!(cart.read(0xA000 + i), k);
            }
        }
    }
}
