#[allow(unused_macros)]
macro_rules! set_a {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.af.split.upper = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_f {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.af.split.lower = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_af {
    ($self_: ident, $x:expr) => {
        $self_.af.whole = $x;
    };
}

#[allow(unused_macros)]
macro_rules! set_b {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.bc.split.upper = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_c {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.bc.split.lower = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_bc {
    ($self_: ident, $x:expr) => {
        $self_.bc.whole = $x;
    };
}

#[allow(unused_macros)]
macro_rules! set_d {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.de.split.upper = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_e {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.de.split.lower = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_de {
    ($self_: ident, $x:expr) => {
        $self_.de.whole = $x;
    };
}

#[allow(unused_macros)]
macro_rules! set_h {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.hl.split.upper = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_l {
    ($self_: ident, $x:expr) => {
        unsafe {
            $self_.hl.split.lower = $x;
        }
    };
}

#[allow(unused_macros)]
macro_rules! set_hl {
    ($self_: ident, $x:expr) => {
        $self_.hl.whole = $x;
    };
}

#[allow(unused_macros)]
macro_rules! set_sp {
    ($self_: ident, $x:expr) => {
        $self_.sp.whole = $x;
    };
}

#[allow(unused_macros)]
macro_rules! set_pc {
    ($self_: ident, $x:expr) => {
        $self_.pc.whole = $x;
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

macro_rules! get_mut_a {
    ($self_: ident) => {
        unsafe {
            &mut $self_.af.split.upper
        }
    }
}

macro_rules! get_mut_f {
    ($self_: ident) => {
        unsafe {
            &mut $self_.af.split.lower
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

macro_rules! get_mut_af {
    ($self_: ident) => {
        unsafe {
            &mut $self_.af.whole
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

macro_rules! get_mut_b {
    ($self_: ident) => {
        unsafe {
            &mut $self_.bc.split.upper
        }
    }
}

macro_rules! get_mut_c {
    ($self_: ident) => {
        unsafe {
            &mut $self_.bc.split.lower
        }
    }
}

macro_rules! get_mut_bc {
    ($self_: ident) => {
        unsafe {
            &mut $self_.bc.whole
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

macro_rules! get_mut_d {
    ($self_: ident) => {
        unsafe {
            &mut $self_.de.split.upper
        }
    }
}

macro_rules! get_mut_e {
    ($self_: ident) => {
        unsafe {
            &mut $self_.de.split.lower
        }
    }
}

macro_rules! get_mut_de {
    ($self_: ident) => {
        unsafe {
            &mut $self_.de.whole
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

macro_rules! get_mut_h {
    ($self_: ident) => {
        unsafe {
            &mut $self_.hl.split.upper
        }
    }
}

macro_rules! get_mut_l {
    ($self_: ident) => {
        unsafe {
            &mut $self_.hl.split.lower
        }
    }
}

macro_rules! get_mut_hl {
    ($self_: ident) => {
        unsafe {
            &mut $self_.hl.whole
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

macro_rules! get_mut_sp {
    ($self_: ident) => {
        unsafe {
            &mut $self_.sp.whole
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

macro_rules! get_mut_pc {
    ($self_: ident) => {
        unsafe {
            &mut $self_.pc.whole
        }
    }
}

#[allow(unused_macros)]
macro_rules! get_z_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b10000000) >> 7
        }
    }
}

#[allow(unused_macros)]
macro_rules! get_n_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b01000000) >> 6
        }
    }
}

#[allow(unused_macros)]
macro_rules! get_h_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b00100000) >> 5
        }
    }
}

#[allow(unused_macros)]
macro_rules! get_c_flag {
    ($self_: ident) => {
        unsafe {
            ($self_.af.split.lower & 0b00010000) >> 4
        }
    }
}

#[allow(unused_macros)]
macro_rules! set_z_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b10000000
        }
    }
}

#[allow(unused_macros)]
macro_rules! set_n_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b01000000
        }
    }
}

#[allow(unused_macros)]
macro_rules! set_h_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b00100000
        }
    }
}

#[allow(unused_macros)]
macro_rules! set_c_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower |= 0b00010000
        }
    }
}

#[allow(unused_macros)]
macro_rules! unset_z_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower &= 0b01111111
        }
    }
}

#[allow(unused_macros)]
macro_rules! unset_n_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower &= 0b10111111
        }
    }
}

#[allow(unused_macros)]
macro_rules! unset_h_flag {
    ($self_: ident) => {
        unsafe {
            $self_.af.split.lower &= 0b11011111
        }
    }
}

#[allow(unused_macros)]
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
pub struct Reg8Bit {
    pub lower: u8,
    pub upper: u8,
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
pub struct Reg8Bit {
    pub upper: u8,
    pub lower: u8,
}

#[derive(Clone, Copy)]
pub union Reg16Bit {
    pub split: Reg8Bit,
    pub whole: u16,
}
