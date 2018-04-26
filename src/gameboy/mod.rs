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

            0x7F => ld_a_a!(self),
            0x78 => ld_a_b!(self),
            0x79 => ld_a_c!(self),
            0x7A => ld_a_d!(self),
            0x7B => ld_a_e!(self),
            0x7C => ld_a_h!(self),
            0x7D => ld_a_l!(self),
            0x7E => ld_a_hl_val!(self),
            0x47 => ld_b_a!(self),
            0x40 => ld_b_b!(self),
            0x41 => ld_b_c!(self),
            0x42 => ld_b_d!(self),
            0x43 => ld_b_e!(self),
            0x44 => ld_b_h!(self),
            0x45 => ld_b_l!(self),
            0x46 => ld_b_hl_val!(self),
            0x4F => ld_c_a!(self),
            0x48 => ld_c_b!(self),
            0x49 => ld_c_c!(self),
            0x4A => ld_c_d!(self),
            0x4B => ld_c_e!(self),
            0x4C => ld_c_h!(self),
            0x4D => ld_c_l!(self),
            0x4E => ld_c_hl_val!(self),
            0x57 => ld_d_a!(self),
            0x50 => ld_d_b!(self),
            0x51 => ld_d_c!(self),
            0x52 => ld_d_d!(self),
            0x53 => ld_d_e!(self),
            0x54 => ld_d_h!(self),
            0x55 => ld_d_l!(self),
            0x56 => ld_d_hl_val!(self),
            0x5F => ld_e_a!(self),
            0x58 => ld_e_b!(self),
            0x59 => ld_e_c!(self),
            0x5A => ld_e_d!(self),
            0x5B => ld_e_e!(self),
            0x5C => ld_e_h!(self),
            0x5D => ld_e_l!(self),
            0x5E => ld_e_hl_val!(self),
            0x67 => ld_h_a!(self),
            0x60 => ld_h_b!(self),
            0x61 => ld_h_c!(self),
            0x62 => ld_h_d!(self),
            0x63 => ld_h_e!(self),
            0x64 => ld_h_h!(self),
            0x65 => ld_h_l!(self),
            0x66 => ld_h_hl_val!(self),
            0x6F => ld_l_a!(self),
            0x68 => ld_l_b!(self),
            0x69 => ld_l_c!(self),
            0x6A => ld_l_d!(self),
            0x6B => ld_l_e!(self),
            0x6C => ld_l_h!(self),
            0x6D => ld_l_l!(self),
            0x6E => ld_l_hl_val!(self),

            0x0A => ld_a_bc_val!(self),
            0x1A => ld_a_de_val!(self),

            0x77 => ld_hl_val_a!(self),
            0x70 => ld_hl_val_b!(self),
            0x71 => ld_hl_val_c!(self),
            0x72 => ld_hl_val_d!(self),
            0x73 => ld_hl_val_e!(self),
            0x74 => ld_hl_val_h!(self),
            0x75 => ld_hl_val_l!(self),
            0x02 => ld_bc_val_a!(self),
            0x12 => ld_de_val_a!(self),

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

            0xF5 => push_af!(self),
            0xC5 => push_bc!(self),
            0xD5 => push_de!(self),
            0xE5 => push_hl!(self),

            0x97 => sub_a_a!(self),
            0x90 => sub_a_b!(self),
            0x91 => sub_a_c!(self),
            0x92 => sub_a_d!(self),
            0x93 => sub_a_e!(self),
            0x94 => sub_a_h!(self),
            0x95 => sub_a_l!(self),

            0xF9 => ld_sp_hl!(self),

            0xE2 => ld_c_val_a!(self),
            0xF2 => ld_a_c_val!(self),

            0x22 => ldi_hl_a!(self),
            0x2A => ldi_a_hl!(self),

            0x32 => ldd_hl_a!(self),
            0x3A => ldd_a_hl!(self),

            0xF1 => pop_af!(self),
            0xC1 => pop_bc!(self),
            0xD1 => pop_de!(self),
            0xE1 => pop_hl!(self),

            // RET
            0xC9 => pop_pc!(self),

            0x9F => sbc_a_a!(self),
            0x98 => sbc_a_b!(self),
            0x99 => sbc_a_c!(self),
            0x9A => sbc_a_d!(self),
            0x9B => sbc_a_e!(self),
            0x9C => sbc_a_h!(self),
            0x9D => sbc_a_l!(self),
            0x9E => sbc_a_hl_val!(self),

            0x07 => rlca!(self),
            0x17 => rla!(self),

            0xCB1F => rl_a!(self),
            0xCB10 => rl_b!(self),
            0xCB11 => rl_c!(self),
            0xCB12 => rl_d!(self),
            0xCB13 => rl_e!(self),
            0xCB14 => rl_h!(self),
            0xCB15 => rl_l!(self),

            0xCB7C => bit_7_h!(self),

            _ => println!("opcode dispatch broke :("),
        }
    }

    fn op_param_16_bit(&mut self, opcode: u16) {
        let param = self.get_param_16_bit();
        match opcode {
            0x01 => ld_bc_param!(self, param),
            0x11 => ld_de_param!(self, param),
            0x21 => ld_hl_param!(self, param),
            0x31 => ld_sp_param!(self, param),

            0x08 => ld_param_val_sp!(self, param),

            // CALL
            0xCD => call_nn!(self, param),

            0xFA => ld_a_nn_val!(self, param),

            _ => println!("opcode dispatched to 16 bit param executer but that didnt match the op"),
        };
    }

    fn op_param_8_bit(&mut self, opcode: u16) {
        let param = self.get_param_8_bit();
        match opcode {
            0x3E => ld_a_param!(self, param),
            0x06 => ld_b_param!(self, param),
            0x0E => ld_c_param!(self, param),
            0x16 => ld_d_param!(self, param),
            0x1E => ld_e_param!(self, param),
            0x26 => ld_h_param!(self, param),
            0x2E => ld_l_param!(self, param),

            0xEA => ld_nn_val_a!(self, param),
            0x36 => ld_hl_val_n!(self, param),
            0xE0 => ld_n_val_a!(self, param),
            0xF0 => ld_a_n_val!(self, param),

            0xFE => cp_n!(self, param),
            // Jumps
            0x18 => jr_n!(self, param),

            0x20 => jr_nz_n!(self, param),
            0x28 => jr_z_n!(self, param),
            0x30 => jr_nc_n!(self, param),
            0x38 => jr_c_n!(self, param),

            // this doesnt really work and is complicated so i'll figure it out later
            0xF8 => ldhl_sp_n!(self, param),
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
