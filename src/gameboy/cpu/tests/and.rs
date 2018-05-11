use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_and(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        let ret = and_a!(self);
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_b!(self, 0x0F);
        let ret = and_b!(self);
        assert_eq!(get_a!(self), 0x0F);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_c!(self, 0xF0);
        let ret = and_c!(self);
        assert_eq!(get_a!(self), 0xF0);
        assert_eq!(ret, 1);

        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        let ret = and_d!(self);
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(ret, 1);

        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        let ret = and_e!(self);
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        let ret = and_h!(self);
        assert_eq!(get_a!(self), 0x80);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        let ret = and_l!(self);
        assert_eq!(get_a!(self), 0x70);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        let ret = and_hl_val!(self, mem);
        assert_eq!(get_a!(self), 0x01);
        assert_eq!(ret, 2);

        set_a!(self, 0xFF);
        let ret = and_param!(self, 0x10);
        assert_eq!(get_a!(self), 0x10);
        assert_eq!(ret, 2);
    }

}
