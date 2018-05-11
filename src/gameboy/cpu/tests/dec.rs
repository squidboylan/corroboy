use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_dec(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        let ret = dec_a!(self);
        assert_eq!(get_a!(self), 0xFE);
        assert_eq!(ret, 1);

        set_b!(self, 0x0F);
        let ret = dec_b!(self);
        assert_eq!(get_b!(self), 0x0E);
        assert_eq!(ret, 1);

        set_c!(self, 0xF0);
        let ret = dec_c!(self);
        assert_eq!(get_c!(self), 0xEF);
        assert_eq!(ret, 1);

        set_d!(self, 0xF0);
        let ret = dec_d!(self);
        assert_eq!(get_d!(self), 0xEF);
        assert_eq!(ret, 1);

        set_e!(self, 0xF0);
        let ret = dec_e!(self);
        assert_eq!(get_e!(self), 0xEF);
        assert_eq!(ret, 1);

        set_h!(self, 0x00);
        let ret = dec_h!(self);
        assert_eq!(get_h!(self), 0xFF);
        assert_eq!(ret, 1);

        set_l!(self, 0xF0);
        let ret = dec_l!(self);
        assert_eq!(get_l!(self), 0xEF);
        assert_eq!(ret, 1);

        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x10);
        let ret = dec_hl_val!(self, mem);
        assert_eq!(mem.get_mem_u8(get_hl!(self) as usize), 0x0F);
        assert_eq!(ret, 3);
    }
}
