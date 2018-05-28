use gameboy::mmu::Mmu;

// RST nn

pub fn rst(loc: u16, mem: &mut Mmu, sp: &mut u16, pc: &mut u16) {
    mem.push_u16(sp, *pc);
    *pc = loc;
}
