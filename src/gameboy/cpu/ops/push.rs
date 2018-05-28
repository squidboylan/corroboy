use gameboy::mmu::Mmu;

// PUSH nn

pub fn push(reg: u16, mem: &mut Mmu, sp: &mut u16) {
    mem.push_u16(sp, reg);
}
