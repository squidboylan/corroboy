// Copyright (c) 2018 Caleb Boylan

use super::Cartridge;
use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::time::{SystemTime, UNIX_EPOCH};

// This code was made using the docs that describe the different MBCs at
// http://gbdev.gg8.se/wiki/articles/Memory_Bank_Controllers#MBC3_.28max_2MByte_ROM_and.2For_32KByte_RAM_and_Timer.29
pub struct Mbc3 {
    data: Vec<u8>,
    data_bank: usize,
    ram_enabled: bool,
    ram: Vec<u8>,
    ram_bank: usize,
    ram_dirty: bool,
    batt: bool,
    save_file: Option<File>,
    timer: Timer,
    timer_batt: bool,
}

impl Mbc3 {
    pub fn new(
        data: Vec<u8>,
        ram_size: usize,
        batt: bool,
        save_file_name: &Option<String>,
        timer_batt: bool,
    ) -> Mbc3 {
        let mut ram = Vec::with_capacity(ram_size);

        let mut save_file = None;

        // Create the timer
        let mut timer = Timer::new();

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

                    let ram_len = ram.len();
                    if timer_batt == true && ram_len != 0 {
                        let save_time_buff = ram.split_off(ram_len - 8);
                        let save_time = BigEndian::read_u64(&save_time_buff);
                        let ram_len = ram.len();
                        let timer_data = ram.split_off(ram_len - 5);
                        timer.load(timer_data, save_time);
                    }

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

        Mbc3 {
            data,
            data_bank: 1,
            ram,
            ram_bank: 0,
            batt,
            save_file,
            ram_enabled: false,
            ram_dirty: false,
            timer,
            timer_batt,
        }
    }
}

impl Cartridge for Mbc3 {
    fn read(&self, location: usize) -> u8 {
        if location <= 0x3FFF {
            return self.data[location];
        } else if location >= 0x4000 && location <= 0x7FFF {
            return self.data[(location - 0x4000) + (0x4000 * self.data_bank)];
        } else if location >= 0xA000 && location <= 0xBFFF && self.ram_enabled {
            if self.ram_bank <= 3 {
                return self.ram[(location - 0xA000) + (0x2000 * self.ram_bank)];
            } else {
                return self.timer.get_val(self.ram_bank);
            }
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
            if val == 0 {
                self.data_bank = 1;
            } else {
                self.data_bank = (val & 0x7F) as usize;
            }
        } else if location >= 0x4000 && location <= 0x5FFF {
            self.ram_bank = (val & 0x0F) as usize;
        } else if location >= 0x6000 && location <= 0x7FFF {
            // If val is 1, set the latch to true, otherwise false
            self.timer.update_latch(val == 1);
        } else if location >= 0xA000 && location <= 0xBFFF && self.ram_enabled {
            if self.ram_bank <= 3 {
                self.ram[(location - 0xA000) + 0x2000 * self.ram_bank] = val;
                self.ram_dirty = true;
            } else {
                self.timer.set_val(self.ram_bank, val);
            }
        }
    }

    fn save_ram(&mut self) {
        if self.batt == true && self.ram_dirty == true {
            self.ram_dirty = false;
            if let Some(f) = self.save_file.as_mut() {
                f.seek(SeekFrom::Start(0))
                    .expect("Failed to seek to start of save file");
                f.write(&self.ram)
                    .expect("Failed to write save data to file");

                if self.timer_batt == true {
                    f.write(&self.timer.get_data())
                        .expect("Failed to write save data to file");
                    let time_from_epoch = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Error getting duration since epoch")
                        .as_secs();
                    let mut buf: [u8; 8] = [0; 8];
                    BigEndian::write_u64(&mut buf, time_from_epoch);
                    f.write(&buf).expect("Failed to write save data to file");
                }

                f.flush().expect("Failed to flush save data to file");
            }
        }
    }

    fn update_timer(&mut self, ticks: usize) {
        self.timer.update_timer(ticks);
    }
}

struct Timer {
    // When ticks gets to 1048576 (cpu ticks/s) it is reset and seconds is increased
    ticks: usize,
    seconds: u8,
    minutes: u8,
    hours: u8,
    days: u8,

    // See hardware documentation for what this register is for
    reg5: u8,

    // Whether or not the timer should update
    latch: bool,

    latch_seconds: u8,
    latch_minutes: u8,
    latch_hours: u8,
    latch_days: u8,

    // See hardware documentation for what this register is for
    latch_reg5: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            ticks: 0,
            seconds: 0,
            minutes: 0,
            hours: 0,
            days: 0,
            reg5: 0,
            latch: false,
            latch_seconds: 0,
            latch_minutes: 0,
            latch_hours: 0,
            latch_days: 0,
            latch_reg5: 0,
        }
    }

    pub fn update_timer(&mut self, ticks: usize) {
        if self.reg5 & 0b0100_0000 != 0 {
            return;
        }

        self.ticks += ticks;
        if self.ticks >= 1048576 {
            self.ticks %= 1048576;
            self.inc_seconds();
        }
    }

    fn inc_seconds(&mut self) {
        self.seconds += 1;
        if self.seconds == 60 {
            self.seconds = 0;
            self.inc_minutes();
        }
        // println!("s: {}", self.seconds);
        // println!("m: {}", self.minutes);
        // println!("h: {}", self.hours);
        // println!("d: {}", self.days);
    }

    fn inc_minutes(&mut self) {
        self.minutes += 1;
        if self.minutes == 60 {
            self.minutes = 0;
            self.inc_hours();
        }
    }

    fn inc_hours(&mut self) {
        self.hours += 1;
        if self.hours == 24 {
            self.hours = 0;
            self.inc_days();
        }
    }

    fn inc_days(&mut self) {
        if self.days == 255 {
            self.days = 0;
            if self.reg5 & 0b0000_0001 == 1 {
                // Unset the 9th bit of the day counter and set the overflow bit
                self.reg5 &= 0b1111_1110;
                self.reg5 |= 0b1000_0000;
            } else {
                // set the 9th bit of the day counter
                self.reg5 |= 0b0000_0001;
            }
        } else {
            self.days += 1;
        }
    }

    pub fn update_latch(&mut self, latch: bool) {
        if self.latch == false && latch == true {
            self.latch_seconds = self.seconds;
            self.latch_minutes = self.minutes;
            self.latch_hours = self.hours;
            self.latch_days = self.days;
            self.latch_reg5 = self.reg5;
        }
        self.latch = latch;
    }

    pub fn get_val(&self, reg_num: usize) -> u8 {
        let ret_val;
        if reg_num == 0x8 {
            ret_val = self.latch_seconds;
        } else if reg_num == 0x9 {
            ret_val = self.latch_minutes;
        } else if reg_num == 0xA {
            ret_val = self.latch_hours;
        } else if reg_num == 0xB {
            ret_val = self.latch_days;
        } else if reg_num == 0xC {
            ret_val = self.latch_reg5;
        } else {
            ret_val = 0xFF;
        }

        ret_val
    }

    pub fn set_val(&mut self, reg_num: usize, val: u8) {
        if reg_num == 0x8 {
            self.seconds = val;
        } else if reg_num == 0x9 {
            self.minutes = val;
        } else if reg_num == 0xA {
            self.hours = val;
        } else if reg_num == 0xB {
            self.days = val;
        } else if reg_num == 0xC {
            self.reg5 = val;
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::with_capacity(5);
        data.push(self.seconds);
        data.push(self.minutes);
        data.push(self.hours);
        data.push(self.days);
        data.push(self.reg5);

        data
    }

    pub fn load(&mut self, data: Vec<u8>, save_time: u64) {
        assert_eq!(data.len(), 5);
        self.seconds = data[0];
        self.minutes = data[1];
        self.hours = data[2];
        self.days = data[3];
        self.reg5 = data[4];

        let time_from_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error getting duration since epoch")
            .as_secs();
        println!("save_time: {}", save_time);
        println!("time_from_epoch: {}", time_from_epoch);
        let seconds_passed_since_save = time_from_epoch - save_time;
        println!("{}", seconds_passed_since_save);
        for _i in 0..seconds_passed_since_save {
            self.update_timer(1048576);
        }
    }
}

#[test]
fn test_mbc3() {
    // Fill cart data with 0 - 255
    let mut data: Vec<u8> = Vec::with_capacity(1024 * 32);
    for j in 0..8 {
        for _i in 0..16384 {
            data.push(j as u8);
        }
    }

    // Create cart with 32768B of ram, no battery and no save file
    let mut cart = Mbc3::new(data, 32768, false, &None, false);

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

#[test]
fn test_mbc3_timer() {
    // Fill cart data with 0 - 255
    let mut data: Vec<u8> = Vec::with_capacity(1024 * 32);
    for j in 0..8 {
        for _i in 0..16384 {
            data.push(j as u8);
        }
    }

    // Create cart no ram, no battery and no save file
    let mut cart = Mbc3::new(data, 0, false, &None, false);

    // Enable RAM
    cart.write(0, 0xA);

    cart.write(0x4000, 0xC);
    cart.write(0xA000, 0b0100_0000);

    // Test that timer is not updated when the bit to disable it is set
    for _ in 0..(86400u64 * 513) {
        // Latch the updated time
        cart.write(0x6000, 0);
        cart.write(0x6000, 1);

        cart.write(0x4000, 0x8);
        assert_eq!(cart.read(0xA000), 0);

        cart.write(0x4000, 0x9);
        assert_eq!(cart.read(0xA000), 0);

        cart.write(0x4000, 0xA);
        assert_eq!(cart.read(0xA000), 0);

        cart.write(0x4000, 0xB);
        assert_eq!(cart.read(0xA000), 0);

        cart.write(0x4000, 0xC);
        let reg5 = cart.read(0xA000);

        assert_eq!(reg5 & 0b0000_0001, 0);
        assert_eq!(reg5 & 0b1000_0000, 0);

        cart.update_timer(1048576);
    }

    // Enable the timer
    cart.write(0x4000, 0xC);
    cart.write(0xA000, 0);

    // Test that the timer counts when it is enabled
    for i in 0..(86400u64 * 513) {
        let sec = i;
        let minute = sec / 60;
        let hour = minute / 60;
        let day = hour / 24;
        let day_bit_8 = day/256;
        let day_c = day/512;

        // Latch the updated time
        cart.write(0x6000, 0);
        cart.write(0x6000, 1);

        cart.write(0x4000, 0x8);
        assert_eq!(cart.read(0xA000), (sec % 60) as u8);

        cart.write(0x4000, 0x9);
        assert_eq!(cart.read(0xA000), (minute % 60) as u8);

        cart.write(0x4000, 0xA);
        assert_eq!(cart.read(0xA000), (hour % 24) as u8);

        cart.write(0x4000, 0xB);
        assert_eq!(cart.read(0xA000), day as u8);

        cart.write(0x4000, 0xC);
        let reg5 = cart.read(0xA000);

        assert_eq!(reg5 & 0b0000_0001, (day_bit_8 % 2) as u8);
        assert_eq!(reg5 & 0b1000_0000, (day_c as u8) << 7);

        cart.update_timer(1048576);
    }
}
