pub mod no_mbc;

pub mod mbc1;

pub mod mbc2;

pub mod mbc5;

pub trait Cartridge {
    fn read(&self, location: usize) -> u8;
    fn write(&mut self, location: usize, val: u8);
    fn save_ram(&mut self);
}
