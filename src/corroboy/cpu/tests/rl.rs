// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use corroboy::cpu::ops::rl::*;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_rl(&mut self) {
        set_f!(self, 0);
        set_a!(self, 0xFF);
        rl_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFE);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x0F);
        rl_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x1F);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_a!(self, 0x80);
        rl_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0xFF);
        rlc_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_c_flag!(self);
        set_a!(self, 0x0F);
        rlc_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x1E);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_a!(self, 0x80);
        rlc_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x01);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);
    }
}
