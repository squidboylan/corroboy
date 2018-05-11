// RR n rotate n right through carrry flag (C flag)

macro_rules! rr_a {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_a!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) >> 1) + tmp << 7);
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rr_b {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_b!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_b!($self_, (get_b!($self_) >> 1) + tmp << 7);
        if get_b!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rr_c {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_c!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_c!($self_, (get_c!($self_) >> 1) + tmp << 7);
        if get_c!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rr_d {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_d!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_d!($self_, (get_d!($self_) >> 1) + tmp << 7);
        if get_d!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rr_e {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_e!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_e!($self_, (get_e!($self_) >> 1) + tmp << 7);
        if get_e!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rr_h {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_h!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_h!($self_, (get_h!($self_) >> 1) + tmp << 7);
        if get_h!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rr_l {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_l!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_l!($self_, (get_l!($self_) >> 1) + tmp << 7);
        if get_l!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

// RRCA


macro_rules! rrca {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_a!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) >> 1));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        1
    }};
}

// RRA

macro_rules! rra {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if (get_a!($self_) & 0b00000001) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) >> 1) | tmp << 7);
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        1
    }};
}

