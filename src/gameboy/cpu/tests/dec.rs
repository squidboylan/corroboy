use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;
use gameboy::cpu::ops::dec::*;

impl Cpu {
    pub fn test_dec(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        dec(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFE);

        set_b!(self, 0x0F);
        dec(get_mut_b!(self), get_mut_f!(self));
        assert_eq!(get_b!(self), 0x0E);

        set_c!(self, 0xF0);
        dec(get_mut_c!(self), get_mut_f!(self));
        assert_eq!(get_c!(self), 0xEF);

        set_d!(self, 0xF0);
        dec(get_mut_d!(self), get_mut_f!(self));
        assert_eq!(get_d!(self), 0xEF);

        set_e!(self, 0xF0);
        dec(get_mut_e!(self), get_mut_f!(self));
        assert_eq!(get_e!(self), 0xEF);

        set_h!(self, 0x00);
        dec(get_mut_h!(self), get_mut_f!(self));
        assert_eq!(get_h!(self), 0xFF);

        set_l!(self, 0xF0);
        dec(get_mut_l!(self), get_mut_f!(self));
        assert_eq!(get_l!(self), 0xEF);

        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x10);
        let ret = dec_hl_val!(self, mem);
        assert_eq!(mem.get_mem_u8(get_hl!(self) as usize), 0x0F);
        assert_eq!(ret, 3);
    }
}
