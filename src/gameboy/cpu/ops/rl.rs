// RL n rotate n left through carrry flag (C flag)

macro_rules! rl_a {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_a!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) << 1) + tmp);
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rl_b {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_b!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_b!($self_, (get_b!($self_) << 1) + tmp);
        if get_b!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rl_c {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_c!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_c!($self_, (get_c!($self_) << 1) + tmp);
        if get_c!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rl_d {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_d!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_d!($self_, (get_d!($self_) << 1) + tmp);
        if get_d!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rl_e {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_e!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_e!($self_, (get_e!($self_) << 1) + tmp);
        if get_e!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rl_h {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_h!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_h!($self_, (get_h!($self_) << 1) + tmp);
        if get_h!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! rl_l {
    ($self_: ident) => {{
        let tmp = get_c_flag!($self_);
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_l!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_l!($self_, (get_l!($self_) << 1) + tmp);
        if get_l!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
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

