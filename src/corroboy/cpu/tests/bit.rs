// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::corroboy::cpu::ops::bit::*;
use crate::corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_bit(&mut self) {
        set_f!(self, 0);
        set_h!(self, 0x80);
        bit(get_h!(self), 7, get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);

        set_h!(self, 0x70);
        bit(get_h!(self), 7, get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 0);
    }
}
