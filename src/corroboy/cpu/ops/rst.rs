// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::corroboy::mmu::Mmu;

// RST nn

pub fn rst(loc: u16, mem: &mut Mmu, sp: &mut u16, pc: &mut u16) {
    mem.push_u16(sp, *pc);
    *pc = loc;
}
