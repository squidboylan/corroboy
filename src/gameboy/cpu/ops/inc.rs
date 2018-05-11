// INC n
macro_rules! inc_a {
    ($self_: ident) => {{ unset_n_flag!($self_); set_a!($self_, get_a!($self_) + 1);
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if get_a!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        1
    }};
}

macro_rules! inc_b {
    ($self_: ident) => {{ unset_n_flag!($self_); set_b!($self_, get_b!($self_) + 1);
        if get_b!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if get_b!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        1
    }};
}

macro_rules! inc_c {
    ($self_: ident) => {{ unset_n_flag!($self_); set_c!($self_, get_c!($self_) + 1);
        if get_c!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if get_c!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        1
    }};
}

macro_rules! inc_d {
    ($self_: ident) => {{ unset_n_flag!($self_); set_d!($self_, get_d!($self_) + 1);
        if get_d!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if get_d!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        1
    }};
}

macro_rules! inc_e {
    ($self_: ident) => {{ unset_n_flag!($self_); set_e!($self_, get_e!($self_) + 1);
        if get_e!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if get_e!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        1
    }};
}

macro_rules! inc_h {
    ($self_: ident) => {{ unset_n_flag!($self_); set_h!($self_, get_h!($self_) + 1);
        if get_h!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if get_h!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        1
    }};
}

macro_rules! inc_l {
    ($self_: ident) => {{ unset_n_flag!($self_); set_l!($self_, get_l!($self_) + 1);
        if get_l!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if get_l!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        1
    }};
}

macro_rules! inc_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        let old_val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = old_val + 1;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if new_val & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        3
    }};
}


// INC nn

macro_rules! inc_bc {
    ($self_: ident) => {{
        set_bc!($self_, get_bc!($self_) + 1);
        2
    }};
}

macro_rules! inc_de {
    ($self_: ident) => {{
        set_de!($self_, get_de!($self_) + 1);
        2
    }};
}

macro_rules! inc_hl {
    ($self_: ident) => {{
        set_hl!($self_, get_hl!($self_) + 1);
        2
    }};
}

macro_rules! inc_sp {
    ($self_: ident) => {{
        set_sp!($self_, get_sp!($self_) + 1);
        2
    }};
}

