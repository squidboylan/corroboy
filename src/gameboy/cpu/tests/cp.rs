use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_cp(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        let ret = cp_a!(self);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        let ret = cp_b!(self);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_c!(self, 0x0F);
        let ret = cp_c!(self);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_d!(self, 0x0F);
        let ret = cp_d!(self);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_e!(self, 0x0F);
        let ret = cp_e!(self);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_h!(self, 0x0F);
        let ret = cp_h!(self);
        assert_eq!(get_z_flag!(self), 0);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        let ret = cp_n!(self, 0xFF);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 2);

        set_a!(self, 0xFF);
        mem.set_mem_u8(get_hl!(self) as usize, 0xFF);
        let ret = cp_hl_val!(self, mem);
        assert_eq!(get_z_flag!(self), 1);
        assert_eq!(get_n_flag!(self), 1);
        assert_eq!(ret, 2);
    }
}
