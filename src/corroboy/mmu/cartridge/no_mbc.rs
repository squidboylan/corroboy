// Copyright (c) 2018 Caleb Boylan

use super::Cartridge;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

pub struct NoMbc {
    data: Vec<u8>,
    ram: Vec<u8>,
    batt: bool,
    save_file: Option<File>,
}

impl NoMbc {
    pub fn new(
        data: Vec<u8>,
        ram_size: usize,
        batt: bool,
        save_file_name: &Option<String>,
    ) -> NoMbc {
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

        NoMbc {
            data,
            ram,
            batt,
            save_file,
        }
    }
}

impl Cartridge for NoMbc {
    fn read(&self, location: usize) -> u8 {
        if location <= 0x7FFF {
            return self.data[location];
        } else if location >= 0xA000 && location <= 0xBFFF {
            return self.ram[location - 0xA000];
        }
        // This should never be reached
        return 0xFF;
    }

    fn write(&mut self, location: usize, val: u8) {
        if location <= 0xBFFF && location >= 0xA000 {
            if self.ram.len() - (location - 0xA000) > 0 {
                self.ram[location - 0xA000] = val;
            } else {
                println!("Attempt to write to cart ram failed, not enough ram");
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
fn test_no_mbc() {
    // Fill cart data with 0 - 255
    let mut data: Vec<u8> = Vec::with_capacity(1024 * 32);
    for i in 0..(1024 * 32) {
        data.push((i % 256) as u8);
    }

    // Create cart with 8192B of ram, no battery and no save file
    let mut cart = NoMbc::new(data, 8192, false, &None);

    for i in 0..(1024 * 32) {
        // Assert that the data reads correctly
        assert_eq!(cart.read(i), (i % 256) as u8);
    }

    for i in 0..(1024 * 8) as usize {
        for j in 0..0xFF as u8 {
            // Assert that the ram writes and reads correctly
            cart.write(0xA000 + i, j);
            assert_eq!(cart.read(0xA000 + i), j);
        }
    }
}
