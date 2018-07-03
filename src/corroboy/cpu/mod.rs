#[macro_use]
mod ops;
use self::ops::*;

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
    halt: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        let af = registers::Reg16Bit { whole: 0 };
        let bc = registers::Reg16Bit { whole: 0 };
        let de = registers::Reg16Bit { whole: 0 };
        let hl = registers::Reg16Bit { whole: 0 };
        let sp = registers::Reg16Bit { whole: 0 };
        let pc = registers::Reg16Bit { whole: 0 };

        Cpu {
            af,
            bc,
            de,
            hl,
            sp,
            pc,
            ime: 0,
            halt: 0,
        }
    }

    // functions for debugging
    pub fn print_regs(&self) {
        println!("AF: 0x{:04x}", get_af!(self));
        println!("BC: 0x{:04x}", get_bc!(self));
        println!("DE: 0x{:04x}", get_de!(self));
        println!("HL: 0x{:04x}", get_hl!(self));
        println!("sp: 0x{:04x}", get_sp!(self));
        println!("pc: 0x{:04x}", get_pc!(self));
    }

    pub fn print_flags(&self) {
        println!("Z: {}", get_z_flag!(self));
        println!("N: {}", get_n_flag!(self));
        println!("H: {}", get_h_flag!(self));
        println!("C: {}", get_c_flag!(self));
    }

    pub fn get_pc(&self) -> u16 {
        get_pc!(self)
    }

    /// Get opcode for the next instruction fo execute
    fn get_opcode(&mut self, mem: &mut Mmu) -> u16 {
        let mut pc = get_pc!(self);
        let temp = mem.get_mem_u8(pc as usize);
        pc += 1;
        let temp_u16: u16;
        // If the opcode starts with CB get the next byte as well as CB
        // indicates a 2 byte opcode
        if temp == 0xCB {
            temp_u16 = 0xCB00 + (mem.get_mem_u8(pc as usize) as u16);
            pc += 1;
        } else {
            temp_u16 = temp as u16;
        }

        set_pc!(self, pc);

        temp_u16
    }

    /// Get an 8 bit parameter for an instruction
    fn get_param_8_bit(&mut self, mem: &mut Mmu) -> u8 {
        let temp = mem.get_mem_u8(get_pc!(self) as usize);
        set_pc!(self, get_pc!(self) + 1);
        if cfg!(debug_assertions = true) {
            println!(
                "8 bit param: b: {:b}, hex: {:x}, signed: {:x}",
                temp,
                temp,
                (temp as i8) as i16
            );
        }
        temp
    }

    /// Get an 16 bit parameter for an instruction
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

    /// Execute the opcode passed as an argument
    fn exec_dispatcher(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        match opcode {
            0x00 => return 1,
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
            0xCE => return self.op_param_8_bit(mem, opcode),
            0xD6 => return self.op_param_8_bit(mem, opcode),
            0xDE => return self.op_param_8_bit(mem, opcode),
            0xE0 => return self.op_param_8_bit(mem, opcode),
            0xE6 => return self.op_param_8_bit(mem, opcode),
            0xE8 => return self.op_param_8_bit(mem, opcode),
            0xEE => return self.op_param_8_bit(mem, opcode),
            0xF0 => return self.op_param_8_bit(mem, opcode),
            0xF6 => return self.op_param_8_bit(mem, opcode),
            0xF8 => return self.op_param_8_bit(mem, opcode),
            0xFE => return self.op_param_8_bit(mem, opcode),

            0x10 => return self.op_param_8_bit(mem, opcode),

            0x3C => {
                inc(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x04 => {
                inc(get_mut_b!(self), get_mut_f!(self));
                return 1;
            }
            0x0C => {
                inc(get_mut_c!(self), get_mut_f!(self));
                return 1;
            }
            0x14 => {
                inc(get_mut_d!(self), get_mut_f!(self));
                return 1;
            }
            0x1C => {
                inc(get_mut_e!(self), get_mut_f!(self));
                return 1;
            }
            0x24 => {
                inc(get_mut_h!(self), get_mut_f!(self));
                return 1;
            }
            0x2C => {
                inc(get_mut_l!(self), get_mut_f!(self));
                return 1;
            }
            0x34 => {
                inc_mem(mem, get_hl!(self), get_mut_f!(self));
                return 3;
            }

            0xBF => {
                cp(get_a!(self), get_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB8 => {
                cp(get_b!(self), get_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB9 => {
                cp(get_c!(self), get_a!(self), get_mut_f!(self));
                return 1;
            }
            0xBA => {
                cp(get_d!(self), get_a!(self), get_mut_f!(self));
                return 1;
            }
            0xBB => {
                cp(get_e!(self), get_a!(self), get_mut_f!(self));
                return 1;
            }
            0xBC => {
                cp(get_h!(self), get_a!(self), get_mut_f!(self));
                return 1;
            }
            0xBD => {
                cp(get_l!(self), get_a!(self), get_mut_f!(self));
                return 1;
            }
            0xBE => {
                cp(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_a!(self),
                    get_mut_f!(self),
                );
                return 2;
            }

            0x3D => {
                dec(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x05 => {
                dec(get_mut_b!(self), get_mut_f!(self));
                return 1;
            }
            0x0D => {
                dec(get_mut_c!(self), get_mut_f!(self));
                return 1;
            }
            0x15 => {
                dec(get_mut_d!(self), get_mut_f!(self));
                return 1;
            }
            0x1D => {
                dec(get_mut_e!(self), get_mut_f!(self));
                return 1;
            }
            0x25 => {
                dec(get_mut_h!(self), get_mut_f!(self));
                return 1;
            }
            0x2D => {
                dec(get_mut_l!(self), get_mut_f!(self));
                return 1;
            }
            0x35 => {
                dec_mem(mem, get_hl!(self), get_mut_f!(self));
                return 3;
            }

            0x87 => {
                add(get_a!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x80 => {
                add(get_b!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x81 => {
                add(get_c!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x82 => {
                add(get_d!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x83 => {
                add(get_e!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x84 => {
                add(get_h!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x85 => {
                add(get_l!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x86 => {
                add(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_mut_a!(self),
                    get_mut_f!(self),
                );
                return 2;
            }

            0x09 => {
                add_16bit(get_bc!(self), get_mut_hl!(self), get_mut_f!(self));
                return 2;
            }
            0x19 => {
                add_16bit(get_de!(self), get_mut_hl!(self), get_mut_f!(self));
                return 2;
            }
            0x29 => {
                add_16bit(get_hl!(self), get_mut_hl!(self), get_mut_f!(self));
                return 2;
            }
            0x39 => {
                add_16bit(get_sp!(self), get_mut_hl!(self), get_mut_f!(self));
                return 2;
            }

            0x03 => {
                inc_16bit(get_mut_bc!(self));
                return 2;
            }
            0x13 => {
                inc_16bit(get_mut_de!(self));
                return 2;
            }
            0x23 => {
                inc_16bit(get_mut_hl!(self));
                return 2;
            }
            0x33 => {
                inc_16bit(get_mut_sp!(self));
                return 2;
            }

            0x0B => {
                dec_16bit(get_mut_bc!(self));
                return 2;
            }
            0x1B => {
                dec_16bit(get_mut_de!(self));
                return 2;
            }
            0x2B => {
                dec_16bit(get_mut_hl!(self));
                return 2;
            }
            0x3B => {
                dec_16bit(get_mut_sp!(self));
                return 2;
            }

            0xCB37 => {
                swap(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB30 => {
                swap(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB31 => {
                swap(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB32 => {
                swap(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB33 => {
                swap(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB34 => {
                swap(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB35 => {
                swap(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB36 => {
                swap_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0x27 => {
                daa(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x2F => {
                cpl(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }

            0x37 => {
                scf(get_mut_f!(self));
                return 1;
            }
            0x3F => {
                ccf(get_mut_f!(self));
                return 1;
            }

            // JP (HL)
            0xE9 => {
                jp(get_hl!(self), get_mut_pc!(self));
                return 3;
            }

            // RST nn
            0xC7 => {
                rst(0x00, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }
            0xCF => {
                rst(0x08, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }
            0xD7 => {
                rst(0x10, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }
            0xDF => {
                rst(0x18, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }
            0xE7 => {
                rst(0x20, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }
            0xEF => {
                rst(0x28, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }
            0xF7 => {
                rst(0x30, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }
            0xFF => {
                rst(0x38, mem, get_mut_sp!(self), get_mut_pc!(self));
                return 8;
            }

            0x7F => {
                ld_reg(get_a!(self), get_mut_a!(self));
                return 1;
            }
            0x78 => {
                ld_reg(get_b!(self), get_mut_a!(self));
                return 1;
            }
            0x79 => {
                ld_reg(get_c!(self), get_mut_a!(self));
                return 1;
            }
            0x7A => {
                ld_reg(get_d!(self), get_mut_a!(self));
                return 1;
            }
            0x7B => {
                ld_reg(get_e!(self), get_mut_a!(self));
                return 1;
            }
            0x7C => {
                ld_reg(get_h!(self), get_mut_a!(self));
                return 1;
            }
            0x7D => {
                ld_reg(get_l!(self), get_mut_a!(self));
                return 1;
            }
            0x7E => {
                ld_reg(mem.get_mem_u8(get_hl!(self) as usize), get_mut_a!(self));
                return 2;
            }
            0x47 => {
                ld_reg(get_a!(self), get_mut_b!(self));
                return 1;
            }
            0x40 => {
                ld_reg(get_b!(self), get_mut_b!(self));
                return 1;
            }
            0x41 => {
                ld_reg(get_c!(self), get_mut_b!(self));
                return 1;
            }
            0x42 => {
                ld_reg(get_d!(self), get_mut_b!(self));
                return 1;
            }
            0x43 => {
                ld_reg(get_e!(self), get_mut_b!(self));
                return 1;
            }
            0x44 => {
                ld_reg(get_h!(self), get_mut_b!(self));
                return 1;
            }
            0x45 => {
                ld_reg(get_l!(self), get_mut_b!(self));
                return 1;
            }
            0x46 => {
                ld_reg(mem.get_mem_u8(get_hl!(self) as usize), get_mut_b!(self));
                return 2;
            }
            0x4F => {
                ld_reg(get_a!(self), get_mut_c!(self));
                return 1;
            }
            0x48 => {
                ld_reg(get_b!(self), get_mut_c!(self));
                return 1;
            }
            0x49 => {
                ld_reg(get_c!(self), get_mut_c!(self));
                return 1;
            }
            0x4A => {
                ld_reg(get_d!(self), get_mut_c!(self));
                return 1;
            }
            0x4B => {
                ld_reg(get_e!(self), get_mut_c!(self));
                return 1;
            }
            0x4C => {
                ld_reg(get_h!(self), get_mut_c!(self));
                return 1;
            }
            0x4D => {
                ld_reg(get_l!(self), get_mut_c!(self));
                return 1;
            }
            0x4E => {
                ld_reg(mem.get_mem_u8(get_hl!(self) as usize), get_mut_c!(self));
                return 2;
            }
            0x57 => {
                ld_reg(get_a!(self), get_mut_d!(self));
                return 1;
            }
            0x50 => {
                ld_reg(get_b!(self), get_mut_d!(self));
                return 1;
            }
            0x51 => {
                ld_reg(get_c!(self), get_mut_d!(self));
                return 1;
            }
            0x52 => {
                ld_reg(get_d!(self), get_mut_d!(self));
                return 1;
            }
            0x53 => {
                ld_reg(get_e!(self), get_mut_d!(self));
                return 1;
            }
            0x54 => {
                ld_reg(get_h!(self), get_mut_d!(self));
                return 1;
            }
            0x55 => {
                ld_reg(get_l!(self), get_mut_d!(self));
                return 1;
            }
            0x56 => {
                ld_reg(mem.get_mem_u8(get_hl!(self) as usize), get_mut_d!(self));
                return 2;
            }
            0x5F => {
                ld_reg(get_a!(self), get_mut_e!(self));
                return 1;
            }
            0x58 => {
                ld_reg(get_b!(self), get_mut_e!(self));
                return 1;
            }
            0x59 => {
                ld_reg(get_c!(self), get_mut_e!(self));
                return 1;
            }
            0x5A => {
                ld_reg(get_d!(self), get_mut_e!(self));
                return 1;
            }
            0x5B => {
                ld_reg(get_e!(self), get_mut_e!(self));
                return 1;
            }
            0x5C => {
                ld_reg(get_h!(self), get_mut_e!(self));
                return 1;
            }
            0x5D => {
                ld_reg(get_l!(self), get_mut_e!(self));
                return 1;
            }
            0x5E => {
                ld_reg(mem.get_mem_u8(get_hl!(self) as usize), get_mut_e!(self));
                return 2;
            }
            0x67 => {
                ld_reg(get_a!(self), get_mut_h!(self));
                return 1;
            }
            0x60 => {
                ld_reg(get_b!(self), get_mut_h!(self));
                return 1;
            }
            0x61 => {
                ld_reg(get_c!(self), get_mut_h!(self));
                return 1;
            }
            0x62 => {
                ld_reg(get_d!(self), get_mut_h!(self));
                return 1;
            }
            0x63 => {
                ld_reg(get_e!(self), get_mut_h!(self));
                return 1;
            }
            0x64 => {
                ld_reg(get_h!(self), get_mut_h!(self));
                return 1;
            }
            0x65 => {
                ld_reg(get_l!(self), get_mut_h!(self));
                return 1;
            }
            0x66 => {
                ld_reg(mem.get_mem_u8(get_hl!(self) as usize), get_mut_h!(self));
                return 2;
            }
            0x6F => {
                ld_reg(get_a!(self), get_mut_l!(self));
                return 1;
            }
            0x68 => {
                ld_reg(get_b!(self), get_mut_l!(self));
                return 1;
            }
            0x69 => {
                ld_reg(get_c!(self), get_mut_l!(self));
                return 1;
            }
            0x6A => {
                ld_reg(get_d!(self), get_mut_l!(self));
                return 1;
            }
            0x6B => {
                ld_reg(get_e!(self), get_mut_l!(self));
                return 1;
            }
            0x6C => {
                ld_reg(get_h!(self), get_mut_l!(self));
                return 1;
            }
            0x6D => {
                ld_reg(get_l!(self), get_mut_l!(self));
                return 1;
            }
            0x6E => {
                ld_reg(mem.get_mem_u8(get_hl!(self) as usize), get_mut_l!(self));
                return 2;
            }

            0x0A => {
                ld_reg(mem.get_mem_u8(get_bc!(self) as usize), get_mut_a!(self));
                return 2;
            }
            0x1A => {
                ld_reg(mem.get_mem_u8(get_de!(self) as usize), get_mut_a!(self));
                return 2;
            }

            0x77 => {
                ld_mem(get_hl!(self), mem, get_a!(self));
                return 2;
            }
            0x70 => {
                ld_mem(get_hl!(self), mem, get_b!(self));
                return 2;
            }
            0x71 => {
                ld_mem(get_hl!(self), mem, get_c!(self));
                return 2;
            }
            0x72 => {
                ld_mem(get_hl!(self), mem, get_d!(self));
                return 2;
            }
            0x73 => {
                ld_mem(get_hl!(self), mem, get_e!(self));
                return 2;
            }
            0x74 => {
                ld_mem(get_hl!(self), mem, get_h!(self));
                return 2;
            }
            0x75 => {
                ld_mem(get_hl!(self), mem, get_l!(self));
                return 2;
            }
            0x02 => {
                ld_mem(get_bc!(self), mem, get_a!(self));
                return 2;
            }
            0x12 => {
                ld_mem(get_de!(self), mem, get_a!(self));
                return 2;
            }

            0xA7 => {
                and(get_a!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA0 => {
                and(get_b!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA1 => {
                and(get_c!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA2 => {
                and(get_d!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA3 => {
                and(get_e!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA4 => {
                and(get_h!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA5 => {
                and(get_l!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA6 => {
                and(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_mut_a!(self),
                    get_mut_f!(self),
                );
                return 2;
            }

            0xB7 => {
                or(get_a!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB0 => {
                or(get_b!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB1 => {
                or(get_c!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB2 => {
                or(get_d!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB3 => {
                or(get_e!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB4 => {
                or(get_h!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB5 => {
                or(get_l!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xB6 => {
                or(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_mut_a!(self),
                    get_mut_f!(self),
                );
                return 2;
            }

            0xAF => {
                xor(get_a!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA8 => {
                xor(get_b!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xA9 => {
                xor(get_c!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xAA => {
                xor(get_d!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xAB => {
                xor(get_e!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xAC => {
                xor(get_h!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xAD => {
                xor(get_l!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0xAE => {
                xor(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_mut_a!(self),
                    get_mut_f!(self),
                );
                return 2;
            }

            0xF5 => {
                push(get_af!(self), mem, get_mut_sp!(self));
                return 4;
            }
            0xC5 => {
                push(get_bc!(self), mem, get_mut_sp!(self));
                return 4;
            }
            0xD5 => {
                push(get_de!(self), mem, get_mut_sp!(self));
                return 4;
            }
            0xE5 => {
                push(get_hl!(self), mem, get_mut_sp!(self));
                return 4;
            }

            0x97 => {
                sub(get_a!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x90 => {
                sub(get_b!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x91 => {
                sub(get_c!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x92 => {
                sub(get_d!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x93 => {
                sub(get_e!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x94 => {
                sub(get_h!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x95 => {
                sub(get_l!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x96 => {
                sub(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_mut_a!(self),
                    get_mut_f!(self),
                );
                return 1;
            }

            0xF9 => {
                ld_reg_16bit(get_hl!(self), get_mut_sp!(self));
                return 2;
            }

            0xE2 => {
                ld_mem(get_c!(self) as u16 + 0xFF00, mem, get_a!(self));
                return 2;
            }
            0xF2 => {
                ld_reg(
                    mem.get_mem_u8(get_c!(self) as usize + 0xFF00),
                    get_mut_a!(self),
                );
                return 2;
            }

            0x22 => {
                ldi_hl_a(get_mut_a!(self), get_mut_hl!(self), mem);
                return 2;
            }
            0x2A => {
                ldi_a_hl(get_mut_a!(self), get_mut_hl!(self), mem);
                return 2;
            }

            0x32 => {
                ldd_hl_a(get_mut_a!(self), get_mut_hl!(self), mem);
                return 2;
            }
            0x3A => {
                ldd_a_hl(get_mut_a!(self), get_mut_hl!(self), mem);
                return 2;
            }

            0xF1 => {
                pop_af(get_mut_af!(self), &mem, get_mut_sp!(self));
                return 3;
            }
            0xC1 => {
                pop(get_mut_bc!(self), &mem, get_mut_sp!(self));
                return 3;
            }
            0xD1 => {
                pop(get_mut_de!(self), &mem, get_mut_sp!(self));
                return 3;
            }
            0xE1 => {
                pop(get_mut_hl!(self), &mem, get_mut_sp!(self));
                return 3;
            }

            // RET
            0xC9 => {
                ret(mem, get_mut_pc!(self), get_mut_sp!(self));
                return 2;
            }
            0xC0 => {
                ret_nz(mem, get_f!(self), get_mut_pc!(self), get_mut_sp!(self));
                return 2;
            }
            0xC8 => {
                ret_z(mem, get_f!(self), get_mut_pc!(self), get_mut_sp!(self));
                return 2;
            }
            0xD0 => {
                ret_nc(mem, get_f!(self), get_mut_pc!(self), get_mut_sp!(self));
                return 2;
            }
            0xD8 => {
                ret_c(mem, get_f!(self), get_mut_pc!(self), get_mut_sp!(self));
                return 2;
            }

            0x8F => {
                adc(get_a!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x88 => {
                adc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x89 => {
                adc(get_c!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x8A => {
                adc(get_d!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x8B => {
                adc(get_e!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x8C => {
                adc(get_h!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x8D => {
                adc(get_l!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x8E => {
                adc(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_mut_a!(self),
                    get_mut_f!(self),
                );
                return 2;
            }

            0x9F => {
                sbc(get_a!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x98 => {
                sbc(get_b!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x99 => {
                sbc(get_c!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x9A => {
                sbc(get_d!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x9B => {
                sbc(get_e!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x9C => {
                sbc(get_h!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x9D => {
                sbc(get_l!(self), get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x9E => {
                sbc(
                    mem.get_mem_u8(get_hl!(self) as usize),
                    get_mut_a!(self),
                    get_mut_f!(self),
                );
                return 2;
            }

            // Interrupt disabling and enabling
            0xF3 => {
                di(&mut self.ime);
                return 1;
            }
            0xFB => {
                ei(&mut self.ime);
                return 1;
            }

            0xD9 => {
                ret(mem, get_mut_pc!(self), get_mut_sp!(self));
                ei(&mut self.ime);
                return 2;
            }

            0x07 => {
                rlca(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x17 => {
                rla(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }

            0x0F => {
                rrca(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }
            0x1F => {
                rra(get_mut_a!(self), get_mut_f!(self));
                return 1;
            }

            // HALT
            0x76 => {
                halt(&mut self.halt);
                return 1;
            }

            0xCB07 => {
                rlc_reg(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB00 => {
                rlc_reg(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB01 => {
                rlc_reg(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB02 => {
                rlc_reg(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB03 => {
                rlc_reg(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB04 => {
                rlc_reg(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB05 => {
                rlc_reg(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB06 => {
                rlc_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0xCB0F => {
                rrc_reg(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB08 => {
                rrc_reg(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB09 => {
                rrc_reg(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB0A => {
                rrc_reg(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB0B => {
                rrc_reg(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB0C => {
                rrc_reg(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB0D => {
                rrc_reg(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB0E => {
                rrc_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0xCB17 => {
                rl_reg(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB10 => {
                rl_reg(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB11 => {
                rl_reg(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB12 => {
                rl_reg(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB13 => {
                rl_reg(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB14 => {
                rl_reg(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB15 => {
                rl_reg(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB16 => {
                rl_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0xCB1F => {
                rr_reg(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB18 => {
                rr_reg(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB19 => {
                rr_reg(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB1A => {
                rr_reg(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB1B => {
                rr_reg(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB1C => {
                rr_reg(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB1D => {
                rr_reg(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB1E => {
                rr_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0xCB27 => {
                sla_reg(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB20 => {
                sla_reg(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB21 => {
                sla_reg(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB22 => {
                sla_reg(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB23 => {
                sla_reg(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB24 => {
                sla_reg(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB25 => {
                sla_reg(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB26 => {
                sla_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0xCB2F => {
                sra_reg(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB28 => {
                sra_reg(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB29 => {
                sra_reg(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB2A => {
                sra_reg(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB2B => {
                sra_reg(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB2C => {
                sra_reg(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB2D => {
                sra_reg(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB2E => {
                sra_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0xCB3F => {
                srl_reg(get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xCB38 => {
                srl_reg(get_mut_b!(self), get_mut_f!(self));
                return 2;
            }
            0xCB39 => {
                srl_reg(get_mut_c!(self), get_mut_f!(self));
                return 2;
            }
            0xCB3A => {
                srl_reg(get_mut_d!(self), get_mut_f!(self));
                return 2;
            }
            0xCB3B => {
                srl_reg(get_mut_e!(self), get_mut_f!(self));
                return 2;
            }
            0xCB3C => {
                srl_reg(get_mut_h!(self), get_mut_f!(self));
                return 2;
            }
            0xCB3D => {
                srl_reg(get_mut_l!(self), get_mut_f!(self));
                return 2;
            }
            0xCB3E => {
                srl_mem(mem, get_hl!(self), get_mut_f!(self));
                return 4;
            }

            0xCB47 => {
                bit(get_a!(self), 0, get_mut_f!(self));
                return 2;
            }
            0xCB4F => {
                bit(get_a!(self), 1, get_mut_f!(self));
                return 2;
            }
            0xCB57 => {
                bit(get_a!(self), 2, get_mut_f!(self));
                return 2;
            }
            0xCB5F => {
                bit(get_a!(self), 3, get_mut_f!(self));
                return 2;
            }
            0xCB67 => {
                bit(get_a!(self), 4, get_mut_f!(self));
                return 2;
            }
            0xCB6F => {
                bit(get_a!(self), 5, get_mut_f!(self));
                return 2;
            }
            0xCB77 => {
                bit(get_a!(self), 6, get_mut_f!(self));
                return 2;
            }
            0xCB7F => {
                bit(get_a!(self), 7, get_mut_f!(self));
                return 2;
            }

            0xCB40 => {
                bit(get_b!(self), 0, get_mut_f!(self));
                return 2;
            }
            0xCB48 => {
                bit(get_b!(self), 1, get_mut_f!(self));
                return 2;
            }
            0xCB50 => {
                bit(get_b!(self), 2, get_mut_f!(self));
                return 2;
            }
            0xCB58 => {
                bit(get_b!(self), 3, get_mut_f!(self));
                return 2;
            }
            0xCB60 => {
                bit(get_b!(self), 4, get_mut_f!(self));
                return 2;
            }
            0xCB68 => {
                bit(get_b!(self), 5, get_mut_f!(self));
                return 2;
            }
            0xCB70 => {
                bit(get_b!(self), 6, get_mut_f!(self));
                return 2;
            }
            0xCB78 => {
                bit(get_b!(self), 7, get_mut_f!(self));
                return 2;
            }

            0xCB41 => {
                bit(get_c!(self), 0, get_mut_f!(self));
                return 2;
            }
            0xCB49 => {
                bit(get_c!(self), 1, get_mut_f!(self));
                return 2;
            }
            0xCB51 => {
                bit(get_c!(self), 2, get_mut_f!(self));
                return 2;
            }
            0xCB59 => {
                bit(get_c!(self), 3, get_mut_f!(self));
                return 2;
            }
            0xCB61 => {
                bit(get_c!(self), 4, get_mut_f!(self));
                return 2;
            }
            0xCB69 => {
                bit(get_c!(self), 5, get_mut_f!(self));
                return 2;
            }
            0xCB71 => {
                bit(get_c!(self), 6, get_mut_f!(self));
                return 2;
            }
            0xCB79 => {
                bit(get_c!(self), 7, get_mut_f!(self));
                return 2;
            }

            0xCB42 => {
                bit(get_d!(self), 0, get_mut_f!(self));
                return 2;
            }
            0xCB4A => {
                bit(get_d!(self), 1, get_mut_f!(self));
                return 2;
            }
            0xCB52 => {
                bit(get_d!(self), 2, get_mut_f!(self));
                return 2;
            }
            0xCB5A => {
                bit(get_d!(self), 3, get_mut_f!(self));
                return 2;
            }
            0xCB62 => {
                bit(get_d!(self), 4, get_mut_f!(self));
                return 2;
            }
            0xCB6A => {
                bit(get_d!(self), 5, get_mut_f!(self));
                return 2;
            }
            0xCB72 => {
                bit(get_d!(self), 6, get_mut_f!(self));
                return 2;
            }
            0xCB7A => {
                bit(get_d!(self), 7, get_mut_f!(self));
                return 2;
            }

            0xCB43 => {
                bit(get_e!(self), 0, get_mut_f!(self));
                return 2;
            }
            0xCB4B => {
                bit(get_e!(self), 1, get_mut_f!(self));
                return 2;
            }
            0xCB53 => {
                bit(get_e!(self), 2, get_mut_f!(self));
                return 2;
            }
            0xCB5B => {
                bit(get_e!(self), 3, get_mut_f!(self));
                return 2;
            }
            0xCB63 => {
                bit(get_e!(self), 4, get_mut_f!(self));
                return 2;
            }
            0xCB6B => {
                bit(get_e!(self), 5, get_mut_f!(self));
                return 2;
            }
            0xCB73 => {
                bit(get_e!(self), 6, get_mut_f!(self));
                return 2;
            }
            0xCB7B => {
                bit(get_e!(self), 7, get_mut_f!(self));
                return 2;
            }

            0xCB44 => {
                bit(get_h!(self), 0, get_mut_f!(self));
                return 2;
            }
            0xCB4C => {
                bit(get_h!(self), 1, get_mut_f!(self));
                return 2;
            }
            0xCB54 => {
                bit(get_h!(self), 2, get_mut_f!(self));
                return 2;
            }
            0xCB5C => {
                bit(get_h!(self), 3, get_mut_f!(self));
                return 2;
            }
            0xCB64 => {
                bit(get_h!(self), 4, get_mut_f!(self));
                return 2;
            }
            0xCB6C => {
                bit(get_h!(self), 5, get_mut_f!(self));
                return 2;
            }
            0xCB74 => {
                bit(get_h!(self), 6, get_mut_f!(self));
                return 2;
            }
            0xCB7C => {
                bit(get_h!(self), 7, get_mut_f!(self));
                return 2;
            }

            0xCB45 => {
                bit(get_l!(self), 0, get_mut_f!(self));
                return 2;
            }
            0xCB4D => {
                bit(get_l!(self), 1, get_mut_f!(self));
                return 2;
            }
            0xCB55 => {
                bit(get_l!(self), 2, get_mut_f!(self));
                return 2;
            }
            0xCB5D => {
                bit(get_l!(self), 3, get_mut_f!(self));
                return 2;
            }
            0xCB65 => {
                bit(get_l!(self), 4, get_mut_f!(self));
                return 2;
            }
            0xCB6D => {
                bit(get_l!(self), 5, get_mut_f!(self));
                return 2;
            }
            0xCB75 => {
                bit(get_l!(self), 6, get_mut_f!(self));
                return 2;
            }
            0xCB7D => {
                bit(get_l!(self), 7, get_mut_f!(self));
                return 2;
            }

            0xCB46 => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 0, get_mut_f!(self));
                return 4;
            }
            0xCB4E => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 1, get_mut_f!(self));
                return 4;
            }
            0xCB56 => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 2, get_mut_f!(self));
                return 4;
            }
            0xCB5E => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 3, get_mut_f!(self));
                return 4;
            }
            0xCB66 => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 4, get_mut_f!(self));
                return 4;
            }
            0xCB6E => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 5, get_mut_f!(self));
                return 4;
            }
            0xCB76 => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 6, get_mut_f!(self));
                return 4;
            }
            0xCB7E => {
                bit(mem.get_mem_u8(get_hl!(self) as usize), 7, get_mut_f!(self));
                return 4;
            }

            0xCB87 => {
                res_reg(get_mut_a!(self), 0);
                return 2;
            }
            0xCB8F => {
                res_reg(get_mut_a!(self), 1);
                return 2;
            }
            0xCB97 => {
                res_reg(get_mut_a!(self), 2);
                return 2;
            }
            0xCB9F => {
                res_reg(get_mut_a!(self), 3);
                return 2;
            }
            0xCBA7 => {
                res_reg(get_mut_a!(self), 4);
                return 2;
            }
            0xCBAF => {
                res_reg(get_mut_a!(self), 5);
                return 2;
            }
            0xCBB7 => {
                res_reg(get_mut_a!(self), 6);
                return 2;
            }
            0xCBBF => {
                res_reg(get_mut_a!(self), 7);
                return 2;
            }

            0xCB80 => {
                res_reg(get_mut_b!(self), 0);
                return 2;
            }
            0xCB88 => {
                res_reg(get_mut_b!(self), 1);
                return 2;
            }
            0xCB90 => {
                res_reg(get_mut_b!(self), 2);
                return 2;
            }
            0xCB98 => {
                res_reg(get_mut_b!(self), 3);
                return 2;
            }
            0xCBA0 => {
                res_reg(get_mut_b!(self), 4);
                return 2;
            }
            0xCBA8 => {
                res_reg(get_mut_b!(self), 5);
                return 2;
            }
            0xCBB0 => {
                res_reg(get_mut_b!(self), 6);
                return 2;
            }
            0xCBB8 => {
                res_reg(get_mut_b!(self), 7);
                return 2;
            }

            0xCB81 => {
                res_reg(get_mut_c!(self), 0);
                return 2;
            }
            0xCB89 => {
                res_reg(get_mut_c!(self), 1);
                return 2;
            }
            0xCB91 => {
                res_reg(get_mut_c!(self), 2);
                return 2;
            }
            0xCB99 => {
                res_reg(get_mut_c!(self), 3);
                return 2;
            }
            0xCBA1 => {
                res_reg(get_mut_c!(self), 4);
                return 2;
            }
            0xCBA9 => {
                res_reg(get_mut_c!(self), 5);
                return 2;
            }
            0xCBB1 => {
                res_reg(get_mut_c!(self), 6);
                return 2;
            }
            0xCBB9 => {
                res_reg(get_mut_c!(self), 7);
                return 2;
            }

            0xCB82 => {
                res_reg(get_mut_d!(self), 0);
                return 2;
            }
            0xCB8A => {
                res_reg(get_mut_d!(self), 1);
                return 2;
            }
            0xCB92 => {
                res_reg(get_mut_d!(self), 2);
                return 2;
            }
            0xCB9A => {
                res_reg(get_mut_d!(self), 3);
                return 2;
            }
            0xCBA2 => {
                res_reg(get_mut_d!(self), 4);
                return 2;
            }
            0xCBAA => {
                res_reg(get_mut_d!(self), 5);
                return 2;
            }
            0xCBB2 => {
                res_reg(get_mut_d!(self), 6);
                return 2;
            }
            0xCBBA => {
                res_reg(get_mut_d!(self), 7);
                return 2;
            }

            0xCB83 => {
                res_reg(get_mut_e!(self), 0);
                return 2;
            }
            0xCB8B => {
                res_reg(get_mut_e!(self), 1);
                return 2;
            }
            0xCB93 => {
                res_reg(get_mut_e!(self), 2);
                return 2;
            }
            0xCB9B => {
                res_reg(get_mut_e!(self), 3);
                return 2;
            }
            0xCBA3 => {
                res_reg(get_mut_e!(self), 4);
                return 2;
            }
            0xCBAB => {
                res_reg(get_mut_e!(self), 5);
                return 2;
            }
            0xCBB3 => {
                res_reg(get_mut_e!(self), 6);
                return 2;
            }
            0xCBBB => {
                res_reg(get_mut_e!(self), 7);
                return 2;
            }

            0xCB84 => {
                res_reg(get_mut_h!(self), 0);
                return 2;
            }
            0xCB8C => {
                res_reg(get_mut_h!(self), 1);
                return 2;
            }
            0xCB94 => {
                res_reg(get_mut_h!(self), 2);
                return 2;
            }
            0xCB9C => {
                res_reg(get_mut_h!(self), 3);
                return 2;
            }
            0xCBA4 => {
                res_reg(get_mut_h!(self), 4);
                return 2;
            }
            0xCBAC => {
                res_reg(get_mut_h!(self), 5);
                return 2;
            }
            0xCBB4 => {
                res_reg(get_mut_h!(self), 6);
                return 2;
            }
            0xCBBC => {
                res_reg(get_mut_h!(self), 7);
                return 2;
            }

            0xCB85 => {
                res_reg(get_mut_l!(self), 0);
                return 2;
            }
            0xCB8D => {
                res_reg(get_mut_l!(self), 1);
                return 2;
            }
            0xCB95 => {
                res_reg(get_mut_l!(self), 2);
                return 2;
            }
            0xCB9D => {
                res_reg(get_mut_l!(self), 3);
                return 2;
            }
            0xCBA5 => {
                res_reg(get_mut_l!(self), 4);
                return 2;
            }
            0xCBAD => {
                res_reg(get_mut_l!(self), 5);
                return 2;
            }
            0xCBB5 => {
                res_reg(get_mut_l!(self), 6);
                return 2;
            }
            0xCBBD => {
                res_reg(get_mut_l!(self), 7);
                return 2;
            }

            0xCB86 => {
                res_mem(mem, 0, get_hl!(self));
                return 4;
            }
            0xCB8E => {
                res_mem(mem, 1, get_hl!(self));
                return 4;
            }
            0xCB96 => {
                res_mem(mem, 2, get_hl!(self));
                return 4;
            }
            0xCB9E => {
                res_mem(mem, 3, get_hl!(self));
                return 4;
            }
            0xCBA6 => {
                res_mem(mem, 4, get_hl!(self));
                return 4;
            }
            0xCBAE => {
                res_mem(mem, 5, get_hl!(self));
                return 4;
            }
            0xCBB6 => {
                res_mem(mem, 6, get_hl!(self));
                return 4;
            }
            0xCBBE => {
                res_mem(mem, 7, get_hl!(self));
                return 4;
            }

            0xCBC7 => {
                set_reg(get_mut_a!(self), 0);
                return 2;
            }
            0xCBCF => {
                set_reg(get_mut_a!(self), 1);
                return 2;
            }
            0xCBD7 => {
                set_reg(get_mut_a!(self), 2);
                return 2;
            }
            0xCBDF => {
                set_reg(get_mut_a!(self), 3);
                return 2;
            }
            0xCBE7 => {
                set_reg(get_mut_a!(self), 4);
                return 2;
            }
            0xCBEF => {
                set_reg(get_mut_a!(self), 5);
                return 2;
            }
            0xCBF7 => {
                set_reg(get_mut_a!(self), 6);
                return 2;
            }
            0xCBFF => {
                set_reg(get_mut_a!(self), 7);
                return 2;
            }

            0xCBC0 => {
                set_reg(get_mut_b!(self), 0);
                return 2;
            }
            0xCBC8 => {
                set_reg(get_mut_b!(self), 1);
                return 2;
            }
            0xCBD0 => {
                set_reg(get_mut_b!(self), 2);
                return 2;
            }
            0xCBD8 => {
                set_reg(get_mut_b!(self), 3);
                return 2;
            }
            0xCBE0 => {
                set_reg(get_mut_b!(self), 4);
                return 2;
            }
            0xCBE8 => {
                set_reg(get_mut_b!(self), 5);
                return 2;
            }
            0xCBF0 => {
                set_reg(get_mut_b!(self), 6);
                return 2;
            }
            0xCBF8 => {
                set_reg(get_mut_b!(self), 7);
                return 2;
            }

            0xCBC1 => {
                set_reg(get_mut_c!(self), 0);
                return 2;
            }
            0xCBC9 => {
                set_reg(get_mut_c!(self), 1);
                return 2;
            }
            0xCBD1 => {
                set_reg(get_mut_c!(self), 2);
                return 2;
            }
            0xCBD9 => {
                set_reg(get_mut_c!(self), 3);
                return 2;
            }
            0xCBE1 => {
                set_reg(get_mut_c!(self), 4);
                return 2;
            }
            0xCBE9 => {
                set_reg(get_mut_c!(self), 5);
                return 2;
            }
            0xCBF1 => {
                set_reg(get_mut_c!(self), 6);
                return 2;
            }
            0xCBF9 => {
                set_reg(get_mut_c!(self), 7);
                return 2;
            }

            0xCBC2 => {
                set_reg(get_mut_d!(self), 0);
                return 2;
            }
            0xCBCA => {
                set_reg(get_mut_d!(self), 1);
                return 2;
            }
            0xCBD2 => {
                set_reg(get_mut_d!(self), 2);
                return 2;
            }
            0xCBDA => {
                set_reg(get_mut_d!(self), 3);
                return 2;
            }
            0xCBE2 => {
                set_reg(get_mut_d!(self), 4);
                return 2;
            }
            0xCBEA => {
                set_reg(get_mut_d!(self), 5);
                return 2;
            }
            0xCBF2 => {
                set_reg(get_mut_d!(self), 6);
                return 2;
            }
            0xCBFA => {
                set_reg(get_mut_d!(self), 7);
                return 2;
            }

            0xCBC3 => {
                set_reg(get_mut_e!(self), 0);
                return 2;
            }
            0xCBCB => {
                set_reg(get_mut_e!(self), 1);
                return 2;
            }
            0xCBD3 => {
                set_reg(get_mut_e!(self), 2);
                return 2;
            }
            0xCBDB => {
                set_reg(get_mut_e!(self), 3);
                return 2;
            }
            0xCBE3 => {
                set_reg(get_mut_e!(self), 4);
                return 2;
            }
            0xCBEB => {
                set_reg(get_mut_e!(self), 5);
                return 2;
            }
            0xCBF3 => {
                set_reg(get_mut_e!(self), 6);
                return 2;
            }
            0xCBFB => {
                set_reg(get_mut_e!(self), 7);
                return 2;
            }

            0xCBC4 => {
                set_reg(get_mut_h!(self), 0);
                return 2;
            }
            0xCBCC => {
                set_reg(get_mut_h!(self), 1);
                return 2;
            }
            0xCBD4 => {
                set_reg(get_mut_h!(self), 2);
                return 2;
            }
            0xCBDC => {
                set_reg(get_mut_h!(self), 3);
                return 2;
            }
            0xCBE4 => {
                set_reg(get_mut_h!(self), 4);
                return 2;
            }
            0xCBEC => {
                set_reg(get_mut_h!(self), 5);
                return 2;
            }
            0xCBF4 => {
                set_reg(get_mut_h!(self), 6);
                return 2;
            }
            0xCBFC => {
                set_reg(get_mut_h!(self), 7);
                return 2;
            }

            0xCBC5 => {
                set_reg(get_mut_l!(self), 0);
                return 2;
            }
            0xCBCD => {
                set_reg(get_mut_l!(self), 1);
                return 2;
            }
            0xCBD5 => {
                set_reg(get_mut_l!(self), 2);
                return 2;
            }
            0xCBDD => {
                set_reg(get_mut_l!(self), 3);
                return 2;
            }
            0xCBE5 => {
                set_reg(get_mut_l!(self), 4);
                return 2;
            }
            0xCBED => {
                set_reg(get_mut_l!(self), 5);
                return 2;
            }
            0xCBF5 => {
                set_reg(get_mut_l!(self), 6);
                return 2;
            }
            0xCBFD => {
                set_reg(get_mut_l!(self), 7);
                return 2;
            }

            0xCBC6 => {
                set_mem(mem, 0, get_hl!(self));
                return 4;
            }
            0xCBCE => {
                set_mem(mem, 1, get_hl!(self));
                return 4;
            }
            0xCBD6 => {
                set_mem(mem, 2, get_hl!(self));
                return 4;
            }
            0xCBDE => {
                set_mem(mem, 3, get_hl!(self));
                return 4;
            }
            0xCBE6 => {
                set_mem(mem, 4, get_hl!(self));
                return 4;
            }
            0xCBEE => {
                set_mem(mem, 5, get_hl!(self));
                return 4;
            }
            0xCBF6 => {
                set_mem(mem, 6, get_hl!(self));
                return 4;
            }
            0xCBFE => {
                set_mem(mem, 7, get_hl!(self));
                return 4;
            }

            _ => {
                println!("opcode dispatch broke :( opcode: {:x}", opcode);
                return 1;
            }
        }
    }

    /// Execute an opcode that takes a 16 bit argument
    fn op_param_16_bit(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        let param = self.get_param_16_bit(mem);
        match opcode {
            0x01 => {
                ld_reg_16bit(param, get_mut_bc!(self));
                return 3;
            }
            0x11 => {
                ld_reg_16bit(param, get_mut_de!(self));
                return 3;
            }
            0x21 => {
                ld_reg_16bit(param, get_mut_hl!(self));
                return 3;
            }
            0x31 => {
                ld_reg_16bit(param, get_mut_sp!(self));
                return 3;
            }

            0xEA => {
                ld_mem(param, mem, get_a!(self));
                return 4;
            }

            0x08 => {
                ld_mem_16bit(param, mem, get_sp!(self));
                return 5;
            }

            // CALL
            0xCD => {
                call(param, get_mut_pc!(self), get_mut_sp!(self), mem);
                return 3;
            }

            0xC4 => {
                call_nz(
                    param,
                    get_f!(self),
                    get_mut_pc!(self),
                    get_mut_sp!(self),
                    mem,
                );
                return 3;
            }
            0xCC => {
                call_z(
                    param,
                    get_f!(self),
                    get_mut_pc!(self),
                    get_mut_sp!(self),
                    mem,
                );
                return 3;
            }
            0xD4 => {
                call_nc(
                    param,
                    get_f!(self),
                    get_mut_pc!(self),
                    get_mut_sp!(self),
                    mem,
                );
                return 3;
            }
            0xDC => {
                call_c(
                    param,
                    get_f!(self),
                    get_mut_pc!(self),
                    get_mut_sp!(self),
                    mem,
                );
                return 3;
            }

            // JUMP
            0xC3 => {
                jp(param, get_mut_pc!(self));
                return 3;
            }

            0xC2 => {
                jp_nz(param, get_f!(self), get_mut_pc!(self));
                return 3;
            }
            0xCA => {
                jp_z(param, get_f!(self), get_mut_pc!(self));
                return 3;
            }
            0xD2 => {
                jp_nc(param, get_f!(self), get_mut_pc!(self));
                return 3;
            }
            0xDA => {
                jp_c(param, get_f!(self), get_mut_pc!(self));
                return 3;
            }

            0xFA => {
                ld_reg(mem.get_mem_u8(param as usize), get_mut_a!(self));
                return 4;
            }

            _ => {
                println!("opcode dispatched to 16 bit param executer but that didnt match the op");
                return 1;
            }
        };
    }

    /// Execute an opcode that takes a 8 bit argument
    fn op_param_8_bit(&mut self, mem: &mut Mmu, opcode: u16) -> u8 {
        let param = self.get_param_8_bit(mem);
        match opcode {
            0x3E => {
                ld_reg(param, get_mut_a!(self));
                return 2;
            }
            0x06 => {
                ld_reg(param, get_mut_b!(self));
                return 2;
            }
            0x0E => {
                ld_reg(param, get_mut_c!(self));
                return 2;
            }
            0x16 => {
                ld_reg(param, get_mut_d!(self));
                return 2;
            }
            0x1E => {
                ld_reg(param, get_mut_e!(self));
                return 2;
            }
            0x26 => {
                ld_reg(param, get_mut_h!(self));
                return 2;
            }
            0x2E => {
                ld_reg(param, get_mut_l!(self));
                return 2;
            }

            0x36 => {
                ld_mem(get_hl!(self), mem, param);
                return 3;
            }
            0xE0 => {
                ld_mem(param as u16 + 0xFF00, mem, get_a!(self));
                return 3;
            }
            0xEE => {
                xor(param, get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xF6 => {
                or(param, get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xF0 => {
                ld_reg(mem.get_mem_u8(param as usize + 0xFF00), get_mut_a!(self));
                return 3;
            }

            0xE6 => {
                and(param, get_mut_a!(self), get_mut_f!(self));
                return 2;
            }
            0xC6 => {
                add(param, get_mut_a!(self), get_mut_f!(self));
                return 2;
            }

            0xE8 => {
                add_sp_param(param, get_mut_sp!(self), get_mut_f!(self));
                return 4;
            }

            0xD6 => {
                sub(param, get_mut_a!(self), get_mut_f!(self));
                return 2;
            }

            0xCE => {
                adc(param, get_mut_a!(self), get_mut_f!(self));
                return 2;
            }

            0xDE => {
                sbc(param, get_mut_a!(self), get_mut_f!(self));
                return 2;
            }

            0xFE => {
                cp(param, get_a!(self), get_mut_f!(self));
                return 2;
            }

            0x10 => {
                stop(&mut self.halt);
                return 2;
            }
            // Jumps
            0x18 => {
                jr(param, get_mut_pc!(self));
                return 2;
            }

            0x20 => {
                jr_nz(param, get_f!(self), get_mut_pc!(self));
                return 2;
            }
            0x28 => {
                jr_z(param, get_f!(self), get_mut_pc!(self));
                return 2;
            }
            0x30 => {
                jr_nc(param, get_f!(self), get_mut_pc!(self));
                return 2;
            }
            0x38 => {
                jr_c(param, get_f!(self), get_mut_pc!(self));
                return 2;
            }

            0xF8 => {
                ldhl_sp_n(param, get_sp!(self), get_mut_hl!(self), get_mut_f!(self));
                return 3;
            }
            _ => {
                println!("opcode dispatched to 8 bit param executer but that didnt match the op");
                return 1;
            }
        };
    }

    /// Exec the next instruction and return how many machine cycles it takes (cycles/4)
    pub fn exec_next(&mut self, mem: &mut Mmu) -> u8 {
        let val = self.handle_int(mem);
        if val != 0 {
            return val;
        }

        if cfg!(debug_assertions = true) {
            println!("a: {:x}", get_a!(self));
            println!("pc: {:x}", get_pc!(self));
            println!("sp: {:x}", get_sp!(self));
            println!("hl: {:x}", get_hl!(self));
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

    /// Handle interrupts
    pub fn handle_int(&mut self, mem: &mut Mmu) -> u8 {
        let ir = mem.get_interrupts();
        let interrupts = mem.get_interrupts_enabled() & ir;
        if cfg!(debug_assertions = true) {
            println!("interrupts: {:b}", interrupts);
        }

        if interrupts != 0 {
            self.halt = 0;
            if self.ime == 1 {
                let mut addr_to_call: u16 = 0;
                if interrupts & 0b00000001 == 0b00000001 {
                    addr_to_call = 0x40;
                    mem.set_mem_u8(0xFF0F, ir - 0b00000001);
                } else if interrupts & 0b00000010 == 0b00000010 {
                    addr_to_call = 0x48;
                    mem.set_mem_u8(0xFF0F, ir - 0b00000010);
                } else if interrupts & 0b00000100 == 0b00000100 {
                    addr_to_call = 0x50;
                    mem.set_mem_u8(0xFF0F, ir - 0b00000100);
                } else if interrupts & 0b00001000 == 0b00001000 {
                    addr_to_call = 0x58;
                    mem.set_mem_u8(0xFF0F, ir - 0b00001000);
                } else if interrupts & 0b00010000 == 0b00010000 {
                    addr_to_call = 0x60;
                    mem.set_mem_u8(0xFF0F, ir - 0b00010000);
                }

                // Disable interrupts
                self.ime = 0;

                // Push current PC
                mem.push_u16(get_sp_mut!(self), get_pc!(self));

                // Jump
                set_pc!(self, addr_to_call);
                return 4;
            }
        }
        return 0;
    }

    pub fn skip_bios(&mut self) {
        set_af!(self, 0x01B0);
        set_bc!(self, 0x0013);
        set_de!(self, 0x00DB);
        set_hl!(self, 0x014D);
        set_pc!(self, 0x0100);
        set_sp!(self, 0xFFFE);
    }
}
