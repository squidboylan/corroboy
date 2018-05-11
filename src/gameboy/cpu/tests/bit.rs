use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_bit(&mut self, mem: &mut Mmu) {
        set_h!(self, 0x80);
        bit_7_h!(self);
        assert_eq!(get_z_flag!(self), 0);

        set_h!(self, 0x70);
        bit_7_h!(self);
        assert_eq!(get_z_flag!(self), 1);
    }
}
