// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::corroboy::mmu::Mmu;

// LD a,r2

pub fn ld_reg(val: u8, reg: &mut u8) {
    *reg = val;
}

pub fn ld_reg_16bit(val: u16, reg: &mut u16) {
    *reg = val;
}

pub fn ld_mem(loc: u16, mem: &mut Mmu, val: u8) {
    mem.set_mem_u8(loc as usize, val);
}

pub fn ld_mem_16bit(loc: u16, mem: &mut Mmu, val: u16) {
    mem.set_mem_u16(loc as usize, val);
}

// LDI A, (HL)
// Load (HL) into A and then increment HL

pub fn ldi_a_hl(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    *a = mem.get_mem_u8(*hl as usize);
    *hl += 1;
}

// LDI (HL), A
// Load A into (HL) and then increment HL

pub fn ldi_hl_a(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    mem.set_mem_u8(*hl as usize, *a);
    *hl += 1;
}

// LDD A, (HL)
// Load (HL) into A and then decrement HL

pub fn ldd_a_hl(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    *a = mem.get_mem_u8(*hl as usize);
    *hl -= 1;
}

// LDD (HL), A
// Load A into (HL) and then decrement HL

pub fn ldd_hl_a(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    mem.set_mem_u8(*hl as usize, *a);
    *hl -= 1;
}

// LDHL SP,n

pub fn ldhl_sp_n(val: u8, sp: u16, hl: &mut u16, flags: &mut u8) {
    let val_i16 = val as i8 as i16;

    *flags = 0;
    *hl = (sp as i16 + val_i16) as u16;

    if (val_i16 as u16 ^ sp ^ *hl) & 0x10 != 0 {
        *flags |= 0b00100000;
    }
    if (val_i16 as u16 ^ sp ^ *hl) & 0x100 != 0 {
        *flags |= 0b00010000;
    }
}
