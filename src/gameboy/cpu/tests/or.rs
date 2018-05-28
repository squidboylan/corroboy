use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;
use gameboy::cpu::ops::or::*;

impl Cpu {
    pub fn test_or(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        or(get_a!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        or(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0xFF);
        set_c!(self, 0xF0);
        or(get_c!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        or(get_d!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        or(get_e!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        or(get_h!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0x08);
        set_l!(self, 0x70);
        or(get_l!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x78);

        set_a!(self, 0x10);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        or(mem.get_mem_u8(get_hl!(self) as usize), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x11);

        set_a!(self, 0x30);
        or(0x30, get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x30);
    }

}
