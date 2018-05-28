// SRA N

macro_rules! srl_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_a!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) >> 1));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! srl_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_b!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_b!($self_, (get_b!($self_) >> 1));
        if get_b!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! srl_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_c!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_c!($self_, (get_c!($self_) >> 1));
        if get_c!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! srl_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_d!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_d!($self_, (get_d!($self_) >> 1));
        if get_d!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! srl_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_e!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_e!($self_, (get_e!($self_) >> 1));
        if get_e!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! srl_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_h!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_h!($self_, (get_h!($self_) >> 1));
        if get_h!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! srl_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_l!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_l!($self_, (get_l!($self_) >> 1));
        if get_l!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
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

// SRA

macro_rules! sra_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_a!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) >> 1) + (get_a!($self_) & 0b10000000));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sra_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_b!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_b!($self_, (get_b!($self_) >> 1) + (get_b!($self_) & 0b10000000));
        if get_b!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sra_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_c!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_c!($self_, (get_c!($self_) >> 1) + (get_c!($self_) & 0b10000000));
        if get_c!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sra_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_d!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_d!($self_, (get_d!($self_) >> 1) + (get_d!($self_) & 0b10000000));
        if get_d!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sra_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_e!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_e!($self_, (get_e!($self_) >> 1) + (get_e!($self_) & 0b10000000));
        if get_e!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sra_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_h!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_h!($self_, (get_h!($self_) >> 1) + (get_h!($self_) & 0b10000000));
        if get_h!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sra_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_l!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_l!($self_, (get_l!($self_) >> 1) + (get_l!($self_) & 0b10000000));
        if get_l!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
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
