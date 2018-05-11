use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_swap(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xF0);
        let ret = swap_a!(self);
        assert_eq!(get_a!(self), 0x0F);
        assert_eq!(ret, 2);

        set_b!(self, 0x0F);
        let ret = swap_b!(self);
        assert_eq!(get_b!(self), 0xF0);
        assert_eq!(ret, 2);

        set_c!(self, 0xF0);
        let ret = swap_c!(self);
        assert_eq!(get_c!(self), 0x0F);
        assert_eq!(ret, 2);

        set_d!(self, 0x03);
        let ret = swap_d!(self);
        assert_eq!(get_d!(self), 0x30);
        assert_eq!(ret, 2);

        set_e!(self, 0x20);
        let ret = swap_e!(self);
        assert_eq!(get_e!(self), 0x02);
        assert_eq!(ret, 2);

        set_h!(self, 0x80);
        let ret = swap_h!(self);
        assert_eq!(get_h!(self), 0x08);
        assert_eq!(ret, 2);

        set_l!(self, 0x70);
        let ret = swap_l!(self);
        assert_eq!(get_l!(self), 0x07);
        assert_eq!(ret, 2);

        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        let ret = swap_hl_val!(self, mem);
        assert_eq!(mem.get_mem_u8(get_hl!(self) as usize), 0x10);
        assert_eq!(ret, 4);
    }

}
