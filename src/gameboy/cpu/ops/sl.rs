// CB extended ops
// SLA N
// Shift N left, old bit 7 goes into C flag

macro_rules! sla_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_a!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_a!($self_, (get_a!($self_) << 1));
        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sla_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_b!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_b!($self_, (get_b!($self_) << 1));
        if get_b!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sla_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_c!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_c!($self_, (get_c!($self_) << 1));
        if get_c!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sla_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_d!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_d!($self_, (get_d!($self_) << 1));
        if get_d!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sla_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_e!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_e!($self_, (get_e!($self_) << 1));
        if get_e!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sla_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_h!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_h!($self_, (get_h!($self_) << 1));
        if get_h!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sla_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        if ((get_l!($self_) & 0b10000000) >> 7) == 1 { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
        set_l!($self_, (get_l!($self_) << 1));
        if get_l!($self_) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! sla_hl_val {
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

