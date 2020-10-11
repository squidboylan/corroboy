// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::super::super::mmu::Mmu;
use crate::corroboy::cpu::ops::sbc::*;
use crate::corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_sbc(&mut self, mem: &mut Mmu) {
        set_c_flag!(self);
        set_a!(self, 0xFF);
        sbc(get_a!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        sbc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xE0);

        set_c_flag!(self);
        set_a!(self, 0x80);
        set_c!(self, 0x08);
        sbc(get_c!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x77);

        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        sbc(get_d!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xE0);

        set_c_flag!(self);
        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        sbc(get_e!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x1E);

        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        sbc(get_h!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x7E);

        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        sbc(get_l!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x8E);

        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        sbc(
            mem.get_mem_u8(get_hl!(self) as usize),
            get_mut_a!(self),
            get_mut_f!(self),
        );
        assert_eq!(get_a!(self), 0xFD);

        set_c_flag!(self);
        set_a!(self, 0x00);
        sbc(0x10, get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xEF);

        // Test flags
        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        sbc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xE0);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x02);
        set_b!(self, 0x01);
        sbc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x01);
        set_b!(self, 0x01);
        sbc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 1);
    }
}
