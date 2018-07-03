use corroboy::mmu::Mmu;
// INC n

#[inline(always)]
pub fn inc(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10110000;

    if *reg & 0b00001111 == 0b00001111 {
        *flags |= 0b00100000;
    } else {
        *flags &= 0b11010000;
    }

    *reg += 1;
    if *reg == 0 {
        *flags |= 0b10000000;
    } else {
        *flags &= 0b01110000;
    }
}

pub fn inc_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let val = mem.get_mem_u8(hl as usize);

    if val & 0b00001111 == 0b00001111 {
        *flags |= 0b00100000;
    } else {
        *flags &= 0b11010000;
    }

    let val = val + 1;
    mem.set_mem_u8(hl as usize, val);
    *flags &= 0b10110000;
    if val == 0 {
        *flags |= 0b10000000;
    } else {
        *flags &= 0b01110000;
    }
}

// INC nn

pub fn inc_16bit(reg: &mut u16) {
    *reg += 1;
}
