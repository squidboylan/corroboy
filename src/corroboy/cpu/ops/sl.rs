use corroboy::mmu::Mmu;

// CB extended ops
// SLA N
// Shift N left, old bit 7 goes into C flag

pub fn sla_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = *reg << 1;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

pub fn sla_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let val = mem.get_mem_u8(hl as usize);
    let new_val = val << 1;
    *flags &= 0b10010000;
    if val & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}
