#[macro_use]
mod ops;

#[macro_use]
mod registers;

#[cfg(test)]
mod tests;

use super::mmu::Mmu;

pub struct Cpu {
    af: registers::Reg16Bit,
    bc: registers::Reg16Bit,
    de: registers::Reg16Bit,
    hl: registers::Reg16Bit,
    sp: registers::Reg16Bit,
    pc: registers::Reg16Bit,
    // Interrupt master enable
    ime: u8,
    halt: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        let af = registers::Reg16Bit{ whole: 0 };
        let bc = registers::Reg16Bit{ whole: 0 };
        let de = registers::Reg16Bit{ whole: 0 };
        let hl = registers::Reg16Bit{ whole: 0 };
        let sp = registers::Reg16Bit{ whole: 0 };
        let pc = registers::Reg16Bit{ whole: 0 };

        Cpu {af, bc, de, hl, sp, pc, ime: 0, halt: 0}
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

            // CALL nn
            0xCD => return self.op_param_16_bit(mem, opcode),

            // CALL cc,nn
            0xC4 => return self.op_param_16_bit(mem, opcode),
            0xCC => return self.op_param_16_bit(mem, opcode),
            0xD4 => return self.op_param_16_bit(mem, opcode),
            0xDC => return self.op_param_16_bit(mem, opcode),

            // JP nn
            0xC3 => return self.op_param_16_bit(mem, opcode),

            // JP cc,nn
            0xC2 => return self.op_param_16_bit(mem, opcode),
            0xCA => return self.op_param_16_bit(mem, opcode),
            0xD2 => return self.op_param_16_bit(mem, opcode),
            0xDA => return self.op_param_16_bit(mem, opcode),

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
            0xC6 => return self.op_param_8_bit(mem, opcode),
            0xE0 => return self.op_param_8_bit(mem, opcode),
            0xE6 => return self.op_param_8_bit(mem, opcode),
            0xEE => return self.op_param_8_bit(mem, opcode),
            0xF0 => return self.op_param_8_bit(mem, opcode),
            0xF6 => return self.op_param_8_bit(mem, opcode),
            0xFE => return self.op_param_8_bit(mem, opcode),

            0xE8 => return self.op_param_8_bit(mem, opcode),
            0x10 => return self.op_param_8_bit(mem, opcode),

            0x3C => return inc_a!(self),
            0x04 => return inc_b!(self),
            0x0C => return inc_c!(self),
            0x14 => return inc_d!(self),
            0x1C => return inc_e!(self),
            0x24 => return inc_h!(self),
            0x2C => return inc_l!(self),
            0x34 => return inc_hl_val!(self, mem),

            0xBE => return cp_hl_val!(self, mem),
            0xBF => return cp_a!(self),
            0xB8 => return cp_b!(self),
            0xB9 => return cp_c!(self),
            0xBA => return cp_d!(self),
            0xBB => return cp_e!(self),
            0xBC => return cp_h!(self),
            0xBD => return cp_l!(self),

            0x35 => return dec_hl_val!(self, mem),
            0x3D => return dec_a!(self),
            0x05 => return dec_b!(self),
            0x0D => return dec_c!(self),
            0x15 => return dec_d!(self),
            0x1D => return dec_e!(self),
            0x25 => return dec_h!(self),
            0x2D => return dec_l!(self),

            0x87 => return add_a_a!(self),
            0x80 => return add_a_b!(self),
            0x81 => return add_a_c!(self),
            0x82 => return add_a_d!(self),
            0x83 => return add_a_e!(self),
            0x84 => return add_a_h!(self),
            0x85 => return add_a_l!(self),
            0x86 => return add_a_hl_val!(self, mem),

            0x09 => return add_hl_bc!(self),
            0x19 => return add_hl_de!(self),
            0x29 => return add_hl_hl!(self),
            0x39 => return add_hl_sp!(self),

            0x03 => return inc_bc!(self),
            0x13 => return inc_de!(self),
            0x23 => return inc_hl!(self),
            0x33 => return inc_sp!(self),

            0x0B => return dec_bc!(self),
            0x1B => return dec_de!(self),
            0x2B => return dec_hl!(self),
            0x3B => return dec_sp!(self),

            0xCB37 => return swap_a!(self),
            0xCB30 => return swap_b!(self),
            0xCB31 => return swap_c!(self),
            0xCB32 => return swap_d!(self),
            0xCB33 => return swap_e!(self),
            0xCB34 => return swap_h!(self),
            0xCB35 => return swap_h!(self),
            0xCB36 => return swap_hl_val!(self, mem),

            0x27 => return daa!(self),
            0x2F => return cpl!(self),

            0x37 => return ccf!(self),
            0x3F => return scf!(self),

            // JP (HL)
            0xE9 => return jp_hl!(self, param),

            // RST nn
            0xC7 => return rst_nn!(self, 0x00),
            0xCF => return rst_nn!(self, 0x08),
            0xD7 => return rst_nn!(self, 0x10),
            0xDF => return rst_nn!(self, 0x18),
            0xE7 => return rst_nn!(self, 0x20),
            0xEF => return rst_nn!(self, 0x28),
            0xF7 => return rst_nn!(self, 0x30),
            0xFF => return rst_nn!(self, 0x38),

            0x7F => return ld_a_a!(self),
            0x78 => return ld_a_b!(self),
            0x79 => return ld_a_c!(self),
            0x7A => return ld_a_d!(self),
            0x7B => return ld_a_e!(self),
            0x7C => return ld_a_h!(self),
            0x7D => return ld_a_l!(self),
            0x7E => return ld_a_hl_val!(self, mem),
            0x47 => return ld_b_a!(self),
            0x40 => return ld_b_b!(self),
            0x41 => return ld_b_c!(self),
            0x42 => return ld_b_d!(self),
            0x43 => return ld_b_e!(self),
            0x44 => return ld_b_h!(self),
            0x45 => return ld_b_l!(self),
            0x46 => return ld_b_hl_val!(self, mem),
            0x4F => return ld_c_a!(self),
            0x48 => return ld_c_b!(self),
            0x49 => return ld_c_c!(self),
            0x4A => return ld_c_d!(self),
            0x4B => return ld_c_e!(self),
            0x4C => return ld_c_h!(self),
            0x4D => return ld_c_l!(self),
            0x4E => return ld_c_hl_val!(self, mem),
            0x57 => return ld_d_a!(self),
            0x50 => return ld_d_b!(self),
            0x51 => return ld_d_c!(self),
            0x52 => return ld_d_d!(self),
            0x53 => return ld_d_e!(self),
            0x54 => return ld_d_h!(self),
            0x55 => return ld_d_l!(self),
            0x56 => return ld_d_hl_val!(self, mem),
            0x5F => return ld_e_a!(self),
            0x58 => return ld_e_b!(self),
            0x59 => return ld_e_c!(self),
            0x5A => return ld_e_d!(self),
            0x5B => return ld_e_e!(self),
            0x5C => return ld_e_h!(self),
            0x5D => return ld_e_l!(self),
            0x5E => return ld_e_hl_val!(self, mem),
            0x67 => return ld_h_a!(self),
            0x60 => return ld_h_b!(self),
            0x61 => return ld_h_c!(self),
            0x62 => return ld_h_d!(self),
            0x63 => return ld_h_e!(self),
            0x64 => return ld_h_h!(self),
            0x65 => return ld_h_l!(self),
            0x66 => return ld_h_hl_val!(self, mem),
            0x6F => return ld_l_a!(self),
            0x68 => return ld_l_b!(self),
            0x69 => return ld_l_c!(self),
            0x6A => return ld_l_d!(self),
            0x6B => return ld_l_e!(self),
            0x6C => return ld_l_h!(self),
            0x6D => return ld_l_l!(self),
            0x6E => return ld_l_hl_val!(self, mem),

            0x0A => return ld_a_bc_val!(self, mem),
            0x1A => return ld_a_de_val!(self, mem),

            0x77 => return ld_hl_val_a!(self, mem),
            0x70 => return ld_hl_val_b!(self, mem),
            0x71 => return ld_hl_val_c!(self, mem),
            0x72 => return ld_hl_val_d!(self, mem),
            0x73 => return ld_hl_val_e!(self, mem),
            0x74 => return ld_hl_val_h!(self, mem),
            0x75 => return ld_hl_val_l!(self, mem),
            0x02 => return ld_bc_val_a!(self, mem),
            0x12 => return ld_de_val_a!(self, mem),

            0xA7 => return and_a!(self),
            0xA0 => return and_b!(self),
            0xA1 => return and_c!(self),
            0xA2 => return and_d!(self),
            0xA3 => return and_e!(self),
            0xA4 => return and_h!(self),
            0xA5 => return and_l!(self),
            0xA6 => return and_hl_val!(self, mem),

            0xB7 => return or_a!(self),
            0xB0 => return or_b!(self),
            0xB1 => return or_c!(self),
            0xB2 => return or_d!(self),
            0xB3 => return or_e!(self),
            0xB4 => return or_h!(self),
            0xB5 => return or_l!(self),
            0xB6 => return or_hl_val!(self, mem),

            0xAF => return xor_a!(self),
            0xA8 => return xor_b!(self),
            0xA9 => return xor_c!(self),
            0xAA => return xor_d!(self),
            0xAB => return xor_e!(self),
            0xAC => return xor_h!(self),
            0xAD => return xor_l!(self),
            0xAE => return xor_hl_val!(self, mem),

            0xF5 => return push_af!(self, mem),
            0xC5 => return push_bc!(self, mem),
            0xD5 => return push_de!(self, mem),
            0xE5 => return push_hl!(self, mem),

            0x97 => return sub_a_a!(self),
            0x90 => return sub_a_b!(self),
            0x91 => return sub_a_c!(self),
            0x92 => return sub_a_d!(self),
            0x93 => return sub_a_e!(self),
            0x94 => return sub_a_h!(self),
            0x95 => return sub_a_l!(self),
            0x96 => return sub_a_hl_val!(self, mem),

            0xF9 => return ld_sp_hl!(self),

            0xE2 => return ld_c_val_a!(self, mem),
            0xF2 => return ld_a_c_val!(self, mem),

            0x22 => return ldi_hl_a!(self, mem),
            0x2A => return ldi_a_hl!(self, mem),

            0x32 => return ldd_hl_a!(self, mem),
            0x3A => return ldd_a_hl!(self, mem),

            0xF1 => return pop_af!(self, mem),
            0xC1 => return pop_bc!(self, mem),
            0xD1 => return pop_de!(self, mem),
            0xE1 => return pop_hl!(self, mem),

            // RET
            0xC9 => return ret!(self, mem),
            0xC0 => return ret_nz!(self, mem),
            0xC8 => return ret_z!(self, mem),
            0xD0 => return ret_nc!(self, mem),
            0xD8 => return ret_c!(self, mem),

            0x8F => return adc_a_a!(self),
            0x88 => return adc_a_b!(self),
            0x89 => return adc_a_c!(self),
            0x8A => return adc_a_d!(self),
            0x8B => return adc_a_e!(self),
            0x8C => return adc_a_h!(self),
            0x8D => return adc_a_l!(self),
            0x8E => return adc_a_hl_val!(self, mem),

            0x9F => return sbc_a_a!(self),
            0x98 => return sbc_a_b!(self),
            0x99 => return sbc_a_c!(self),
            0x9A => return sbc_a_d!(self),
            0x9B => return sbc_a_e!(self),
            0x9C => return sbc_a_h!(self),
            0x9D => return sbc_a_l!(self),
            0x9E => return sbc_a_hl_val!(self, mem),

            // Interrupt disabling and enabling
            0xF3 => return di!(self),
            0xFB => return ei!(self),

            0xD9 => return reti!(self, mem),

            0x07 => return rlca!(self),
            0x17 => return rla!(self),

            0x0F => return rrca!(self),
            0x1F => return rra!(self),

            // HALT
            0x76 => return halt!(self),

            0xCB17 => return rl_a!(self),
            0xCB10 => return rl_b!(self),
            0xCB11 => return rl_c!(self),
            0xCB12 => return rl_d!(self),
            0xCB13 => return rl_e!(self),
            0xCB14 => return rl_h!(self),
            0xCB15 => return rl_l!(self),

            0xCB1F => return rr_a!(self),
            0xCB18 => return rr_b!(self),
            0xCB19 => return rr_c!(self),
            0xCB1A => return rr_d!(self),
            0xCB1B => return rr_e!(self),
            0xCB1C => return rr_h!(self),
            0xCB1D => return rr_l!(self),

            0xCB7C => return bit_7_h!(self),

            0xCB87 => return res_7_a!(self),

            _ => { println!("opcode dispatch broke :( opcode: {:x}", opcode); return 1; },
        }
    }

    fn op_param_16_bit(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        let param = self.get_param_16_bit(mem);
        match opcode {
            0x01 => return ld_bc_param!(self, param),
            0x11 => return ld_de_param!(self, param),
            0x21 => return ld_hl_param!(self, param),
            0x31 => return ld_sp_param!(self, param),

            0xEA => return ld_nn_val_a!(self, mem, param),

            0x08 => return ld_param_val_sp!(self, mem, param),

            // CALL
            0xCD => return call_nn!(self, mem, param),

            0xC4 => return call_nz_nn!(self, mem, param),
            0xCC => return call_z_nn!(self, mem, param),
            0xD4 => return call_nc_nn!(self, mem, param),
            0xDC => return call_c_nn!(self, mem, param),

            // JUMP
            0xC3 => return jp_nn!(self,  param),

            0xC2 => return jp_nz_nn!(self,  param),
            0xCA => return jp_z_nn!(self,  param),
            0xD2 => return jp_nc_nn!(self,  param),
            0xDA => return jp_c_nn!(self,  param),

            0xFA => return ld_a_nn_val!(self, mem, param),

            _ => { println!("opcode dispatched to 16 bit param executer but that didnt match the op"); return 1; },
        };
    }

    fn op_param_8_bit(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        let param = self.get_param_8_bit(mem);
        match opcode {
            0x3E => return ld_a_param!(self, param),
            0x06 => return ld_b_param!(self, param),
            0x0E => return ld_c_param!(self, param),
            0x16 => return ld_d_param!(self, param),
            0x1E => return ld_e_param!(self, param),
            0x26 => return ld_h_param!(self, param),
            0x2E => return ld_l_param!(self, param),

            0x36 => return ld_hl_val_n!(self, mem, param),
            0xE0 => return ld_n_val_a!(self, mem, param),
            0xEE => return xor_param!(self, param),
            0xF6 => return or_param!(self, param),
            0xF0 => return ld_a_n_val!(self, mem, param),

            0xE6 => return and_param!(self, param),
            0xC6 => return add_a_param!(self, param),

            0xE8 => return add_sp_param!(self, param),

            0xD6 => return sub_a_param!(self, param),

            0xCE => return adc_a_param!(self, param),

            0xDE => return sbc_a_param!(self, param),

            0xFE => return cp_n!(self, param),

            0x10 => return stop!(self),
            // Jumps
            0x18 => return jr_n!(self, param),

            0x20 => return jr_nz_n!(self, param),
            0x28 => return jr_z_n!(self, param),
            0x30 => return jr_nc_n!(self, param),
            0x38 => return jr_c_n!(self, param),

            0xF8 => return ldhl_sp_n!(self, param),
            _ => { println!("opcode dispatched to 8 bit param executer but that didnt match the op"); return 1; },
        };
    }

    // Exec the next instruction and return how many machine cycles it takes (cycles/4)
    pub fn exec_next(&mut self, mem: &mut Mmu) -> u8 {
        if cfg!(debug_assertions = true) {
            println!("a: {:x}", get_a!(self));
            println!("pc: {:x}", get_pc!(self));
            println!("sp: {:x}", get_sp!(self));
            println!("hl: {:x}", get_hl!(self));
        }

        if self.ime == 1 {
            let val = self.handle_int(mem);
            if val != 0 {
                return val;
            }
        }

        // If we're not halted run the next instruction
        // otherwise burn a cycle
        if self.halt == 0 {
            let opcode = self.get_opcode(mem);

            if cfg!(debug_assertions = true) {
                println!("opcode: {:x}\n", opcode);
            }

            return self.exec_dispatcher(mem, opcode);
        }
        return 1;
    }

    // Handle interrupts
    pub fn handle_int(&mut self, mem: &mut Mmu) -> u8 {
        let interrupts = mem.get_mem_u8(0xFFFF) & mem.get_mem_u8(0xFF0F);
        if cfg!(debug_assertions = true) {
            println!("interrupts: {:b}", interrupts);
        }

        if interrupts != 0 {
            self.halt = 0;
            let mut addr_to_call: u16 = 0;
            if interrupts & 0b00000001 == 0b00000001 {
                addr_to_call = 0x40;
                mem.set_mem_u8(0xFF0F, interrupts - 0b00000001);
            }
            else if interrupts & 0b00000010 == 0b00000010 {
                addr_to_call = 0x48;
                mem.set_mem_u8(0xFF0F, interrupts - 0b00000010);
            }
            else if interrupts & 0b00000100 == 0b00000100 {
                addr_to_call = 0x50;
                mem.set_mem_u8(0xFF0F, interrupts - 0b00000100);
            }
            else if interrupts & 0b00001000 == 0b00001000 {
                addr_to_call = 0x58;
                mem.set_mem_u8(0xFF0F, interrupts - 0b00001000);
            }
            else if interrupts & 0b00010000 == 0b00010000 {
                addr_to_call = 0x60;
                mem.set_mem_u8(0xFF0F, interrupts - 0b00010000);
            }

            println!("handling interrupt, jump to: {:x}", addr_to_call);

            // Disable interrupts
            self.ime = 0;

            // Push current PC
            mem.push_u16(get_sp_mut!(self), get_pc!(self));

            // Jump
            set_pc!(self, addr_to_call);
            return 4;
        }
        return 0;
    }
}
