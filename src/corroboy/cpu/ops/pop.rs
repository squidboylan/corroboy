// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::corroboy::mmu::Mmu;

// POP nn

pub fn pop(reg: &mut u16, mem: &Mmu, sp: &mut u16) {
    *reg = mem.pop_u16(sp);
}

pub fn pop_af(reg: &mut u16, mem: &Mmu, sp: &mut u16) {
    *reg = mem.pop_u16(sp) & 0xFFF0;
}
