use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_add(&mut self, mem: &mut Mmu) {
        set_a!(self, 0xFF);
        let ret = add_a_a!(self);
        assert_eq!(get_a!(self), 0xFE);
        assert_eq!(ret, 1);
        assert_eq!(get_c_flag!(self), 1);

        set_a!(self, 0xF0);
        set_b!(self, 0x0F);
        let ret = add_a_b!(self);
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(ret, 1);

        set_a!(self, 0x80);
        set_c!(self, 0x08);
        let ret = add_a_c!(self);
        assert_eq!(get_a!(self), 0x88);
        assert_eq!(ret, 1);

        set_a!(self, 0xF0);
        set_d!(self, 0x0F);
        let ret = add_a_d!(self);
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(ret, 1);

        set_a!(self, 0x0F);
        set_e!(self, 0xF0);
        let ret = add_a_e!(self);
        assert_eq!(get_a!(self), 0xFF);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_h!(self, 0x80);
        let ret = add_a_h!(self);
        assert_eq!(get_a!(self), 0x7F);
        assert_eq!(ret, 1);
        assert_eq!(get_c_flag!(self), 1);

        set_a!(self, 0xFF);
        set_l!(self, 0x70);
        let ret = add_a_l!(self);
        assert_eq!(get_a!(self), 0x6F);
        assert_eq!(ret, 1);

        set_a!(self, 0xFF);
        set_hl!(self, 0xC000);
        mem.set_mem_u8(get_hl!(self) as usize, 0x01);
        let ret = add_a_hl_val!(self, mem);
        assert_eq!(get_a!(self), 0x00);
        assert_eq!(ret, 2);
        assert_eq!(get_c_flag!(self), 1);

        set_a!(self, 0x00);
        let ret = add_a_param!(self, 0x10);
        assert_eq!(get_a!(self), 0x10);
        assert_eq!(ret, 2);

        set_hl!(self, 0x00FF);
        set_bc!(self, 0x0001);
        let ret = add_hl_bc!(self);
        assert_eq!(get_hl!(self), 0x0100);
        assert_eq!(ret, 2);

        set_hl!(self, 0x00FF);
        set_de!(self, 0x00FF);
        let ret = add_hl_de!(self);
        assert_eq!(get_hl!(self), 0x01FE);
        assert_eq!(ret, 2);

        set_hl!(self, 0x0440);
        let ret = add_hl_hl!(self);
        assert_eq!(get_hl!(self), 0x0880);
        assert_eq!(ret, 2);

        set_hl!(self, 0x0440);
        set_sp!(self, 0xF440);
        let ret = add_hl_sp!(self);
        assert_eq!(get_hl!(self), 0xF880);
        assert_eq!(ret, 2);
    }
}
