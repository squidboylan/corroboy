use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_inc(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        let ret = inc_a!(self);
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(ret, 1);
        assert_eq!(get_z_flag!(self), 1);

        set_b!(self, 0x0F);
        let ret = inc_b!(self);
        assert_eq!(get_b!(self), 0x10);
        assert_eq!(ret, 1);

        set_c!(self, 0xF0);
        let ret = inc_c!(self);
        assert_eq!(get_c!(self), 0xF1);
        assert_eq!(ret, 1);

        set_d!(self, 0xF0);
        let ret = inc_d!(self);
        assert_eq!(get_d!(self), 0xF1);
        assert_eq!(ret, 1);

        set_e!(self, 0xF0);
        let ret = inc_e!(self);
        assert_eq!(get_e!(self), 0xF1);
        assert_eq!(ret, 1);

        set_h!(self, 0x00);
        let ret = inc_h!(self);
        assert_eq!(get_h!(self), 0x01);
        assert_eq!(ret, 1);

        set_l!(self, 0xF0);
        let ret = inc_l!(self);
        assert_eq!(get_l!(self), 0xF1);
        assert_eq!(ret, 1);

        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0xF0);
        let ret = inc_hl_val!(self, mem);
        assert_eq!(mem.get_mem_u8(get_hl!(self) as usize), 0xF1);
        assert_eq!(ret, 3);
    }
}
