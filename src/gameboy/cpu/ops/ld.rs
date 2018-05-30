use gameboy::mmu::Mmu;

// LD a,r2

pub fn ld_reg(val: u8, reg: &mut u8) {
    *reg = val;
}

pub fn ld_reg_16bit(val: u16, reg: &mut u16) {
    *reg = val;
}

pub fn ld_mem(loc: u16, mem: &mut Mmu, val: u8) {
    mem.set_mem_u8(loc as usize, val);
}

pub fn ld_mem_16bit(loc: u16, mem: &mut Mmu, val: u16) {
    mem.set_mem_u16(loc as usize, val);
}

// LDI A, (HL)
// Load (HL) into A and then increment HL

pub fn ldi_a_hl(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    *a = mem.get_mem_u8(*hl as usize);
    *hl += 1;
}

// LDI (HL), A
// Load A into (HL) and then increment HL

pub fn ldi_hl_a(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    mem.set_mem_u8(*hl as usize, *a);
    *hl += 1;
}

// LDD A, (HL)
// Load (HL) into A and then decrement HL

pub fn ldd_a_hl(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    *a = mem.get_mem_u8(*hl as usize);
    *hl -= 1;
}

// LDD (HL), A
// Load A into (HL) and then decrement HL

pub fn ldd_hl_a(a: &mut u8, hl: &mut u16, mem: &mut Mmu) {
    mem.set_mem_u8(*hl as usize, *a);
    *hl -= 1;
}

// LDHL SP,n

// TODO make sure this works as intended and write tests
pub fn ldhl_sp_n(val: u8, sp: u16, hl: &mut u16, flags: &mut u8) {
    *hl = (sp as i16 + ((val as i8) as i16)) as u16;
    *flags &= 0b00111111;
    // This is fine because we care about the last 4 bits and not the sign? Docs are hard to
    // find on this
    if sp & 0b0000000000001111 + (val as u16) & 0b0000000000001111 > 0xF { *flags |= 0b00100000; }
    else { *flags &= 0b11011111; }
    if *hl < sp { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
}
