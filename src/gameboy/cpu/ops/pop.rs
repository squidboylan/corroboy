use gameboy::mmu::Mmu;

// POP nn

pub fn pop(reg: &mut u16, mem: &Mmu, sp: &mut u16) {
    *reg = mem.pop_u16(sp);
}

pub fn pop_af(reg: &mut u16, mem: &Mmu, sp: &mut u16) {
    *reg = mem.pop_u16(sp) & 0xFFF0;
}
