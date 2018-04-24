// SWAP n
//
macro_rules! swap_a {
    ($self_: ident) => {{
        set_a!($self_, (get_a!($self_) >> 4) + (get_a!($self_) << 4));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}

macro_rules! swap_b {
    ($self_: ident) => {{
        set_b!($self_, (get_b!($self_) >> 4) + (get_b!($self_) << 4));
        if get_b!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}

macro_rules! swap_c {
    ($self_: ident) => {{
        set_c!($self_, (get_c!($self_) >> 4) + (get_c!($self_) << 4));
        if get_c!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}

macro_rules! swap_d {
    ($self_: ident) => {{
        set_d!($self_, (get_d!($self_) >> 4) + (get_d!($self_) << 4));
        if get_d!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}

macro_rules! swap_e {
    ($self_: ident) => {{
        set_e!($self_, (get_e!($self_) >> 4) + (get_e!($self_) << 4));
        if get_e!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}

macro_rules! swap_h {
    ($self_: ident) => {{
        set_h!($self_, (get_h!($self_) >> 4) + (get_h!($self_) << 4));
        if get_h!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}

macro_rules! swap_l {
    ($self_: ident) => {{
        set_l!($self_, (get_l!($self_) >> 4) + (get_l!($self_) << 4));
        if get_l!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}

macro_rules! swap_hl_val {
    ($self_: ident) => {{
        let old_val = $self_.mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = (old_val >> 4) + (old_val << 4);
        $self_.mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
    }};
}
