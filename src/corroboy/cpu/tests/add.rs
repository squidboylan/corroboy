// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::super::super::mmu::Mmu;
use crate::corroboy::cpu::ops::add::*;
use crate::corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_add(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        add(get_a!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFE);
        assert_eq!(get_c_flag!(self), 1);

        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        add(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0x80);
        set_c!(self, 0x08);
        add(get_c!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x88);

        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        add(get_d!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        add(get_e!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        add(get_h!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x7F);
        assert_eq!(get_c_flag!(self), 1);

        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        add(get_l!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x6F);

        set_a!(self, 0xFF);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        add(
            mem.get_mem_u8(get_hl!(self) as usize),
            get_mut_a!(self),
            get_mut_f!(self),
        );
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_c_flag!(self), 1);

        set_a!(self, 0x00);
        add(0x10, get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x10);

        set_hl!(self, 0x00FF);
        set_bc!(self, 0x0001);
        add_16bit(get_bc!(self), get_mut_hl!(self), get_mut_f!(self));
        assert_eq!(get_hl!(self), 0x0100);

        set_hl!(self, 0x00FF);
        set_de!(self, 0x00FF);
        add_16bit(get_de!(self), get_mut_hl!(self), get_mut_f!(self));
        assert_eq!(get_hl!(self), 0x01FE);

        set_hl!(self, 0x0440);
        add_16bit(get_hl!(self), get_mut_hl!(self), get_mut_f!(self));
        assert_eq!(get_hl!(self), 0x0880);

        set_hl!(self, 0x0440);
        set_sp!(self, 0xF440);
        add_16bit(get_sp!(self), get_mut_hl!(self), get_mut_f!(self));
        assert_eq!(get_hl!(self), 0xF880);

        // Test flags
        set_f!(self, 0x00);
        set_a!(self, 0xFF);
        set_b!(self, 0x01);
        add(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_f!(self), 0b10110000);
    }
}
