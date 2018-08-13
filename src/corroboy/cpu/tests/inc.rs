// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::super::super::mmu::Mmu;
use corroboy::cpu::ops::inc::*;
use corroboy::cpu::Cpu;

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

        // Test flags
        set_f!(self, 0);
        set_a!(self, 0xFF);
        inc(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_a!(self, 0x0F);
        inc(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x10);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_a!(self, 0x0E);
        inc(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x0F);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);
    }
}
