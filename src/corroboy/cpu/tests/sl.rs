// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use corroboy::cpu::ops::sl::*;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_sl(&mut self) {
        set_f!(self, 0);
        set_a!(self, 0xFF);
        sla_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFE);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0x0F);
        sla_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x1E);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_a!(self, 0x80);
        sla_reg(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 1);
    }
}
