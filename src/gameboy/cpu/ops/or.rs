// OR n

macro_rules! or_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | get_a!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        1
    }};
}

macro_rules! or_b {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | get_b!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        1
    }};
}


macro_rules! or_c {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | get_c!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        1
    }};
}

macro_rules! or_d {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | get_d!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        1
    }};
}

macro_rules! or_e {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | get_e!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        1
    }};
}

macro_rules! or_h {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | get_h!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        1
    }};
}

macro_rules! or_l {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | get_l!($self_));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        1
    }};
}

macro_rules! or_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_a!($self_, get_a!($self_) | $mem.get_mem_u8(get_hl!($self_) as usize));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        2
    }};
}

macro_rules! or_param {
    ($self_: ident, $param: expr) => {{
        set_a!($self_, get_a!($self_) | $param);
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_c_flag!($self_);
        unset_h_flag!($self_);
        2
    }};
}
