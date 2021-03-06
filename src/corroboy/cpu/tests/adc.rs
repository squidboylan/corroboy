// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::super::super::mmu::Mmu;
use crate::corroboy::cpu::ops::adc::*;
use crate::corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_adc(&mut self, mem: &mut Mmu) {
        set_c_flag!(self);
        set_a!(self, 0x44);
        adc(get_a!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x89);

        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        adc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);

        set_c_flag!(self);
        set_a!(self, 0x80);
        set_c!(self, 0x08);
        adc(get_c!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x89);

        set_c_flag!(self);
        set_a!(self, 0x00);
        set_d!(self, 0x0F);
        adc(get_d!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x10);

        set_c_flag!(self);
        set_a!(self, 0x00);
        set_e!(self, 0xF0);
        adc(get_e!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xF1);

        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        adc(get_h!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x80);
        assert_eq!(get_c_flag!(self), 1);

        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        adc(get_l!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x70);
        assert_eq!(get_c_flag!(self), 1);

        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        adc(
            mem.get_mem_u8(get_hl!(self) as usize),
            get_mut_a!(self),
            get_mut_f!(self),
        );
        assert_eq!(get_a!(self), 0xF2);

        set_c_flag!(self);
        set_a!(self, 0x00);
        adc(0x10, get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x11);

        // Test flags
        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        adc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_b!(self, 0x00);
        adc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x0F);
        set_b!(self, 0x00);
        adc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x10);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x00);
        set_b!(self, 0x00);
        adc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x01);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);
    }
}
