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

    fn exec_opcode(&mut self, opcode: u8, param: u8) {
        match opcode {
            0x06 => set_b!(self, param),
            0x0E => set_c!(self, param),
            0x16 => set_d!(self, param),
            0x1E => set_e!(self, param),
            0x26 => set_h!(self, param),
            0x2E => set_l!(self, param),

            0x0A => set_a!(self, self.mem.get_ram_u8(get_bc!(self) as usize)),
            0x1A => set_a!(self, self.mem.get_ram_u8(get_de!(self) as usize)),
            0x7E => set_a!(self, self.mem.get_ram_u8(get_hl!(self) as usize)),
            // 0xFA => set_a!(self, self.mem.get_ram_u8((param2 | param) as usize)),
            0x3E => set_a!(self, param),

            0x02 => self.mem.set_ram_u8(get_bc!(self) as usize, get_a!(self)),
            0x12 => self.mem.set_ram_u8(get_de!(self) as usize, get_a!(self)),
            0x77 => self.mem.set_ram_u8(get_hl!(self) as usize, get_a!(self)),
            0xEA => self.mem.set_ram_u8(param as usize, get_a!(self)),

            0xE2 => self.mem.set_ram_u8((get_c!(self) + 0xFF00) as usize, get_a!(self)),
            0xF2 => set_a!(self, self.mem.get_ram_u8((get_c!(self) + 0xFF00) as usize)),

            0x32 => { self.mem.set_ram_u8(get_hl!(self) as usize, get_a!(self));
                      set_hl!(self, get_hl!(self) - 1);},

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
            0x70 => self.mem.set_ram_u8(get_hl!(self) as usize, get_b!(self)),
            0x71 => self.mem.set_ram_u8(get_hl!(self) as usize, get_c!(self)),
            0x72 => self.mem.set_ram_u8(get_hl!(self) as usize, get_d!(self)),
            0x73 => self.mem.set_ram_u8(get_hl!(self) as usize, get_e!(self)),
            0x74 => self.mem.set_ram_u8(get_hl!(self) as usize, get_h!(self)),
            0x75 => self.mem.set_ram_u8(get_hl!(self) as usize, get_l!(self)),
            0x36 => self.mem.set_ram_u8(get_hl!(self) as usize, param),
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
    }

    pub fn test_opcodes(&mut self) {
        self.exec_opcode(0x06, 0b00000001);
        assert_eq!(get_b!(self), 0b00000001);
        self.exec_opcode(0x0E, 0b00000010);
        assert_eq!(get_c!(self), 0b00000010);
        assert_eq!(get_bc!(self), 0b0000000100000010);

        self.exec_opcode(0x16, 30);
        assert_eq!(get_d!(self), 30);
        self.exec_opcode(0x1E, 40);
        assert_eq!(get_e!(self), 40);

        self.exec_opcode(0x26, 0b00000100);
        assert_eq!(get_h!(self), 0b00000100);
        self.exec_opcode(0x2E, 0b10101010);
        assert_eq!(get_l!(self), 0b10101010);
        assert_eq!(get_hl!(self), 0b0000010010101010);

        self.mem.set_ram_u8(0b0000010010101010, 60);

        self.exec_opcode(0x7E, 0);
        assert_eq!(get_a!(self), 60);
    }
}

#[test]
fn test_cpu() {
    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_registers();

    // Get a new CPU in to start at a known state
    let mut derp = Cpu::new();
    derp.test_opcodes();
}
