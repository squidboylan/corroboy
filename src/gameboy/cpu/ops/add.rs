// ADD HL,n

macro_rules! add_hl_bc {
    ($self_: ident) => {{ let old_hl = get_hl!($self_);
        set_hl!($self_, get_hl!($self_) + get_bc!($self_));
        unset_n_flag!($self_);
        if old_hl > get_hl!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_hl & 0b0000111111111111) + (get_bc!($self_) & 0b0000111111111111) >= 0b0001000000000000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_hl_de {
    ($self_: ident) => {{ let old_hl = get_hl!($self_);
        set_hl!($self_, get_hl!($self_) + get_de!($self_));
        unset_n_flag!($self_);
        if old_hl > get_hl!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_hl & 0b0000111111111111) + (get_de!($self_) & 0b0000111111111111) >= 0b0001000000000000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_hl_hl {
    ($self_: ident) => {{ let old_hl = get_hl!($self_);
        set_hl!($self_, get_hl!($self_) + get_hl!($self_));
        unset_n_flag!($self_);
        if old_hl > get_hl!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_hl & 0b0000111111111111) + (get_hl!($self_) & 0b0000111111111111) >= 0b0001000000000000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_hl_sp {
    ($self_: ident) => {{ let old_hl = get_hl!($self_);
        set_hl!($self_, get_hl!($self_) + get_sp!($self_));
        unset_n_flag!($self_);
        if old_hl > get_hl!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_hl & 0b0000111111111111) + (get_sp!($self_) & 0b0000111111111111) >= 0b0001000000000000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

// ADD A, n

macro_rules! add_a_a {
    ($self_: ident) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + get_a!($self_));
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (get_a!($self_) & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_b {
    ($self_: ident) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + get_b!($self_));
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (get_b!($self_) & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_c {
    ($self_: ident) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + get_c!($self_));
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (get_c!($self_) & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_d {
    ($self_: ident) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + get_d!($self_));
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (get_d!($self_) & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_e {
    ($self_: ident) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + get_e!($self_));
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (get_e!($self_) & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_h {
    ($self_: ident) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + get_h!($self_));
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (get_h!($self_) & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_l {
    ($self_: ident) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + get_l!($self_));
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (get_l!($self_) & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_hl_val {
    ($self_: ident, $mem: ident) => {{ let old_a = get_a!($self_);
        let hl_val = $mem.get_mem_u8(get_hl!($self_) as usize);
        set_a!($self_, get_a!($self_) + hl_val);
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + (hl_val & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}

macro_rules! add_a_param {
    ($self_: ident, $param: expr) => {{ let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) + $param);
        unset_n_flag!($self_);
        if old_a > get_a!($self_) { set_c_flag!($self_); }
        else{ unset_c_flag!($self_); }
        if (old_a & 0b00001111) + ($param & 0b00001111) >= 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
    }};
}
