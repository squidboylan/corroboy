// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::super::super::mmu::Mmu;
use corroboy::cpu::ops::and::*;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_and(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        and(get_a!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        and(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x0F);

        set_a!(self, 0xFF);
        set_c!(self, 0xF0);
        and(get_c!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xF0);

        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        and(get_d!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);

        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        and(get_e!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);

        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        and(get_h!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x80);

        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        and(get_l!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x70);

        set_a!(self, 0xFF);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        and(
            mem.get_mem_u8(get_hl!(self) as usize),
            get_mut_a!(self),
            get_mut_f!(self),
        );
        assert_eq!(get_a!(self), 0x01);

        set_a!(self, 0xFF);
        and(0x10, get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x10);

        // Test flags
        set_f!(self, 0);
        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        and(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x0F);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        // Test flags
        set_f!(self, 0);
        set_a!(self, 0xFF);
        set_b!(self, 0x00);
        and(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        // Test flags
        set_f!(self, 0);
        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        and(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);
    }
}
