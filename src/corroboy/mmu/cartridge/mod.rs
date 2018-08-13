// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

pub mod no_mbc;

pub mod mbc1;

pub mod mbc2;

pub mod mbc3;

pub mod mbc5;

pub trait Cartridge {
    fn read(&self, location: usize) -> u8;
    fn write(&mut self, location: usize, val: u8);
    fn save_ram(&mut self);
    // By default there is no timer so do nothing
    fn update_timer(&mut self, _ticks: usize) {}
}
