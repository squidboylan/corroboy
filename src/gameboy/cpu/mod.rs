#[macro_use]
pub mod ops;

#[macro_use]
pub mod registers;

use super::mmu::Mmu;

pub struct Cpu {
    af: registers::Reg16Bit,
    bc: registers::Reg16Bit,
    de: registers::Reg16Bit,
    hl: registers::Reg16Bit,
    sp: registers::Reg16Bit,
    pc: registers::Reg16Bit,
}

impl Cpu {
    pub fn new() -> Cpu {
        let af = registers::Reg16Bit{ whole: 0 };
        let bc = registers::Reg16Bit{ whole: 0 };
        let de = registers::Reg16Bit{ whole: 0 };
        let hl = registers::Reg16Bit{ whole: 0 };
        let sp = registers::Reg16Bit{ whole: 0 };
        let pc = registers::Reg16Bit{ whole: 0 };

        Cpu {af, bc, de, hl, sp, pc}
    }

    fn get_opcode(&mut self, mem: &mut Mmu) -> u16 {
        let mut pc = get_pc!(self);
        let temp = mem.get_mem_u8(pc as usize);
        pc += 1;
        let temp_u16: u16;
        if temp == 0xCB {
            temp_u16 = 0xCB00 + (mem.get_mem_u8(pc as usize) as u16);
            pc += 1;
        }
        else {
            temp_u16 = temp as u16;
        }

        set_pc!(self, pc);

        temp_u16
    }

    fn get_param_8_bit(&mut self, mem: &mut Mmu) -> u8 {
        let temp = mem.get_mem_u8(get_pc!(self) as usize);
        set_pc!(self, get_pc!(self) + 1);
        if cfg!(debug_assertions = true) {
            println!("8 bit param: b: {:b}, hex: {:x}, signed: {:x}", temp, temp, (temp as i8) as i16);
        }
        temp
    }

    fn get_param_16_bit(&mut self, mem: &mut Mmu) -> u16 {
        let mut pc = get_pc!(self);
        let temp = mem.get_mem_u8(pc as usize);
        pc += 1;
        let temp_u16: u16 = (temp as u16) + ((mem.get_mem_u8(pc as usize) as u16) << 8);
        pc += 1;
        set_pc!(self, pc);
        if cfg!(debug_assertions = true) {
            println!("16 bit param: b: {:b}, hex: {:x}", temp_u16, temp_u16);
        }
        temp_u16
    }

    fn exec_dispatcher(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        match opcode {
            0x00 => nop!(self),
            0x01 => return self.op_param_16_bit(mem, opcode),
            0x08 => return self.op_param_16_bit(mem, opcode),
            0x11 => return self.op_param_16_bit(mem, opcode),
            0x21 => return self.op_param_16_bit(mem, opcode),
            0x31 => return self.op_param_16_bit(mem, opcode),
            0xFA => return self.op_param_16_bit(mem, opcode),
            0xCD => return self.op_param_16_bit(mem, opcode),

            // JP cc,nn
            0xC2 => return self.op_param_16_bit(mem, opcode),
            0xCA => return self.op_param_16_bit(mem, opcode),
            0xD2 => return self.op_param_16_bit(mem, opcode),
            0xDA => return self.op_param_16_bit(mem, opcode),

            0xC3 => return self.op_param_16_bit(mem, opcode),
            0xEA => return self.op_param_16_bit(mem, opcode),
            0x06 => return self.op_param_8_bit(mem, opcode),
            0x0E => return self.op_param_8_bit(mem, opcode),
            0x16 => return self.op_param_8_bit(mem, opcode),
            0x18 => return self.op_param_8_bit(mem, opcode),
            0x1E => return self.op_param_8_bit(mem, opcode),
            0x26 => return self.op_param_8_bit(mem, opcode),
            0x2E => return self.op_param_8_bit(mem, opcode),
            0x3E => return self.op_param_8_bit(mem, opcode),
            0x20 => return self.op_param_8_bit(mem, opcode),
            0x28 => return self.op_param_8_bit(mem, opcode),
            0x30 => return self.op_param_8_bit(mem, opcode),
            0x36 => return self.op_param_8_bit(mem, opcode),
            0x38 => return self.op_param_8_bit(mem, opcode),
            0xE0 => return self.op_param_8_bit(mem, opcode),
            0xE6 => return self.op_param_8_bit(mem, opcode),
            0xF0 => return self.op_param_8_bit(mem, opcode),
            0xFE => return self.op_param_8_bit(mem, opcode),

            0x3C => inc_a!(self),
            0x04 => inc_b!(self),
            0x0C => inc_c!(self),
            0x14 => inc_d!(self),
            0x1C => inc_e!(self),
            0x24 => inc_h!(self),
            0x2C => inc_l!(self),
            0x34 => inc_hl_val!(self, mem),

            0xBE => cp_hl_val!(self, mem),
            0xBF => cp_a!(self),
            0xB8 => cp_b!(self),
            0xB9 => cp_c!(self),
            0xBA => cp_d!(self),
            0xBB => cp_e!(self),
            0xBC => cp_h!(self),
            0xBD => cp_l!(self),

            0x35 => dec_hl_val!(self, mem),
            0x3D => dec_a!(self),
            0x05 => dec_b!(self),
            0x0D => dec_c!(self),
            0x15 => dec_d!(self),
            0x1D => dec_e!(self),
            0x25 => dec_h!(self),
            0x2D => dec_l!(self),

            0x87 => add_a_a!(self),
            0x80 => add_a_b!(self),
            0x81 => add_a_c!(self),
            0x82 => add_a_d!(self),
            0x83 => add_a_e!(self),
            0x84 => add_a_h!(self),
            0x85 => add_a_l!(self),
            0x86 => add_a_hl_val!(self, mem),

            0x09 => add_hl_bc!(self),
            0x19 => add_hl_de!(self),
            0x29 => add_hl_hl!(self),
            0x39 => add_hl_sp!(self),

            0x03 => inc_bc!(self),
            0x13 => inc_de!(self),
            0x23 => inc_hl!(self),
            0x33 => inc_sp!(self),

            0x0B => dec_bc!(self),
            0x1B => dec_de!(self),
            0x2B => dec_hl!(self),
            0x3B => dec_sp!(self),

            0xCB37 => swap_a!(self),
            0xCB30 => swap_b!(self),
            0xCB31 => swap_c!(self),
            0xCB32 => swap_d!(self),
            0xCB33 => swap_e!(self),
            0xCB34 => swap_h!(self),
            0xCB35 => swap_h!(self),
            0xCB36 => swap_hl_val!(self, mem),

            0x27 => daa!(self),
            0x2F => cpl!(self),

            0x37 => ccf!(self),
            0x3F => scf!(self),

            // RST nn
            0xC7 => rst_nn!(self, 0x00),
            0xCF => rst_nn!(self, 0x08),
            0xD7 => rst_nn!(self, 0x10),
            0xDF => rst_nn!(self, 0x18),
            0xE7 => rst_nn!(self, 0x20),
            0xEF => rst_nn!(self, 0x28),
            0xF7 => rst_nn!(self, 0x30),
            0xFF => rst_nn!(self, 0x38),

            0x7F => ld_a_a!(self),
            0x78 => ld_a_b!(self),
            0x79 => ld_a_c!(self),
            0x7A => ld_a_d!(self),
            0x7B => ld_a_e!(self),
            0x7C => ld_a_h!(self),
            0x7D => ld_a_l!(self),
            0x7E => ld_a_hl_val!(self, mem),
            0x47 => ld_b_a!(self),
            0x40 => ld_b_b!(self),
            0x41 => ld_b_c!(self),
            0x42 => ld_b_d!(self),
            0x43 => ld_b_e!(self),
            0x44 => ld_b_h!(self),
            0x45 => ld_b_l!(self),
            0x46 => ld_b_hl_val!(self, mem),
            0x4F => ld_c_a!(self),
            0x48 => ld_c_b!(self),
            0x49 => ld_c_c!(self),
            0x4A => ld_c_d!(self),
            0x4B => ld_c_e!(self),
            0x4C => ld_c_h!(self),
            0x4D => ld_c_l!(self),
            0x4E => ld_c_hl_val!(self, mem),
            0x57 => ld_d_a!(self),
            0x50 => ld_d_b!(self),
            0x51 => ld_d_c!(self),
            0x52 => ld_d_d!(self),
            0x53 => ld_d_e!(self),
            0x54 => ld_d_h!(self),
            0x55 => ld_d_l!(self),
            0x56 => ld_d_hl_val!(self, mem),
            0x5F => ld_e_a!(self),
            0x58 => ld_e_b!(self),
            0x59 => ld_e_c!(self),
            0x5A => ld_e_d!(self),
            0x5B => ld_e_e!(self),
            0x5C => ld_e_h!(self),
            0x5D => ld_e_l!(self),
            0x5E => ld_e_hl_val!(self, mem),
            0x67 => ld_h_a!(self),
            0x60 => ld_h_b!(self),
            0x61 => ld_h_c!(self),
            0x62 => ld_h_d!(self),
            0x63 => ld_h_e!(self),
            0x64 => ld_h_h!(self),
            0x65 => ld_h_l!(self),
            0x66 => ld_h_hl_val!(self, mem),
            0x6F => ld_l_a!(self),
            0x68 => ld_l_b!(self),
            0x69 => ld_l_c!(self),
            0x6A => ld_l_d!(self),
            0x6B => ld_l_e!(self),
            0x6C => ld_l_h!(self),
            0x6D => ld_l_l!(self),
            0x6E => ld_l_hl_val!(self, mem),

            0x0A => ld_a_bc_val!(self, mem),
            0x1A => ld_a_de_val!(self, mem),

            0x77 => ld_hl_val_a!(self, mem),
            0x70 => ld_hl_val_b!(self, mem),
            0x71 => ld_hl_val_c!(self, mem),
            0x72 => ld_hl_val_d!(self, mem),
            0x73 => ld_hl_val_e!(self, mem),
            0x74 => ld_hl_val_h!(self, mem),
            0x75 => ld_hl_val_l!(self, mem),
            0x02 => ld_bc_val_a!(self, mem),
            0x12 => ld_de_val_a!(self, mem),

            0xA7 => and_a!(self),
            0xA0 => and_b!(self),
            0xA1 => and_c!(self),
            0xA2 => and_d!(self),
            0xA3 => and_e!(self),
            0xA4 => and_h!(self),
            0xA5 => and_l!(self),
            0xA6 => and_hl_val!(self, mem),

            0xB7 => or_a!(self),
            0xB0 => or_b!(self),
            0xB1 => or_c!(self),
            0xB2 => or_d!(self),
            0xB3 => or_e!(self),
            0xB4 => or_h!(self),
            0xB5 => or_l!(self),
            0xB6 => or_hl_val!(self, mem),

            0xAF => xor_a!(self),
            0xA8 => xor_b!(self),
            0xA9 => xor_c!(self),
            0xAA => xor_d!(self),
            0xAB => xor_e!(self),
            0xAC => xor_h!(self),
            0xAD => xor_l!(self),
            0xAE => xor_hl_val!(self, mem),

            0xF5 => push_af!(self, mem),
            0xC5 => push_bc!(self, mem),
            0xD5 => push_de!(self, mem),
            0xE5 => push_hl!(self, mem),

            0x97 => sub_a_a!(self),
            0x90 => sub_a_b!(self),
            0x91 => sub_a_c!(self),
            0x92 => sub_a_d!(self),
            0x93 => sub_a_e!(self),
            0x94 => sub_a_h!(self),
            0x95 => sub_a_l!(self),
            0x96 => sub_a_hl_val!(self, mem),

            0xF9 => ld_sp_hl!(self),

            0xE2 => ld_c_val_a!(self, mem),
            0xF2 => ld_a_c_val!(self, mem),

            0x22 => ldi_hl_a!(self, mem),
            0x2A => ldi_a_hl!(self, mem),

            0x32 => ldd_hl_a!(self, mem),
            0x3A => ldd_a_hl!(self, mem),

            0xF1 => pop_af!(self, mem),
            0xC1 => pop_bc!(self, mem),
            0xD1 => pop_de!(self, mem),
            0xE1 => pop_hl!(self, mem),

            // RET
            0xC9 => ret!(self, mem),
            0xC0 => ret_nz!(self, mem),
            0xC8 => ret_z!(self, mem),
            0xD0 => ret_nc!(self, mem),
            0xD8 => ret_c!(self, mem),

            0x8F => adc_a_a!(self),
            0x88 => adc_a_b!(self),
            0x89 => adc_a_c!(self),
            0x8A => adc_a_d!(self),
            0x8B => adc_a_e!(self),
            0x8C => adc_a_h!(self),
            0x8D => adc_a_l!(self),
            0x8E => adc_a_hl_val!(self, mem),

            0x9F => sbc_a_a!(self),
            0x98 => sbc_a_b!(self),
            0x99 => sbc_a_c!(self),
            0x9A => sbc_a_d!(self),
            0x9B => sbc_a_e!(self),
            0x9C => sbc_a_h!(self),
            0x9D => sbc_a_l!(self),
            0x9E => sbc_a_hl_val!(self, mem),


            0x07 => rlca!(self),
            0x17 => rla!(self),

            0x0F => rrca!(self),
            0x1F => rra!(self),

            0xCB17 => rl_a!(self),
            0xCB10 => rl_b!(self),
            0xCB11 => rl_c!(self),
            0xCB12 => rl_d!(self),
            0xCB13 => rl_e!(self),
            0xCB14 => rl_h!(self),
            0xCB15 => rl_l!(self),

            0xCB1F => rr_a!(self),
            0xCB18 => rr_b!(self),
            0xCB19 => rr_c!(self),
            0xCB1A => rr_d!(self),
            0xCB1B => rr_e!(self),
            0xCB1C => rr_h!(self),
            0xCB1D => rr_l!(self),

            0xCB7C => bit_7_h!(self),

            _ => { println!("opcode dispatch broke :( opcode: {:x}", opcode); return 1; },
        }
    }

    fn op_param_16_bit(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        let param = self.get_param_16_bit(mem);
        match opcode {
            0x01 => ld_bc_param!(self, param),
            0x11 => ld_de_param!(self, param),
            0x21 => ld_hl_param!(self, param),
            0x31 => ld_sp_param!(self, param),

            0xEA => ld_nn_val_a!(self, mem, param),

            0x08 => ld_param_val_sp!(self, mem, param),

            // CALL
            0xCD => call_nn!(self, mem, param),

            // JUMP
            0xC3 => jp_nn!(self,  param),

            0xC2 => jp_nz_nn!(self,  param),
            0xCA => jp_z_nn!(self,  param),
            0xD2 => jp_nc_nn!(self,  param),
            0xDA => jp_c_nn!(self,  param),

            0xFA => ld_a_nn_val!(self, mem, param),

            _ => { println!("opcode dispatched to 16 bit param executer but that didnt match the op"); return 1; },
        };
    }

    fn op_param_8_bit(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        let param = self.get_param_8_bit(mem);
        match opcode {
            0x3E => ld_a_param!(self, param),
            0x06 => ld_b_param!(self, param),
            0x0E => ld_c_param!(self, param),
            0x16 => ld_d_param!(self, param),
            0x1E => ld_e_param!(self, param),
            0x26 => ld_h_param!(self, param),
            0x2E => ld_l_param!(self, param),

            0x36 => ld_hl_val_n!(self, mem, param),
            0xE0 => ld_n_val_a!(self, mem, param),
            0xF0 => ld_a_n_val!(self, mem, param),

            0xE6 => and_param!(self, param),
            0xC6 => add_a_param!(self, param),

            0xD6 => sub_a_param!(self, param),

            0xCE => adc_a_param!(self, param),

            0xDE => sbc_a_param!(self, param),

            0xFE => cp_n!(self, param),
            // Jumps
            0x18 => jr_n!(self, param),

            0x20 => jr_nz_n!(self, param),
            0x28 => jr_z_n!(self, param),
            0x30 => jr_nc_n!(self, param),
            0x38 => jr_c_n!(self, param),

            0xF8 => ldhl_sp_n!(self, param),
            _ => { println!("opcode dispatched to 8 bit param executer but that didnt match the op"); return 1; },
        };
    }

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
        // 0x06 arg
        mem.set_mem_u8(0, 0x01);
        // 0x0E arg
        mem.set_mem_u8(1, 0b10);
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
        mem.set_mem_u8(0, 0xCB);
        mem.set_mem_u8(1, 0x10);
        mem.set_mem_u8(2, 0x20);

        set_pc!(self, 0);
        assert_eq!(self.get_opcode(mem), 0xCB10);
        assert_eq!(get_pc!(self), 2);
        assert_eq!(self.get_opcode(mem), 0x20);
        assert_eq!(get_pc!(self), 3);
    }

    pub fn test_get_param(&mut self, mem: &mut Mmu) {
        mem.set_mem_u8(3, 0x10);
        mem.set_mem_u8(4, 0x20);
        mem.set_mem_u8(5, 0x30);

        set_pc!(self, 3);
        assert_eq!(self.get_param_8_bit(mem), 0x10);
        assert_eq!(self.get_param_16_bit(mem), 0x3020);
        assert_eq!(get_pc!(self), 6);
    }

    pub fn test_stack(&mut self, mem: &mut Mmu) {
        set_sp!(self, 0xFFFE);
        mem.push_u8(get_sp_mut!(self), 10);
        mem.push_u8(get_sp_mut!(self), 20);
        mem.push_u16(get_sp_mut!(self), 0x3020);

        assert_eq!(get_sp!(self), 0xFFFA);
        assert_eq!(mem.pop_u16(get_sp_mut!(self)), 0x3020);
        assert_eq!(mem.pop_u8(get_sp_mut!(self)), 20);
        assert_eq!(mem.pop_u8(get_sp_mut!(self)), 10);
    }

    // Exec the next instruction and return how many machine cycles it takes (cycles/4)
    pub fn exec_next(&mut self, mem: &mut Mmu) -> u8 {
        if cfg!(debug_assertions = true) {
            println!("a: {:x}", get_a!(self));
            println!("pc: {:x}", get_pc!(self));
            println!("sp: {:x}", get_sp!(self));
        }

        let opcode = self.get_opcode(mem);

        if cfg!(debug_assertions = true) {
            println!("opcode: {:x}", opcode);
        }

        return self.exec_dispatcher(mem, opcode);
    }
}

#[test]
fn test_registers() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_registers();
}

#[test]
fn test_opcodes() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new();
    derp.test_opcodes(&mut mem);
}

#[test]
fn test_flag_bits() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_flag_bits();
}

#[test]
fn test_get_opcode() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new();
    derp.test_get_opcode(&mut mem);
}

#[test]
fn test_get_param() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new();
    derp.test_get_param(&mut mem);
}

#[test]
fn test_stack() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    let mut mem = Mmu::new();
    derp.test_stack(&mut mem);
}
