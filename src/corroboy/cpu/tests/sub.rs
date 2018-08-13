// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::super::super::mmu::Mmu;
use corroboy::cpu::ops::sub::*;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_sub(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        sub(get_a!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x0);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        sub(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xE1);
        assert_eq!(get_c_flag!(self), 0);

        set_a!(self, 0x80);
        set_c!(self, 0x08);
        sub(get_c!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x78);
        assert_eq!(get_c_flag!(self), 0);

        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        sub(get_d!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xE1);
        assert_eq!(get_c_flag!(self), 0);

        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        sub(get_e!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x1F);
        assert_eq!(get_c_flag!(self), 1);

        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        sub(get_h!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x7F);
        assert_eq!(get_c_flag!(self), 0);

        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        sub(get_l!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x8F);
        assert_eq!(get_c_flag!(self), 0);

        set_a!(self, 0xFF);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        sub(
            mem.get_mem_u8(get_hl!(self) as usize),
            get_mut_a!(self),
            get_mut_f!(self),
        );
        assert_eq!(get_a!(self), 0xFE);
        assert_eq!(get_c_flag!(self), 0);

        set_a!(self, 0x00);
        sub(0x10, get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xF0);
        assert_eq!(get_c_flag!(self), 1);

        // Test sub flags
        set_f!(self, 0x00);
        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        sub(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xE1);
        assert_eq!(get_c_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);

        // Test sub flags
        set_f!(self, 0x00);
        set_a!(self, 0x00);
        set_b!(self, 0x01);
        sub(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(get_c_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);

        // Test sub flags
        set_f!(self, 0x00);
        set_a!(self, 0x01);
        set_b!(self, 0x01);
        sub(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_c_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_z_flag!(self), 1);
    }
}
