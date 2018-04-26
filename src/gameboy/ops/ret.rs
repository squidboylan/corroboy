// RET

macro_rules! ret {
    ($self_: ident) => {{
        set_pc!($self_, $self_.mem.pop_u16(get_sp_mut!($self_)));
    }};
}

// RET nn
macro_rules! ret_nz {
    ($self_: ident) => {{
        if get_z_flag!($self_) == 0 {
            ret!($self_);
        }
    }};
}

macro_rules! ret_z {
    ($self_: ident) => {{
        if get_z_flag!($self_) == 1 {
            ret!($self_);
        }
    }};
}

macro_rules! ret_nc {
    ($self_: ident) => {{
        if get_c_flag!($self_) == 0 {
            ret!($self_);
        }
    }};
}

macro_rules! ret_c {
    ($self_: ident) => {{
        if get_c_flag!($self_) == 1 {
            ret!($self_);
        }
    }};
}

