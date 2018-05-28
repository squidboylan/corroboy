pub use gameboy::mmu::Mmu;

// RES n,A

#[inline(always)]
pub fn res_reg(reg: &mut u8, bit: u8) {
    *reg &= 0b11111111 - (1 << bit);
}

// RES n,HL

#[inline(always)]
pub fn res_mem(mem: &mut Mmu, bit: u8, hl: u16) {
    let new_val = mem.get_mem_u8(hl as usize) & (0b11111111 - (1 << bit));
    mem.set_mem_u8(hl as usize, new_val);
}
