pub mod mmu;

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

#[derive(Clone, Copy)]
struct Reg8Bit {
    lower: u8,
    upper: u8,
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
        let temp = self.mem.get_ins(pc as usize);
        pc += 1;
        let temp_u16: u16;
        if temp == 0xCB {
            temp_u16 = 0xCB00 + (self.mem.get_ins(pc as usize) as u16);
            pc += 1;
        }
        else {
            temp_u16 = temp as u16;
        }

        set_pc!(self, pc);

        temp_u16
    }

    fn exec_dispatcher(&mut self, opcode: u16) {
        let param_16_bit: u16 = 0;
        let param_8_bit: u8 = 0;
        match opcode {
            0x01 => self.op_param_16_bit(opcode, param_16_bit),
            0x11 => self.op_param_16_bit(opcode, param_16_bit),
            0x21 => self.op_param_16_bit(opcode, param_16_bit),
            0x31 => self.op_param_16_bit(opcode, param_16_bit),
            0xFA => self.op_param_16_bit(opcode, param_16_bit),
            0x06 => self.op_param_8_bit(opcode, param_8_bit),
            0x0E => self.op_param_8_bit(opcode, param_8_bit),
            0x16 => self.op_param_8_bit(opcode, param_8_bit),
            0x1E => self.op_param_8_bit(opcode, param_8_bit),
            0x26 => self.op_param_8_bit(opcode, param_8_bit),
            0x2E => self.op_param_8_bit(opcode, param_8_bit),
            0x3E => self.op_param_8_bit(opcode, param_8_bit),
            0xEA => self.op_param_8_bit(opcode, param_8_bit),
            0x78 => self.op_no_param(opcode),
            0x79 => self.op_no_param(opcode),
            0x7A => self.op_no_param(opcode),
            0x7B => self.op_no_param(opcode),
            0x7C => self.op_no_param(opcode),
            0x7D => self.op_no_param(opcode),
            0x7E => self.op_no_param(opcode),
            0x7F => self.op_no_param(opcode),
            0x40 => self.op_no_param(opcode),
            0x41 => self.op_no_param(opcode),
            0x42 => self.op_no_param(opcode),
            0x43 => self.op_no_param(opcode),
            0x44 => self.op_no_param(opcode),
            0x45 => self.op_no_param(opcode),
            0x46 => self.op_no_param(opcode),
            0x47 => self.op_no_param(opcode),
            0x48 => self.op_no_param(opcode),
            0x49 => self.op_no_param(opcode),
            0x4A => self.op_no_param(opcode),
            0x4B => self.op_no_param(opcode),
            0x4C => self.op_no_param(opcode),
            0x4D => self.op_no_param(opcode),
            0x4E => self.op_no_param(opcode),
            0x4F => self.op_no_param(opcode),
            0x50 => self.op_no_param(opcode),
            0x51 => self.op_no_param(opcode),
            0x52 => self.op_no_param(opcode),
            0x53 => self.op_no_param(opcode),
            0x54 => self.op_no_param(opcode),
            0x55 => self.op_no_param(opcode),
            0x56 => self.op_no_param(opcode),
            0x57 => self.op_no_param(opcode),
            0x58 => self.op_no_param(opcode),
            0x59 => self.op_no_param(opcode),
            0x5A => self.op_no_param(opcode),
            0x5B => self.op_no_param(opcode),
            0x5C => self.op_no_param(opcode),
            0x5D => self.op_no_param(opcode),
            0x5E => self.op_no_param(opcode),
            0x5F => self.op_no_param(opcode),
            0x60 => self.op_no_param(opcode),
            0x61 => self.op_no_param(opcode),
            0x62 => self.op_no_param(opcode),
            0x63 => self.op_no_param(opcode),
            0x64 => self.op_no_param(opcode),
            0x65 => self.op_no_param(opcode),
            0x66 => self.op_no_param(opcode),
            0x67 => self.op_no_param(opcode),
            0x68 => self.op_no_param(opcode),
            0x69 => self.op_no_param(opcode),
            0x6A => self.op_no_param(opcode),
            0x6B => self.op_no_param(opcode),
            0x6C => self.op_no_param(opcode),
            0x6D => self.op_no_param(opcode),
            0x6E => self.op_no_param(opcode),
            0x6F => self.op_no_param(opcode),
            0x0A => self.op_no_param(opcode),
            0x1A => self.op_no_param(opcode),
            0x7E => self.op_no_param(opcode),
            0x70 => self.op_no_param(opcode),
            0x71 => self.op_no_param(opcode),
            0x72 => self.op_no_param(opcode),
            0x73 => self.op_no_param(opcode),
            0x74 => self.op_no_param(opcode),
            0x75 => self.op_no_param(opcode),
            0x02 => self.op_no_param(opcode),
            0x12 => self.op_no_param(opcode),
            0x77 => self.op_no_param(opcode),
            0xF9 => self.op_no_param(opcode),
            0xE2 => self.op_no_param(opcode),
            0xF2 => self.op_no_param(opcode),
            0x22 => self.op_no_param(opcode),
            0x2A => self.op_no_param(opcode),
            0x32 => self.op_no_param(opcode),
            0x3A => self.op_no_param(opcode),
            0x36 => self.op_no_param(opcode),
            0xE0 => self.op_no_param(opcode),
            0xF0 => self.op_no_param(opcode),
            0xF8 => self.op_no_param(opcode),
            _ => (),
        }
    }

    fn op_param_16_bit(&mut self, opcode: u16, param: u16) {
        match opcode {
            0x01 => set_bc!(self, param),
            0x11 => set_de!(self, param),
            0x21 => set_hl!(self, param),
            0x31 => set_sp!(self, param),
            0xFA => set_a!(self, self.mem.get_ram_u8(param.swap_bytes() as usize)),
            _ => (),
        };
    }

    fn op_param_8_bit(&mut self, opcode: u16, param: u8) {
        match opcode {
            0x06 => set_b!(self, param),
            0x0E => set_c!(self, param),
            0x16 => set_d!(self, param),
            0x1E => set_e!(self, param),
            0x26 => set_h!(self, param),
            0x2E => set_l!(self, param),
            0x3E => set_a!(self, param),
            0xEA => self.mem.set_ram_u8(param as usize, get_a!(self)),
            0x36 => self.mem.set_ram_u8(get_hl!(self) as usize, param as u8),
            0xE0 => self.mem.set_ram_u8((param as u16 + 0xFF00) as usize, get_a!(self)),
            0xF0 => set_a!(self, self.mem.get_ram_u8((param as u16 + 0xFF00) as usize)),

            // this doesnt really work and is complicated so i'll figure it out later
            0xF8 => { set_hl!(self, get_sp!(self) + param as u16); unset_z_flag!(self); unset_n_flag!(self);},
            _ => (),
        };
    }

    fn op_no_param(&mut self, opcode: u16) {
        match opcode {
            0x78 => set_a!(self, get_b!(self)),
            0x79 => set_a!(self, get_c!(self)),
            0x7A => set_a!(self, get_d!(self)),
            0x7B => set_a!(self, get_e!(self)),
            0x7C => set_a!(self, get_h!(self)),
            0x7D => set_a!(self, get_l!(self)),
            0x7E => set_a!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x7F => set_a!(self, get_a!(self)),
            0x40 => set_b!(self, get_b!(self)),
            0x41 => set_b!(self, get_c!(self)),
            0x42 => set_b!(self, get_d!(self)),
            0x43 => set_b!(self, get_e!(self)),
            0x44 => set_b!(self, get_h!(self)),
            0x45 => set_b!(self, get_l!(self)),
            0x46 => set_b!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x47 => set_b!(self, get_a!(self)),
            0x48 => set_c!(self, get_b!(self)),
            0x49 => set_c!(self, get_c!(self)),
            0x4A => set_c!(self, get_d!(self)),
            0x4B => set_c!(self, get_e!(self)),
            0x4C => set_c!(self, get_h!(self)),
            0x4D => set_c!(self, get_l!(self)),
            0x4E => set_c!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x4F => set_c!(self, get_a!(self)),
            0x50 => set_d!(self, get_b!(self)),
            0x51 => set_d!(self, get_c!(self)),
            0x52 => set_d!(self, get_d!(self)),
            0x53 => set_d!(self, get_e!(self)),
            0x54 => set_d!(self, get_h!(self)),
            0x55 => set_d!(self, get_l!(self)),
            0x56 => set_d!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x57 => set_d!(self, get_a!(self)),
            0x58 => set_e!(self, get_b!(self)),
            0x59 => set_e!(self, get_c!(self)),
            0x5A => set_e!(self, get_d!(self)),
            0x5B => set_e!(self, get_e!(self)),
            0x5C => set_e!(self, get_h!(self)),
            0x5D => set_e!(self, get_l!(self)),
            0x5E => set_e!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x5F => set_e!(self, get_a!(self)),
            0x60 => set_h!(self, get_b!(self)),
            0x61 => set_h!(self, get_c!(self)),
            0x62 => set_h!(self, get_d!(self)),
            0x63 => set_h!(self, get_e!(self)),
            0x64 => set_h!(self, get_h!(self)),
            0x65 => set_h!(self, get_l!(self)),
            0x66 => set_h!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x67 => set_h!(self, get_a!(self)),
            0x68 => set_l!(self, get_b!(self)),
            0x69 => set_l!(self, get_c!(self)),
            0x6A => set_l!(self, get_d!(self)),
            0x6B => set_l!(self, get_e!(self)),
            0x6C => set_l!(self, get_h!(self)),
            0x6D => set_l!(self, get_l!(self)),
            0x6E => set_l!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x6F => set_l!(self, get_a!(self)),
            0x0A => set_a!(self, self.mem.get_ram_u8(get_bc!(self) as usize)),
            0x1A => set_a!(self, self.mem.get_ram_u8(get_de!(self) as usize)),
            0x7E => set_a!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            0x70 => self.mem.set_ram_u8(get_hl!(self) as usize, get_b!(self)),
            0x71 => self.mem.set_ram_u8(get_hl!(self) as usize, get_c!(self)),
            0x72 => self.mem.set_ram_u8(get_hl!(self) as usize, get_d!(self)),
            0x73 => self.mem.set_ram_u8(get_hl!(self) as usize, get_e!(self)),
            0x74 => self.mem.set_ram_u8(get_hl!(self) as usize, get_h!(self)),
            0x75 => self.mem.set_ram_u8(get_hl!(self) as usize, get_l!(self)),
            0x02 => self.mem.set_ram_u8(get_bc!(self) as usize, get_a!(self)),
            0x12 => self.mem.set_ram_u8(get_de!(self) as usize, get_a!(self)),
            0x77 => self.mem.set_ram_u8(get_hl!(self) as usize, get_a!(self)),
            0xF9 => set_sp!(self, get_hl!(self)),
            0xE2 => self.mem.set_ram_u8((get_c!(self) as u16 + 0xFF00) as usize, get_a!(self)),
            0xF2 => set_a!(self, self.mem.get_ram_u8((get_c!(self) as u16 + 0xFF00) as usize)),

            0x22 => { self.mem.set_ram_u8(get_hl!(self) as usize, get_a!(self));
                      set_hl!(self, get_hl!(self) + 1);},
            0x2A => { set_a!(self, self.mem.get_ram_u8(get_hl!(self) as usize));
                      set_hl!(self, get_hl!(self) + 1);},

            0x32 => { self.mem.set_ram_u8(get_hl!(self) as usize, get_a!(self));
                      set_hl!(self, get_hl!(self) - 1);},
            0x3A => { set_a!(self, self.mem.get_ram_u8(get_hl!(self) as usize));
                      set_hl!(self, get_hl!(self) - 1);},

            _ => (),
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
    }

    pub fn test_opcodes(&mut self) {
        self.op_param_8_bit(0x06, 0b00000001);
        assert_eq!(get_b!(self), 0b00000001);
        self.op_param_8_bit(0x0E, 0b00000010);
        assert_eq!(get_c!(self), 0b00000010);
        assert_eq!(get_bc!(self), 0b0000000100000010);

        self.op_param_8_bit(0x16, 30);
        assert_eq!(get_d!(self), 30);
        self.op_param_8_bit(0x1E, 40);
        assert_eq!(get_e!(self), 40);

        self.op_param_8_bit(0x26, 0b00000100);
        assert_eq!(get_h!(self), 0b00000100);
        self.op_param_8_bit(0x2E, 0b10101010);
        assert_eq!(get_l!(self), 0b10101010);
        assert_eq!(get_hl!(self), 0b0000010010101010);

        self.mem.set_ram_u8(0b0000010010101010, 60);

        self.op_no_param(0x7E);
        assert_eq!(get_a!(self), 60);
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
        self.mem.set_cartridge(0, 0xCB);
        self.mem.set_cartridge(1, 0x10);
        self.mem.set_cartridge(2, 0x20);

        set_pc!(self, 0);
        assert_eq!(self.get_opcode(), 0xCB10);
        assert_eq!(get_pc!(self), 2);
        assert_eq!(self.get_opcode(), 0x20);
        assert_eq!(get_pc!(self), 3);
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
