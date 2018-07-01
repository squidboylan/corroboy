use corroboy::mmu::Mmu;
// RL n rotate n left through carrry flag (C flag)

pub fn rla(reg: &mut u8, flags: &mut u8) {
    let tmp = (*flags & 0b00010000) >> 4;
    *flags = 0;
    if *reg & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg << 1) + tmp;
}

pub fn rl_reg(reg: &mut u8, flags: &mut u8) {
    let tmp = (*flags & 0b00010000) >> 4;
    *flags &= 0b10010000;
    if *reg & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg << 1) + tmp;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

pub fn rl_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let tmp = (*flags & 0b00010000) >> 4;
    let val = mem.get_mem_u8(hl as usize);
    let new_val = (val << 1) + tmp;
    *flags &= 0b10010000;
    if val & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

// CB extended ops
// RLC A rotate A left old bit 7 in C flag

pub fn rlca(reg: &mut u8, flags: &mut u8) {
    *flags = 0;

    *reg = reg.rotate_left(1);

    if *reg & 0x01 == 1 { *flags |= 0b00010000; }
}

pub fn rlc_reg(reg: &mut u8, flags: &mut u8) {
    *flags = 0;

    *reg = reg.rotate_left(1);

    if *reg & 0x01 == 1 { *flags |= 0b00010000; }

    if *reg == 0 { *flags |= 0b10000000; }
}

pub fn rlc_mem(mem: &mut Mmu, hl: u16, flags: &mut u8) {
    let val = mem.get_mem_u8(hl as usize);
    let tmp = (val & 0b10000000) >> 7;
    let new_val = (val << 1) | tmp;
    *flags &= 0b10010000;
    if val & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    mem.set_mem_u8(hl as usize, new_val);
    if new_val == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

