use super::super::super::mmu::Mmu;
use corroboy::cpu::ops::xor::*;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_xor(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        xor(get_a!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);

        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        xor(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xF0);

        set_a!(self, 0xFF);
        set_c!(self, 0xF0);
        xor(get_c!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x0F);

        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        xor(get_d!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        xor(get_e!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xFF);

        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        xor(get_h!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x7F);

        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        xor(get_l!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x8F);

        set_a!(self, 0xFF);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        xor(
            mem.get_mem_u8(get_hl!(self) as usize),
            get_mut_a!(self),
            get_mut_f!(self),
        );
        assert_eq!(get_a!(self), 0xFE);

        set_a!(self, 0xFF);
        xor(0x10, get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0xEF);

        // Test flags
        set_f!(self, 0xF0);
        set_a!(self, 0x00);
        set_b!(self, 0x00);
        xor(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0xF0);
        set_a!(self, 0x10);
        set_b!(self, 0x01);
        xor(get_b!(self), get_mut_a!(self), get_mut_f!(self));
        assert_eq!(get_a!(self), 0x11);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 0);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);
    }
}
