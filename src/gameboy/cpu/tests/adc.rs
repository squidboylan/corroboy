use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_adc(&mut self, mem: &mut Mmu) {
        set_c_flag!(self);
        set_a!(self, 0x44);
        let ret = adc_a_a!(self);
        assert_eq!(get_a!(self), 0x89);
        assert_eq!(ret, 1);

        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        let ret = adc_a_b!(self);
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(ret, 1);

        set_c_flag!(self);
        set_a!(self, 0x80);
        set_c!(self, 0x08);
        let ret = adc_a_c!(self);
        assert_eq!(get_a!(self), 0x89);
        assert_eq!(ret, 1);

        set_c_flag!(self);
        set_a!(self, 0x00);
        set_d!(self, 0x0F);
        let ret = adc_a_d!(self);
        assert_eq!(get_a!(self), 0x10);
        assert_eq!(ret, 1);

        set_c_flag!(self);
        set_a!(self, 0x00);
        set_e!(self, 0xF0);
        let ret = adc_a_e!(self);
        assert_eq!(get_a!(self), 0xF1);
        assert_eq!(ret, 1);

        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        let ret = adc_a_h!(self);
        assert_eq!(get_a!(self), 0x80);
        assert_eq!(ret, 1);
        assert_eq!(get_c_flag!(self), 1);

        set_c_flag!(self);
        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        let ret = adc_a_l!(self);
        assert_eq!(get_a!(self), 0x70);
        assert_eq!(ret, 1);
        assert_eq!(get_c_flag!(self), 1);

        set_c_flag!(self);
        set_a!(self, 0xF0);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        let ret = adc_a_hl_val!(self, mem);
        assert_eq!(get_a!(self), 0xF2);
        assert_eq!(ret, 2);

        set_c_flag!(self);
        set_a!(self, 0x00);
        let ret = adc_a_param!(self, 0x10);
        assert_eq!(get_a!(self), 0x11);
        assert_eq!(ret, 2);
    }
}
