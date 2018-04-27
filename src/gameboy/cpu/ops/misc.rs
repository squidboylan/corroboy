// DAA
// Decimal adjust register A

macro_rules! daa {
    ($self_: ident) => {{
        if get_a!($self_) & 0b00001111 > 9 || get_h_flag!($self_) == 1 {
            set_a!($self_, get_a!($self_) + 0x06);
        }
        if get_a!($self_) & (0b11110000 >> 4) > 9 || get_c_flag!($self_) == 1 {
            set_a!($self_, get_a!($self_) + 0x60);
            set_c_flag!($self_);
        }
        else { unset_c_flag!($self_); }
        unset_h_flag!($self_);
        if get_a!($self_) == 0 { set_z_flag!($self_) }
        else { unset_z_flag!($self_) }
    }};
}

// CPL
// compliment A register

macro_rules! cpl {
    ($self_: ident) => {{
        set_a!($self_, !get_a!($self_));
        set_n_flag!($self_);
        set_h_flag!($self_);
    }};
}

// CALL nn

macro_rules! call_nn {
    ($self_: ident, $mem: ident, $param: expr) => {{
        $mem.push_u16(get_sp_mut!($self_), get_pc!($self_));
        set_pc!($self_, $param);
    }};
}

// CCF
// compliment carry flag

macro_rules! ccf {
    ($self_: ident) => {{
        if get_c_flag!($self_) == 1 { unset_c_flag!($self_); }
        else { set_c_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
    }};
}

// SCF
// set carry flag

macro_rules! scf {
    ($self_: ident) => {{
        set_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
    }};
}

// noop - do nothing

macro_rules! nop {
    ($self_: ident) => {{
        ();
    }};
}

// HALT - power down cpu until an interrupt occurs.
// NOT IMPLEMENTED, need to figure out how interrupts will be emulated first

macro_rules! halt {
    ($self_: ident) => {{
        ();
    }};
}

