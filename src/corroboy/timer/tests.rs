// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::Timer;

use super::super::mmu::Mmu;

#[test]
pub fn timer_div() {
    let mut timer = Timer::new();
    let mut mem = Mmu::new(None);

    for i in 0..10000 {
        assert_eq!((i / 64) as u8, mem.get_mem_u8(0xFF04));
        assert_eq!(0, mem.get_mem_u8(0xFF05));
        timer.update(&mut mem);
    }
}

#[test]
pub fn timer_00() {
    let mut timer = Timer::new();
    let mut mem = Mmu::new(None);

    mem.set_mem_u8(0xFF07, 0b00000100);

    for i in 0..(256 * 258) {
        assert_eq!(((i / 256) % 256) as u8, mem.get_mem_u8(0xFF05));
        timer.update(&mut mem);
    }

    assert_eq!(mem.get_mem_u8(0xFF0F) & 0b00000100, 0b00000100);
}

#[test]
pub fn timer_01() {
    let mut timer = Timer::new();
    let mut mem = Mmu::new(None);

    mem.set_mem_u8(0xFF07, 0b00000101);

    for i in 0..(4 * 258) {
        assert_eq!(((i / 4) % 256) as u8, mem.get_mem_u8(0xFF05));
        timer.update(&mut mem);
    }

    assert_eq!(mem.get_mem_u8(0xFF0F) & 0b00000100, 0b00000100);
}

#[test]
pub fn timer_10() {
    let mut timer = Timer::new();
    let mut mem = Mmu::new(None);

    mem.set_mem_u8(0xFF07, 0b00000110);

    for i in 0..(16 * 258) {
        assert_eq!(((i / 16) % 256) as u8, mem.get_mem_u8(0xFF05));
        timer.update(&mut mem);
    }

    assert_eq!(mem.get_mem_u8(0xFF0F) & 0b00000100, 0b00000100);
}

#[test]
pub fn timer_11() {
    let mut timer = Timer::new();
    let mut mem = Mmu::new(None);

    mem.set_mem_u8(0xFF07, 0b00000111);

    for i in 0..(64 * 258) {
        assert_eq!(((i / 64) % 256) as u8, mem.get_mem_u8(0xFF05));
        timer.update(&mut mem);
    }

    assert_eq!(mem.get_mem_u8(0xFF0F) & 0b00000100, 0b00000100);
}
