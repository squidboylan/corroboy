// DEC n
macro_rules! dec_a {
    ($self_: ident) => {{ set_a!($self_, get_a!($self_) - 1);
            unset_n_flag!($self_);
            if get_a!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (get_a!($self_) & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! dec_b {
    ($self_: ident) => {{ set_b!($self_, get_b!($self_) - 1);
            unset_n_flag!($self_);
            if get_b!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (get_b!($self_) & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! dec_c {
    ($self_: ident) => {{ set_c!($self_, get_c!($self_) - 1);
            unset_n_flag!($self_);
            if get_c!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (get_c!($self_) & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! dec_d {
    ($self_: ident) => {{ set_d!($self_, get_d!($self_) - 1);
            unset_n_flag!($self_);
            if get_d!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (get_d!($self_) & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! dec_e {
    ($self_: ident) => {{ set_e!($self_, get_e!($self_) - 1);
            unset_n_flag!($self_);
            if get_e!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (get_e!($self_) & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! dec_h {
    ($self_: ident) => {{ set_h!($self_, get_h!($self_) - 1);
            unset_n_flag!($self_);
            if get_h!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (get_h!($self_) & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! dec_l {
    ($self_: ident) => {{ set_l!($self_, get_l!($self_) - 1);
            unset_n_flag!($self_);
            if get_l!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (get_l!($self_) & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! dec_hl_val {
    ($self_: ident, $mem: ident) => {{ let val = $mem.get_mem_u8(get_hl!($self_) as usize) - 1;
            $mem.set_mem_u8(get_hl!($self_) as usize, val);
            unset_n_flag!($self_);
            if val == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (val & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

// DEC nn

macro_rules! dec_bc {
    ($self_: ident) => {{
        set_bc!($self_, get_bc!($self_) - 1);
    }};
}

macro_rules! dec_de {
    ($self_: ident) => {{
        set_de!($self_, get_de!($self_) - 1);
    }};
}

macro_rules! dec_hl {
    ($self_: ident) => {{
        set_hl!($self_, get_hl!($self_) - 1);
    }};
}

macro_rules! dec_sp {
    ($self_: ident) => {{
        set_sp!($self_, get_sp!($self_) - 1);
    }};
}

