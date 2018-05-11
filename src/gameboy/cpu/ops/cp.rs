// CP n
macro_rules! cp_a {
    ($self_: ident) => {{ let tmp = get_a!($self_) - get_a!($self_);
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (get_a!($self_) & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            1
    }};
}

macro_rules! cp_b {
    ($self_: ident) => {{ let tmp = get_a!($self_) - get_b!($self_);
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (get_b!($self_) & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            1
    }};
}

macro_rules! cp_c {
    ($self_: ident) => {{ let tmp = get_a!($self_) - get_c!($self_);
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (get_c!($self_) & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            1
    }};
}

macro_rules! cp_d {
    ($self_: ident) => {{ let tmp = get_a!($self_) - get_d!($self_);
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (get_d!($self_) & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            1
    }};
}

macro_rules! cp_e {
    ($self_: ident) => {{ let tmp = get_a!($self_) - get_e!($self_);
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (get_e!($self_) & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            1
    }};
}

macro_rules! cp_h {
    ($self_: ident) => {{ let tmp = get_a!($self_) - get_h!($self_);
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (get_h!($self_) & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            1
    }};
}

macro_rules! cp_l {
    ($self_: ident) => {{ let tmp = get_a!($self_) - get_l!($self_);
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (get_l!($self_) & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            1
    }};
}

macro_rules! cp_n {
    ($self_: ident, $param: expr) => {{ let tmp = get_a!($self_) - $param;
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < ($param & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            2
    }};
}

macro_rules! cp_hl_val {
    ($self_: ident, $mem: ident) => {{ let val = $mem.get_mem_u8(get_hl!($self_) as usize);
            let tmp = get_a!($self_) - val;
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (val & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
            2
    }};
}

