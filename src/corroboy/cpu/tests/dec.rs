// Copyright (c) 2018 Caleb Boylan

use super::super::super::mmu::Mmu;
use corroboy::cpu::ops::dec::*;
use corroboy::cpu::Cpu;

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
        dec_mem(mem, get_hl!(self), get_mut_f!(self));
        assert_eq!(mem.get_mem_u8(get_hl!(self) as usize), 0x0F);

        // Test flags
        set_f!(self, 0);
        set_a!(self, 0xFF);
        dec(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFE);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_a!(self, 0xF0);
        dec(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xEF);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);
    }
}
