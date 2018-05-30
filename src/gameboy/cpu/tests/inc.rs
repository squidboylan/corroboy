use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;
use gameboy::cpu::ops::inc::*;

impl Cpu {
    pub fn test_inc(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        inc(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);

        set_b!(self, 0x0F);
        inc(get_mut_b!(self), get_mut_f!(self));
        assert_eq!(get_b!(self), 0x10);

        set_c!(self, 0xF0);
        inc(get_mut_c!(self), get_mut_f!(self));
        assert_eq!(get_c!(self), 0xF1);

        set_d!(self, 0xF0);
        inc(get_mut_d!(self), get_mut_f!(self));
        assert_eq!(get_d!(self), 0xF1);

        set_e!(self, 0xF0);
        inc(get_mut_e!(self), get_mut_f!(self));
        assert_eq!(get_e!(self), 0xF1);

        set_h!(self, 0x00);
        inc(get_mut_h!(self), get_mut_f!(self));
        assert_eq!(get_h!(self), 0x01);

        set_l!(self, 0xF0);
        inc(get_mut_l!(self), get_mut_f!(self));
        assert_eq!(get_l!(self), 0xF1);

        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0xF0);
        inc_mem(mem, get_hl!(self), get_mut_f!(self));
        assert_eq!(mem.get_mem_u8(get_hl!(self) as usize), 0xF1);
    }
}
