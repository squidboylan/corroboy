use super::Cartridge;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

// This code was made using the docs that describe the different MBCs at http://gbdev.gg8.se/wiki/articles/Memory_Bank_Controllers#MBC1_.28max_2MByte_ROM_and.2For_32KByte_RAM.29
pub struct Mbc2 {
    data: Vec<u8>,
    data_bank: usize,
    ram_enabled: bool,
    ram: Vec<u8>,
    batt: bool,
    save_file: Option<File>,
}

impl Mbc2 {
    pub fn new(data: Vec<u8>, batt: bool, save_file_name: &Option<String>) -> Mbc2 {
        let mut ram = Vec::with_capacity(512);

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

        // If the ram hasnt been populated by a save file fill it with 0x0F
        if ram.len() == 0 {
            for _i in 0..512 {
                ram.push(0x0F);
            }
        }

        Mbc2 {
            data,
            data_bank: 1,
            ram,
            batt,
            save_file,
            ram_enabled: false,
        }
    }
}

impl Cartridge for Mbc2 {
    fn read(&self, location: usize) -> u8 {
        if location <= 0x3FFF {
            return self.data[location];
        } else if location >= 0x4000 && location <= 0x7FFF {
            return self.data[(location - 0x4000) + (0x4000 * self.data_bank)];
        } else if location >= 0xA000 && location <= 0xBFFF {
            return self.ram[location - 0xA000];
        }
        return 0xFF;
    }

    fn write(&mut self, location: usize, val: u8) {
        if location <= 0x1FFF && location & 0x10 == 0 {
            if val & 0b00001111 == 0x0A {
                self.ram_enabled = true;
            } else {
                self.ram_enabled = false;
            }
        } else if location >= 0x2000 && location <= 0x3FFF {
            if val & 0x0F == 0 {
                self.data_bank = 1;
            } else {
                self.data_bank = (val & 0x0F) as usize;
            }
        } else if location <= 0xA1FF && location >= 0xA000 && self.ram_enabled == true {
            self.ram[(location - 0xA000)] = val & 0x0F;
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
fn test_mbc2() {
    // Fill cart data with 0 - 16 based on bank
    let mut data: Vec<u8> = Vec::with_capacity(1024 * 256);
    for j in 0..16 {
        for _i in 0..16384 {
            data.push(j as u8);
        }
    }

    // Create cart with no battery and no save file
    let mut cart = Mbc2::new(data, false, &None);

    // Test that the data reads correctly
    for i in 0..16384 {
        assert_eq!(cart.read(i), 0);
    }
    for j in 1..16 {
        cart.write(0x2000, j);
        for i in 0..16384 {
            assert_eq!(cart.read(0x4000 + i), j as u8);
        }
    }

    // Enable RAM
    cart.write(0, 0xA);

    // Test RAM writes and reading and banking
    for k in 0..0x0F as u8 {
        for i in 0..512 as usize {
            // Assert that the ram writes and reads correctly
            cart.write(0xA000 + i, k);
            assert_eq!(cart.read(0xA000 + i), k);
        }
    }

    for k in 0x0F..0xFF as u8 {
        for i in 0..512 as usize {
            // Assert that the ram writes only the lower 4 bits
            cart.write(0xA000 + i, k);
            assert_eq!(cart.read(0xA000 + i), k & 0x0F);
        }
    }
}
