// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use super::super::super::mmu::Mmu;
use corroboy::cpu::ops::rst;
use corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_rst(&mut self, mem: &mut Mmu) {
        set_pc!(self, 0xFFF0);
        rst(0x20, mem, get_mut_sp!(self), get_mut_pc!(self));
        assert_eq!(get_pc!(self), 0x20);

        set_pc!(self, 0xF0F0);
        rst(0x28, mem, get_mut_sp!(self), get_mut_pc!(self));
        assert_eq!(get_pc!(self), 0x28);

        set_pc!(self, 0xF0F0);
        rst(0x30, mem, get_mut_sp!(self), get_mut_pc!(self));
        assert_eq!(get_pc!(self), 0x30);

        set_pc!(self, 0xF0F0);
        rst(0x38, mem, get_mut_sp!(self), get_mut_pc!(self));
        assert_eq!(get_pc!(self), 0x38);

        set_pc!(self, 0xF0F0);
        rst(0x40, mem, get_mut_sp!(self), get_mut_pc!(self));
        assert_eq!(get_pc!(self), 0x40);
    }
}
