// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::corroboy::cpu::ops::sr::*;
use crate::corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_sr(&mut self) {
        set_f!(self, 0);
        set_a!(self, 0xFF);
        sra_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x0F);
        sra_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x07);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0x01);
        sra_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0xFF);
        srl_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x7F);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x0F);
        srl_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x07);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0x01);
        srl_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);
    }
}
