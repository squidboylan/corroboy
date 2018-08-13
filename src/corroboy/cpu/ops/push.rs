// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use corroboy::mmu::Mmu;

// PUSH nn

pub fn push(reg: u16, mem: &mut Mmu, sp: &mut u16) {
    mem.push_u16(sp, reg);
}
