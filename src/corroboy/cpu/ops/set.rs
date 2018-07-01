pub use corroboy::mmu::Mmu;

// SET n,A

#[inline(always)]
pub fn set_reg(reg: &mut u8, bit: u8) {
    *reg |= 1 << bit;
}

// SET n,(HL)

#[inline(always)]
pub fn set_mem(mem: &mut Mmu, bit: u8, hl: u16) {
    let new_val = mem.get_mem_u8(hl as usize) | (1 << bit);
    mem.set_mem_u8(hl as usize, new_val);
}
