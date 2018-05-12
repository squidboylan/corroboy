use super::super::super::mmu::Mmu;
use gameboy::cpu::Cpu;

impl Cpu {
    pub fn test_rst(&mut self, mem: &mut Mmu) {
        set_pc!(self, 0xFFF0);
        let ret = rst_nn!(self, mem, 0x20);
        assert_eq!(get_pc!(self), 0x20);
        assert_eq!(ret, 8);

        set_pc!(self, 0xF0F0);
        let ret = rst_nn!(self, mem, 0x28);
        assert_eq!(get_pc!(self), 0x28);
        assert_eq!(ret, 8);

        set_pc!(self, 0xF0F0);
        let ret = rst_nn!(self, mem, 0x30);
        assert_eq!(get_pc!(self), 0x30);
        assert_eq!(ret, 8);

        set_pc!(self, 0xF0F0);
        let ret = rst_nn!(self, mem, 0x38);
        assert_eq!(get_pc!(self), 0x38);
        assert_eq!(ret, 8);

        set_pc!(self, 0xF0F0);
        let ret = rst_nn!(self, mem, 0x40);
        assert_eq!(get_pc!(self), 0x40);
        assert_eq!(ret, 8);
    }

}
