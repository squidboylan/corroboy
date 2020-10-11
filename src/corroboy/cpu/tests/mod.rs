// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

mod adc;
mod add;
mod and;
mod bit;
mod cp;
mod dec;
mod inc;
mod or;
mod rl;
mod rr;
mod rst;
mod sbc;
mod sl;
mod sr;
mod sub;
mod swap;
mod xor;

use super::super::mmu::Mmu;
use crate::corroboy::cpu::Cpu;

impl Cpu {
    pub fn test_registers(&mut self) {
        set_a!(self, 1);
        set_f!(self, 2);

        assert_eq!(get_a!(self), 1);
        assert_eq!(get_f!(self), 2);
        assert_eq!(get_af!(self), 258);

        set_af!(self, 512);
        assert_eq!(get_a!(self), 2);
        assert_eq!(get_f!(self), 0);
        assert_eq!(get_af!(self), 512);

        set_pc!(self, 10);
        assert_eq!(get_pc!(self), 10);

        set_hl!(self, 2);
        assert_eq!(get_hl!(self), 2);
        set_hl!(self, get_hl!(self) - 1);
        assert_eq!(get_hl!(self), 1);
    }

    pub fn test_opcodes(&mut self, mem: &mut Mmu) {
        set_pc!(self, 0xC000);
        // 0x06 arg
        mem.set_mem_u8(0xC000, 0x01);
        // 0x0E arg
        mem.set_mem_u8(0xC001, 0b10);
        //self.op_param_8_bit(0x06, 0b00000001);
        self.exec_dispatcher(mem, 0x06);
        assert_eq!(get_b!(self), 0b00000001);
        self.exec_dispatcher(mem, 0x0E);
        assert_eq!(get_c!(self), 0b00000010);
        assert_eq!(get_bc!(self), 0b0000000100000010);
    }

    pub fn test_flag_bits(&mut self) {
        set_z_flag!(self);
        assert_eq!(get_z_flag!(self), 1);
        set_z_flag!(self);
        assert_eq!(get_z_flag!(self), 1);
        set_z_flag!(self);
        assert_eq!(get_z_flag!(self), 1);
        unset_z_flag!(self);
        assert_eq!(get_z_flag!(self), 0);
        set_h_flag!(self);
        assert_eq!(get_h_flag!(self), 1);
        unset_h_flag!(self);
        assert_eq!(get_h_flag!(self), 0);
        set_n_flag!(self);
        assert_eq!(get_n_flag!(self), 1);
        unset_n_flag!(self);
        assert_eq!(get_n_flag!(self), 0);
        set_c_flag!(self);
        assert_eq!(get_c_flag!(self), 1);
        unset_c_flag!(self);
        assert_eq!(get_c_flag!(self), 0);
    }

    pub fn test_get_opcode(&mut self, mem: &mut Mmu) {
        mem.set_mem_u8(0xC000, 0xCB);
        mem.set_mem_u8(0xC001, 0x10);
        mem.set_mem_u8(0xC002, 0x20);

        set_pc!(self, 0xC000);
        assert_eq!(self.get_opcode(mem), 0xCB10);
        assert_eq!(get_pc!(self), 0xC002);
        assert_eq!(self.get_opcode(mem), 0x20);
        assert_eq!(get_pc!(self), 0xC003);
    }

    pub fn test_get_param(&mut self, mem: &mut Mmu) {
        mem.set_mem_u8(0xC003, 0x10);
        mem.set_mem_u8(0xC004, 0x20);
        mem.set_mem_u8(0xC005, 0x30);

        set_pc!(self, 0xC003);
        assert_eq!(self.get_param_8_bit(mem), 0x10);
        assert_eq!(self.get_param_16_bit(mem), 0x3020);
        assert_eq!(get_pc!(self), 0xC006);
    }

    pub fn test_stack(&mut self, mem: &mut Mmu) {
        set_sp!(self, 0xFFFE);
        mem.push_u16(get_sp_mut!(self), 0x3020);

        assert_eq!(get_sp!(self), 0xFFFC);
        assert_eq!(mem.pop_u16(get_sp_mut!(self)), 0x3020);
    }
}

#[test]
fn registers() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_registers();
}

#[test]
fn opcodes() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_opcodes(&mut mem);
}

#[test]
fn flag_bits() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_flag_bits();
}

#[test]
fn get_opcode() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_get_opcode(&mut mem);
}

#[test]
fn get_param() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_get_param(&mut mem);
}

#[test]
fn stack() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_stack(&mut mem);
}

#[test]
fn and() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_and(&mut mem);
}

#[test]
fn add() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_add(&mut mem);
}

#[test]
fn adc() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_adc(&mut mem);
}

#[test]
fn xor() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_xor(&mut mem);
}

#[test]
fn or() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_or(&mut mem);
}

#[test]
fn inc() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_inc(&mut mem);
}

#[test]
fn dec() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_dec(&mut mem);
}

#[test]
fn bit() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_bit();
}

#[test]
fn cp() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_cp(&mut mem);
}

#[test]
fn swap() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_swap(&mut mem);
}

#[test]
fn sub() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_sub(&mut mem);
}

#[test]
fn sbc() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_sbc(&mut mem);
}

#[test]
fn rst() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new(None);
    derp.test_rst(&mut mem);
}

#[test]
fn rl() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_rl();
}

#[test]
fn rr() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_rr();
}

#[test]
fn sl() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_sl();
}

#[test]
fn sr() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_sr();
}
