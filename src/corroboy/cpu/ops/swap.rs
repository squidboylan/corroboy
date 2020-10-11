// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::corroboy::mmu::Mmu;
// SWAP n

pub fn swap(reg: &mut u8, flags: &mut u8) {
    *flags &= 0;
    *reg = (*reg >> 4) + (*reg << 4);
    if *reg == 0 {
        *flags |= 0b10000000;
    }
}

pub fn swap_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    *flags &= 0;
    let old_val = mem.get_mem_u8(hl as usize);
    let new_val = (old_val >> 4) + (old_val << 4);
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 {
        *flags |= 0b10000000;
    }
}
