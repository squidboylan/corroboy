// XOR n

macro_rules! xor_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ get_a!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}

macro_rules! xor_b {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ get_b!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}


macro_rules! xor_c {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ get_c!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}

macro_rules! xor_d {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ get_d!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}

macro_rules! xor_e {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ get_e!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}

macro_rules! xor_h {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ get_h!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}

macro_rules! xor_l {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ get_l!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}

macro_rules! xor_hl_val {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) ^ $self_.mem.get_mem_u8(get_hl!($self_) as usize));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
    }};
}
