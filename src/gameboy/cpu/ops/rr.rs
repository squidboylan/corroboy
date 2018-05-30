// RR n rotate n right through carrry flag (C flag)

pub fn rr_reg(reg: &mut u8, flags: &mut u8) {
    let tmp = (*flags & 0b00010000) >> 4;
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg >> 1) + (tmp << 7);
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

macro_rules! rr_hl_val {
    ($self_: ident, $mem: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        let val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = (val >> 1) + (tmp << 7);
        if val & 0b00000001 == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

// RRC n

pub fn rrc_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b00000001 == 1 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg >> 1);
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

macro_rules! rrc_hl_val {
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

