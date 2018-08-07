// Copyright (c) 2018 Caleb Boylan

use super::super::super::mmu::Mmu;
use corroboy::cpu::ops::swap::*;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_swap(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xF0);
        swap(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x0F);

        set_b!(self, 0x0F);
        swap(get_mut_b!(self), get_mut_f!(self));
        assert_eq!(get_b!(self), 0xF0);

        set_c!(self, 0xF0);
        swap(get_mut_c!(self), get_mut_f!(self));
        assert_eq!(get_c!(self), 0x0F);

        set_d!(self, 0x03);
        swap(get_mut_d!(self), get_mut_f!(self));
        assert_eq!(get_d!(self), 0x30);

        set_e!(self, 0x20);
        swap(get_mut_e!(self), get_mut_f!(self));
        assert_eq!(get_e!(self), 0x02);

        set_h!(self, 0x80);
        swap(get_mut_h!(self), get_mut_f!(self));
        assert_eq!(get_h!(self), 0x08);

        set_l!(self, 0x70);
        swap(get_mut_l!(self), get_mut_f!(self));
        assert_eq!(get_l!(self), 0x07);

        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        swap_mem(mem, get_hl!(self), get_mut_f!(self));
        assert_eq!(mem.get_mem_u8(get_hl!(self) as usize), 0x10);

        // Test flags
        set_f!(self, 0xF0);
        set_a!(self, 0xF0);
        swap(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x0F);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0xF0);
        set_a!(self, 0x00);
        swap(get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);
    }
}
