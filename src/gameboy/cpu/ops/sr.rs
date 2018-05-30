use gameboy::mmu::Mmu;

// SRL n

pub fn srl_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = *reg >> 1;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

pub fn srl_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let val = mem.get_mem_u8(hl as usize);
    let new_val = val >> 1;
    *flags &= 0b10010000;
    if val & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

// SRA n

pub fn sra_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg >> 1) + (*reg & 0b10000000);
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

pub fn sra_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let val = mem.get_mem_u8(hl as usize);
    let new_val = (val >> 1) + (val & 0b10000000);
    *flags &= 0b10010000;
    if val & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

