use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;
use gameboy::cpu::ops::cp::*;

impl Cpu {
    pub fn test_cp(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        cp(get_a!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        cp(get_b!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        set_c!(self, 0x0F);
        cp(get_c!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        set_d!(self, 0x0F);
        cp(get_d!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        set_e!(self, 0x0F);
        cp(get_e!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        set_h!(self, 0x0F);
        cp(get_h!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        set_l!(self, 0x0F);
        cp(get_l!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        cp(0xFF, get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);

        set_a!(self, 0xFF);
        mem.set_mem_u8(get_hl!(self) as usize, 0xFF);
        cp(mem.get_mem_u8(get_hl!(self) as usize), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);

        // Test flags
        set_f!(self, 0);
        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        cp(get_b!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);

        set_f!(self, 0);
        set_a!(self, 0x00);
        set_b!(self, 0x01);
        cp(get_b!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 1);
        assert_eq!(get_c_flag!(self), 1);

        set_f!(self, 0);
        set_a!(self, 0x04);
        set_b!(self, 0x04);
        cp(get_b!(self), get_a!(self), get_mut_f!(self));
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(get_h_flag!(self), 0);
        assert_eq!(get_c_flag!(self), 0);
    }
}
