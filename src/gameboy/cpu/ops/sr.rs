// SRL n

pub fn srl_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = *reg >> 1;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

macro_rules! srl_hl_val {
    ($self_: ident, $mem: ident) => {{
        let val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = val >> 1;
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (val & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

// SRA n

pub fn sra_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg >> 1) + (*reg & 0b10000000);
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

macro_rules! sra_hl_val {
    ($self_: ident, $mem: ident) => {{
        let val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = (val >> 1) + (val & 0b10000000);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (val & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

