// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use corroboy::mmu::Mmu;

// RET

pub fn ret(mem: &mut Mmu, pc: &mut u16, sp: &mut u16) {
    *pc = mem.pop_u16(sp);
}

pub fn ret_nz(mem: &mut Mmu, flags: u8, pc: &mut u16, sp: &mut u16) -> bool {
    if flags & 0b10000000 == 0 {
        *pc = mem.pop_u16(sp);
        return true;
    }
    return false;
}

pub fn ret_z(mem: &mut Mmu, flags: u8, pc: &mut u16, sp: &mut u16) -> bool {
    if flags & 0b10000000 != 0 {
        *pc = mem.pop_u16(sp);
        return true;
    }
    return false;
}

pub fn ret_nc(mem: &mut Mmu, flags: u8, pc: &mut u16, sp: &mut u16) -> bool {
    if flags & 0b00010000 == 0 {
        *pc = mem.pop_u16(sp);
        return true;
    }
    return false;
}

pub fn ret_c(mem: &mut Mmu, flags: u8, pc: &mut u16, sp: &mut u16) -> bool {
    if flags & 0b00010000 != 0 {
        *pc = mem.pop_u16(sp);
        return true;
    }
    return false;
}
