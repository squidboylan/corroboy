// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use corroboy::cpu::ops::rr::*;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_rr(&mut self) {
        set_f!(self, 0);
        set_a!(self, 0xFF);
        rr_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x7F);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x0F);
        rr_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x87);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0x01);
        rr_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0xFF);
        rrc_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x0F);
        rrc_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x87);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0x01);
        rrc_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x80);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);
    }
}
