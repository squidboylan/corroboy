use gameboy::mmu::Mmu;
// RR n rotate n right through carrry flag (C flag)

pub fn rr_reg(reg: &mut u8, flags: &mut u8) {
    let tmp = (*flags & 0b00010000) >> 4;
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg >> 1) + (tmp << 7);
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

pub fn rr_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let tmp = (*flags & 0b00010000) >> 4;
    let val = mem.get_mem_u8(hl as usize);
    let new_val = (val >> 1) + (tmp << 7);
    *flags &= 0b10010000;
    if val & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

// RRC n

pub fn rrc_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = *reg >> 1;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

pub fn rrc_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let val = mem.get_mem_u8(hl as usize);
    let new_val = val >> 1;
    *flags &= 0b10010000;
    if val & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}
