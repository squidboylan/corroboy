// RL n rotate n left through carrry flag (C flag)

pub fn rl_reg(reg: &mut u8, flags: &mut u8) {
    let tmp = (*flags & 0b00010000) >> 4;
    *flags &= 0b10010000;
    if *reg & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg << 1) + tmp;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

macro_rules! rl_hl_val {
    ($self_: ident, $mem: ident) => {{
        let tmp = get_c_flag!($self_);
        let val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = (val << 1) + tmp;
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((val & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

// RLCA rotate A left old bit 7 in C flag

macro_rules! rlca {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_a!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) << 1));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        1
    }};
}

// RLA rotate A left through carrry flag (C flag)

macro_rules! rla {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_a!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) << 1) + tmp);
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        1
    }};
}

// CB extended ops
// RLC A rotate A left old bit 7 in C flag

pub fn rlc_reg(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10010000;
    if *reg & 0b10000000 != 0 { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    *reg = (*reg << 1);
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
}

macro_rules! rlc_hl_val {
    ($self_: ident, $mem: ident) => {{
        let val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = val << 1;
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (val & 0b10000000) >> 7 == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

