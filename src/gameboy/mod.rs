//extern crate elapsed;

//use elapsed::measure_time;

use std::time::{Duration, Instant};
pub mod mmu;

#[macro_use]
pub mod ops;

macro_rules! set_a {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.af.split.upper = $x;
        }
    };
}

macro_rules! set_f {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.af.split.lower = $x;
        }
    };
}

macro_rules! set_af {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.af.whole = $x;
        }
    };
}

macro_rules! set_b {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.bc.split.upper = $x;
        }
    };
}

macro_rules! set_c {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.bc.split.lower = $x;
        }
    };
}

macro_rules! set_bc {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.bc.whole = $x;
        }
    };
}

macro_rules! set_d {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.de.split.upper = $x;
        }
    };
}

macro_rules! set_e {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.de.split.lower = $x;
        }
    };
}

macro_rules! set_de {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.de.whole = $x;
        }
    };
}

macro_rules! set_h {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.hl.split.upper = $x;
        }
    };
}

macro_rules! set_l {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.hl.split.lower = $x;
        }
    };
}

macro_rules! set_hl {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.hl.whole = $x;
        }
    };
}

macro_rules! set_sp {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.sp.whole = $x;
        }
    };
}

macro_rules! set_pc {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.pc.whole = $x;
        }
    };
}

macro_rules! get_a {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.upper
        }
    }
}

macro_rules! get_f {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower
        }
    }
}

macro_rules! get_af {
    ($self_: ident) => {
        unsafe {
            $self_.af.whole
        }
    }
}

macro_rules! get_b {
    ($self_: ident) => {
        unsafe {
            $self_.bc.split.upper
        }
    }
}

macro_rules! get_c {
    ($self_: ident) => {
        unsafe {
            $self_.bc.split.lower
        }
    }
}

macro_rules! get_bc {
    ($self_: ident) => {
        unsafe {
            $self_.bc.whole
        }
    }
}

macro_rules! get_d {
    ($self_: ident) => {
        unsafe {
            $self_.de.split.upper
        }
    }
}

macro_rules! get_e {
    ($self_: ident) => {
        unsafe {
            $self_.de.split.lower
        }
    }
}

macro_rules! get_de {
    ($self_: ident) => {
        unsafe {
            $self_.de.whole
        }
    }
}

macro_rules! get_h {
    ($self_: ident) => {
        unsafe {
            $self_.hl.split.upper
        }
    }
}

macro_rules! get_l {
    ($self_: ident) => {
        unsafe {
            $self_.hl.split.lower
        }
    }
}

macro_rules! get_hl {
    ($self_: ident) => {
        unsafe {
            $self_.hl.whole
        }
    }
}

macro_rules! get_sp {
    ($self_: ident) => {
        unsafe {
            $self_.sp.whole
        }
    }
}

macro_rules! get_sp_mut {
    ($self_: ident) => {
        unsafe {
            &mut $self_.sp.whole
        }
    }
}

macro_rules! get_pc {
    ($self_: ident) => {
        unsafe {
            $self_.pc.whole
        }
    }
}

macro_rules! get_z_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b10000000) >> 7
        }
    }
}

macro_rules! get_n_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b01000000) >> 6
        }
    }
}

macro_rules! get_h_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b00100000) >> 5
        }
    }
}

macro_rules! get_c_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b00010000) >> 4
        }
    }
}

macro_rules! set_z_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b10000000
        }
    }
}

macro_rules! set_n_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b01000000
        }
    }
}

macro_rules! set_h_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b00100000
        }
    }
}

macro_rules! set_c_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b00010000
        }
    }
}

macro_rules! unset_z_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower &= 0b01111111
        }
    }
}

macro_rules! unset_n_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower &= 0b10111111
        }
    }
}

macro_rules! unset_h_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower &= 0b11011111
        }
    }
}

macro_rules! unset_c_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower &= 0b11101111
        }
    }
}

// endianness makes my life hard here?
#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
struct Reg8Bit {
    lower: u8,
    upper: u8,
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
struct Reg8Bit {
    upper: u8,
    lower: u8,
}

#[derive(Clone, Copy)]
union Reg16Bit {
    split: Reg8Bit,
    whole: u16,
}

pub struct Cpu {
    af: Reg16Bit,
    bc: Reg16Bit,
    de: Reg16Bit,
    hl: Reg16Bit,
    sp: Reg16Bit,
    pc: Reg16Bit,
    mem: mmu::Mmu,
}

impl Cpu {
    pub fn new() -> Cpu {
        let af = Reg16Bit{ whole: 0 };
        let bc = Reg16Bit{ whole: 0 };
        let de = Reg16Bit{ whole: 0 };
        let hl = Reg16Bit{ whole: 0 };
        let sp = Reg16Bit{ whole: 0 };
        let pc = Reg16Bit{ whole: 0 };
        let mem = mmu::Mmu::new();
        Cpu {af, bc, de, hl, sp, pc, mem}
    }

    fn get_opcode(&mut self) -> u16 {
        let mut pc = get_pc!(self);
        let temp = self.mem.get_mem_u8(pc as usize);
        pc += 1;
        let temp_u16: u16;
        if temp == 0xCB {
            temp_u16 = 0xCB00 + (self.mem.get_mem_u8(pc as usize) as u16);
            pc += 1;
        }
        else {
            temp_u16 = temp as u16;
        }

        set_pc!(self, pc);

        temp_u16
    }

    fn get_param_8_bit(&mut self) -> u8 {
        let mut pc = get_pc!(self);
        let temp = self.mem.get_mem_u8(pc as usize);
        pc += 1;
        set_pc!(self, pc);
        if cfg!(debug_assertions = true) {
            println!("8 bit param: b: {:b}, hex: {:x}, signed: {:x}", temp, temp, (temp as i8) as i16);
        }
        temp
    }

    fn get_param_16_bit(&mut self) -> u16 {
        let mut pc = get_pc!(self);
        let temp = self.mem.get_mem_u8(pc as usize);
        pc += 1;
        let temp_u16: u16 = (temp as u16) + ((self.mem.get_mem_u8(pc as usize) as u16) << 8);
        pc += 1;
        set_pc!(self, pc);
        if cfg!(debug_assertions = true) {
            println!("16 bit param: b: {:b}, hex: {:x}", temp_u16, temp_u16);
        }
        temp_u16
    }

    fn exec_dispatcher(&mut self, opcode: u16) {
        match opcode {
            0x00 => nop!(self),
            0x01 => self.op_param_16_bit(opcode),
            0x08 => self.op_param_16_bit(opcode),
            0x11 => self.op_param_16_bit(opcode),
            0x21 => self.op_param_16_bit(opcode),
            0x31 => self.op_param_16_bit(opcode),
            0xFA => self.op_param_16_bit(opcode),
            0xCD => self.op_param_16_bit(opcode),
            0x06 => self.op_param_8_bit(opcode),
            0x0E => self.op_param_8_bit(opcode),
            0x16 => self.op_param_8_bit(opcode),
            0x18 => self.op_param_8_bit(opcode),
            0x1E => self.op_param_8_bit(opcode),
            0x26 => self.op_param_8_bit(opcode),
            0x2E => self.op_param_8_bit(opcode),
            0x3E => self.op_param_8_bit(opcode),
            0xEA => self.op_param_8_bit(opcode),
            0x20 => self.op_param_8_bit(opcode),
            0x28 => self.op_param_8_bit(opcode),
            0x30 => self.op_param_8_bit(opcode),
            0x38 => self.op_param_8_bit(opcode),
            0xE0 => self.op_param_8_bit(opcode),
            0xF0 => self.op_param_8_bit(opcode),
            0xFE => self.op_param_8_bit(opcode),

            0x3C => inc_a!(self),
            0x04 => inc_b!(self),
            0x0C => inc_c!(self),
            0x14 => inc_d!(self),
            0x1C => inc_e!(self),
            0x24 => inc_h!(self),
            0x2C => inc_l!(self),

            0xBE => cp_hl_val!(self),
            0xBF => cp_a!(self),
            0xB8 => cp_b!(self),
            0xB9 => cp_c!(self),
            0xBA => cp_d!(self),
            0xBB => cp_e!(self),
            0xBC => cp_h!(self),
            0xBD => cp_l!(self),

            0x35 => dec_hl_val!(self),
            0x3D => dec_a!(self),
            0x05 => dec_b!(self),
            0x0D => dec_c!(self),
            0x15 => dec_d!(self),
            0x1D => dec_e!(self),
            0x25 => dec_h!(self),
            0x2D => dec_l!(self),

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
            0xCB36 => swap_hl_val!(self),

            0x27 => daa!(self),
            0x2F => cpl!(self),

            0x37 => ccf!(self),
            0x3F => scf!(self),

            0x79 => set_a!(self, get_c!(self)),
            0x7A => set_a!(self, get_d!(self)),
            0x7B => set_a!(self, get_e!(self)),
            0x7C => set_a!(self, get_h!(self)),
            0x7D => set_a!(self, get_l!(self)),
            0x7E => set_a!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x7F => set_a!(self, get_a!(self)),
            0x40 => set_b!(self, get_b!(self)),
            0x41 => set_b!(self, get_c!(self)),
            0x42 => set_b!(self, get_d!(self)),
            0x43 => set_b!(self, get_e!(self)),
            0x44 => set_b!(self, get_h!(self)),
            0x45 => set_b!(self, get_l!(self)),
            0x46 => set_b!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x47 => set_b!(self, get_a!(self)),
            0x48 => set_c!(self, get_b!(self)),
            0x49 => set_c!(self, get_c!(self)),
            0x4A => set_c!(self, get_d!(self)),
            0x4B => set_c!(self, get_e!(self)),
            0x4C => set_c!(self, get_h!(self)),
            0x4D => set_c!(self, get_l!(self)),
            0x4E => set_c!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x4F => set_c!(self, get_a!(self)),
            0x50 => set_d!(self, get_b!(self)),
            0x51 => set_d!(self, get_c!(self)),
            0x52 => set_d!(self, get_d!(self)),
            0x53 => set_d!(self, get_e!(self)),
            0x54 => set_d!(self, get_h!(self)),
            0x55 => set_d!(self, get_l!(self)),
            0x56 => set_d!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x57 => set_d!(self, get_a!(self)),
            0x58 => set_e!(self, get_b!(self)),
            0x59 => set_e!(self, get_c!(self)),
            0x5A => set_e!(self, get_d!(self)),
            0x5B => set_e!(self, get_e!(self)),
            0x5C => set_e!(self, get_h!(self)),
            0x5D => set_e!(self, get_l!(self)),
            0x5E => set_e!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x5F => set_e!(self, get_a!(self)),
            0x60 => set_h!(self, get_b!(self)),
            0x61 => set_h!(self, get_c!(self)),
            0x62 => set_h!(self, get_d!(self)),
            0x63 => set_h!(self, get_e!(self)),
            0x64 => set_h!(self, get_h!(self)),
            0x65 => set_h!(self, get_l!(self)),
            0x66 => set_h!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x67 => set_h!(self, get_a!(self)),
            0x68 => set_l!(self, get_b!(self)),
            0x69 => set_l!(self, get_c!(self)),
            0x6A => set_l!(self, get_d!(self)),
            0x6B => set_l!(self, get_e!(self)),
            0x6C => set_l!(self, get_h!(self)),
            0x6D => set_l!(self, get_l!(self)),
            0x6E => set_l!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x6F => set_l!(self, get_a!(self)),
            0x0A => set_a!(self, self.mem.get_mem_u8(get_bc!(self) as usize)),
            0x1A => set_a!(self, self.mem.get_mem_u8(get_de!(self) as usize)),
            0x7E => set_a!(self, self.mem.get_mem_u8(get_hl!(self) as usize)),
            0x70 => self.mem.set_mem_u8(get_hl!(self) as usize, get_b!(self)),
            0x71 => self.mem.set_mem_u8(get_hl!(self) as usize, get_c!(self)),
            0x72 => self.mem.set_mem_u8(get_hl!(self) as usize, get_d!(self)),
            0x73 => self.mem.set_mem_u8(get_hl!(self) as usize, get_e!(self)),
            0x74 => self.mem.set_mem_u8(get_hl!(self) as usize, get_h!(self)),
            0x75 => self.mem.set_mem_u8(get_hl!(self) as usize, get_l!(self)),
            0x02 => self.mem.set_mem_u8(get_bc!(self) as usize, get_a!(self)),
            0x12 => self.mem.set_mem_u8(get_de!(self) as usize, get_a!(self)),
            0x77 => self.mem.set_mem_u8(get_hl!(self) as usize, get_a!(self)),

            0xA7 => and_a!(self),
            0xA0 => and_b!(self),
            0xA1 => and_c!(self),
            0xA2 => and_d!(self),
            0xA3 => and_e!(self),
            0xA4 => and_h!(self),
            0xA5 => and_l!(self),
            0xA6 => and_hl_val!(self),

            0xB7 => or_a!(self),
            0xB0 => or_b!(self),
            0xB1 => or_c!(self),
            0xB2 => or_d!(self),
            0xB3 => or_e!(self),
            0xB4 => or_h!(self),
            0xB5 => or_l!(self),
            0xB6 => or_hl_val!(self),

            0xAF => xor_a!(self),
            0xA8 => xor_b!(self),
            0xA9 => xor_c!(self),
            0xAA => xor_d!(self),
            0xAB => xor_e!(self),
            0xAC => xor_h!(self),
            0xAD => xor_l!(self),
            0xAE => xor_hl_val!(self),

            0xF9 => set_sp!(self, get_hl!(self)),
            0xE2 => self.mem.set_mem_u8((get_c!(self) as u16 + 0xFF00) as usize, get_a!(self)),
            0xF2 => set_a!(self, self.mem.get_mem_u8((get_c!(self) as u16 + 0xFF00) as usize)),

            0x22 => { self.mem.set_mem_u8(get_hl!(self) as usize, get_a!(self));
                      set_hl!(self, get_hl!(self) + 1);},
            0x2A => { set_a!(self, self.mem.get_mem_u8(get_hl!(self) as usize));
                      set_hl!(self, get_hl!(self) + 1);},

            0x32 => { self.mem.set_mem_u8(get_hl!(self) as usize, get_a!(self));
                      set_hl!(self, get_hl!(self) - 1);},

            0x3A => { set_a!(self, self.mem.get_mem_u8(get_hl!(self) as usize));
                      set_hl!(self, get_hl!(self) - 1);},

            0xC1 => set_bc!(self, self.mem.pop_u16(get_sp_mut!(self))),
            0xC5 => self.mem.push_u16(get_sp_mut!(self), get_bc!(self)),

            0xC9 => set_pc!(self, self.mem.pop_u16(get_sp_mut!(self))),

            0x99 => set_a!(self, get_c!(self) + get_c_flag!(self)),

            0x17 => { let tmp = get_c_flag!(self); unset_n_flag!(self); unset_h_flag!(self);
                    if ((get_a!(self) & 0b10000000) >> 7) == 1 { set_c_flag!(self); }
                    else { unset_c_flag!(self); }
                    set_a!(self, (get_a!(self) << 1) + tmp);
                    if get_a!(self) == 0 { set_z_flag!(self); }
                    else { unset_z_flag!(self); }
                },

            0xCB11 => { let tmp = get_c_flag!(self); unset_n_flag!(self); unset_h_flag!(self);
                    if ((get_c!(self) & 0b10000000) >> 7) == 1 { set_c_flag!(self); }
                    else { unset_c_flag!(self); }
                    set_c!(self, (get_c!(self) << 1) + tmp);
                    if get_c!(self) == 0 { set_z_flag!(self); }
                    else { unset_z_flag!(self); }
                },

            0xCB7C => { unset_n_flag!(self); set_h_flag!(self);
                if ((get_h!(self) & 0b10000000) >> 7) == 0 { set_z_flag!(self); }
                else { unset_z_flag!(self); } },


            _ => println!("opcode dispatch broke :("),
        }
    }

    fn op_param_16_bit(&mut self, opcode: u16) {
        let param = self.get_param_16_bit();
        match opcode {
            0x01 => set_bc!(self, param),
            0x11 => set_de!(self, param),
            0x21 => set_hl!(self, param),
            0x31 => set_sp!(self, param),

            0x08 => self.mem.set_mem_u16(param as usize, get_sp!(self)),

            // CALL
            0xCD => { self.mem.push_u16(get_sp_mut!(self), get_pc!(self)); set_pc!(self, param); }

            0xFA => set_a!(self, self.mem.get_mem_u8(param.swap_bytes() as usize)),
            _ => println!("opcode dispatched to 16 bit param executer but that didnt match the op"),
        };
    }

    fn op_param_8_bit(&mut self, opcode: u16) {
        let param = self.get_param_8_bit();
        match opcode {
            0x06 => set_b!(self, param),
            0x0E => set_c!(self, param),
            0x16 => set_d!(self, param),
            0x1E => set_e!(self, param),
            0x26 => set_h!(self, param),
            0x2E => set_l!(self, param),
            0x3E => set_a!(self, param),
            0xEA => self.mem.set_mem_u8(param as usize, get_a!(self)),
            0x36 => self.mem.set_mem_u8(get_hl!(self) as usize, param as u8),
            0xE0 => self.mem.set_mem_u8((param as u16 + 0xFF00) as usize, get_a!(self)),
            0xF0 => set_a!(self, self.mem.get_mem_u8((param as u16 + 0xFF00) as usize)),

            0x18 => set_pc!(self, (get_pc!(self) as i16 + ((param as i8) as i16)) as u16),

            0xFE => { let tmp = get_a!(self) - param;
                    if tmp == 0 { set_z_flag!(self); }
                    else { unset_z_flag!(self); }
                    set_n_flag!(self);
                    if get_a!(self) < tmp { set_c_flag!(self); }
                    else { unset_c_flag!(self); }
                    if (get_a!(self) & 0b00001111) < (param & 0b00001111) { unset_h_flag!(self); }
                    else { set_h_flag!(self); }
                },

            // Jumps
            0x20 => { if get_z_flag!(self) == 0 {set_pc!(self, (get_pc!(self) as i16 + ((param as i8) as i16)) as u16);} },
            0x28 => { if get_z_flag!(self) == 1 {set_pc!(self, (get_pc!(self) as i16 + ((param as i8) as i16)) as u16);} },
            0x30 => { if get_c_flag!(self) == 0 {set_pc!(self, (get_pc!(self) as i16 + ((param as i8) as i16)) as u16);} },
            0x38 => { if get_c_flag!(self) == 1 {set_pc!(self, (get_pc!(self) as i16 + ((param as i8) as i16)) as u16);} },

            // this doesnt really work and is complicated so i'll figure it out later
            0xF8 => { set_hl!(self, (get_sp!(self) as i16 + ((param as i8) as i16)) as u16); unset_z_flag!(self); unset_n_flag!(self);},
            _ => println!("opcode dispatched to 8 bit param executer but that didnt match the op"),
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

    pub fn test_opcodes(&mut self) {
        // 0x06 arg
        self.mem.set_mem_u8(0, 0x01);
        // 0x0E arg
        self.mem.set_mem_u8(1, 0b10);
        //self.op_param_8_bit(0x06, 0b00000001);
        self.exec_dispatcher(0x06);
        assert_eq!(get_b!(self), 0b00000001);
        self.exec_dispatcher(0x0E);
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

    pub fn test_get_opcode(&mut self) {
        self.mem.set_mem_u8(0, 0xCB);
        self.mem.set_mem_u8(1, 0x10);
        self.mem.set_mem_u8(2, 0x20);

        set_pc!(self, 0);
        assert_eq!(self.get_opcode(), 0xCB10);
        assert_eq!(get_pc!(self), 2);
        assert_eq!(self.get_opcode(), 0x20);
        assert_eq!(get_pc!(self), 3);
    }

    pub fn test_get_param(&mut self) {
        self.mem.set_mem_u8(3, 0x10);
        self.mem.set_mem_u8(4, 0x20);
        self.mem.set_mem_u8(5, 0x30);

        set_pc!(self, 3);
        assert_eq!(self.get_param_8_bit(), 0x10);
        assert_eq!(self.get_param_16_bit(), 0x3020);
        assert_eq!(get_pc!(self), 6);
    }

    pub fn test_stack(&mut self) {
        set_sp!(self, 0xFFFE);
        self.mem.push_u8(get_sp_mut!(self), 10);
        self.mem.push_u8(get_sp_mut!(self), 20);
        self.mem.push_u16(get_sp_mut!(self), 0x3020);

        assert_eq!(get_sp!(self), 0xFFFA);
        assert_eq!(self.mem.pop_u16(get_sp_mut!(self)), 0x3020);
        assert_eq!(self.mem.pop_u8(get_sp_mut!(self)), 20);
        assert_eq!(self.mem.pop_u8(get_sp_mut!(self)), 10);
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        self.mem.load_rom(rom_path);
    }

    pub fn run(&mut self) {
        loop {

            let start = Instant::now();
            //if cfg!(debug_assertions = true) {
                println!("pc: {:x}", get_pc!(self));
                println!("sp: {:x}", get_sp!(self));
                println!("b: {:b}", get_b!(self));
            //}

            let opcode = self.get_opcode();

            //if cfg!(debug_assertions = true) {
                println!("opcode: {:x}", opcode);
            //}

            self.exec_dispatcher(opcode);

            //if cfg!(debug_assertions = true) {
                println!("elapsed nanos: {}", start.elapsed().subsec_nanos());
                println!("");
            //}
        }
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
    derp.test_opcodes();
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
    derp.test_get_opcode();
}

#[test]
fn test_get_param() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_get_param();
}

#[test]
fn test_stack() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_stack();
}
