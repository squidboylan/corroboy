// INC n
macro_rules! inc_a {
    ($self_: ident) => {{ unset_n_flag!($self_); set_a!($self_, get_a!($self_) + 1);
            if get_a!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if get_a!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! inc_b {
    ($self_: ident) => {{ unset_n_flag!($self_); set_b!($self_, get_b!($self_) + 1);
            if get_b!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if get_b!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! inc_c {
    ($self_: ident) => {{ unset_n_flag!($self_); set_c!($self_, get_c!($self_) + 1);
            if get_c!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if get_c!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! inc_d {
    ($self_: ident) => {{ unset_n_flag!($self_); set_d!($self_, get_d!($self_) + 1);
            if get_d!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if get_d!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! inc_e {
    ($self_: ident) => {{ unset_n_flag!($self_); set_e!($self_, get_e!($self_) + 1);
            if get_e!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if get_e!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! inc_h {
    ($self_: ident) => {{ unset_n_flag!($self_); set_h!($self_, get_h!($self_) + 1);
            if get_h!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if get_h!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

macro_rules! inc_l {
    ($self_: ident) => {{ unset_n_flag!($self_); set_l!($self_, get_l!($self_) + 1);
            if get_l!($self_) == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if get_l!($self_) & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

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
    ($self_: ident) => {{ let val = $self_.mem.get_mem_u8(get_hl!($self_) as usize) - 1;
            $self_.mem.set_mem_u8(get_hl!($self_) as usize, val);
            unset_n_flag!($self_);
            if val == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            if (val & 0b00011111) == 0b00001111 { unset_h_flag!($self_); }
            else { unset_h_flag!($self_); }
    }};
}

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
    }};
}

macro_rules! cp_hl_val {
    ($self_: ident) => {{ let val = $self_.mem.get_mem_u8(get_hl!($self_) as usize);
            let tmp = get_a!($self_) - val;
            if tmp == 0 { set_z_flag!($self_); }
            else { unset_z_flag!($self_); }
            set_n_flag!($self_);
            if get_a!($self_) < tmp { set_c_flag!($self_); }
            else { unset_c_flag!($self_); }
            if (get_a!($self_) & 0b00001111) < (val & 0b00001111) { unset_h_flag!($self_); }
            else { set_h_flag!($self_); }
    }};
}


