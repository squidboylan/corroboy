// SUB n

macro_rules! sub_a_a {
    ($self_: ident) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - get_a!($self_));
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 1;
    }};
}

macro_rules! sub_a_b {
    ($self_: ident) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - get_b!($self_));
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 1;
    }};
}

macro_rules! sub_a_c {
    ($self_: ident) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - get_c!($self_));
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 1;
    }};
}

macro_rules! sub_a_d {
    ($self_: ident) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - get_d!($self_));
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 1;
    }};
}

macro_rules! sub_a_e {
    ($self_: ident) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - get_e!($self_));
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 1;
    }};
}

macro_rules! sub_a_h {
    ($self_: ident) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - get_h!($self_));
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 1;
    }};
}

macro_rules! sub_a_l {
    ($self_: ident) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - get_l!($self_));
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 1;
    }};
}

macro_rules! sub_a_hl_val {
    ($self_: ident, $mem: ident) => {{
        let old_a = get_a!($self_);
        let hl_val = $mem.get_mem_u8(get_hl!($self_) as usize);
        set_a!($self_, get_a!($self_) - hl_val);
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 2;
    }};
}

macro_rules! sub_a_param {
    ($self_: ident, $param: expr) => {{
        let old_a = get_a!($self_);
        set_a!($self_, get_a!($self_) - $param);
        set_n_flag!($self_);
        if get_a!($self_) < old_a { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }

        if old_a & 0b00010000 == 0b00010000 && get_a!($self_) & 0b00010000 == 0 { unset_h_flag!($self_); }
        else { set_h_flag!($self_); }

        if get_a!($self_) == 0 { set_z_flag!($self_); }
        else { set_z_flag!($self_); }

        return 2;
    }};
}
