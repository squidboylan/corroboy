use corroboy::mmu::Mmu;
// DEC n

#[inline(always)]
pub fn dec(reg: &mut u8, flags: &mut u8) {
    *reg -= 1;
    *flags |= 0b01000000;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01110000; }
    if *reg & 0b00001111 == 0b00001111 { *flags |= 0b00100000; }
    else { *flags &= 0b11010000; }
}

pub fn dec_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let val = mem.get_mem_u8(hl as usize) - 1;
    mem.set_mem_u8(hl as usize, val);
    *flags |= 0b01000000;
    if val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01110000; }
    if val & 0b00001111 == 0b00001111 { *flags |= 0b00100000; }
    else { *flags &= 0b11010000; }
}

// DEC nn

pub fn dec_16bit(reg: &mut u16) {
    *reg -= 1;
}
