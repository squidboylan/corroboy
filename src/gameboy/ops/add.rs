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
