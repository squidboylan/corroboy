// JR n

macro_rules! jr_n {
    ($self_: ident, $param: expr) => {{
        set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
    }};
}

// JR cc,n

macro_rules! jr_nz_n {
    ($self_: ident, $param: expr) => {{
        if get_z_flag!($self_) == 0 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
    }};
}

macro_rules! jr_z_n {
    ($self_: ident, $param: expr) => {{
        if get_z_flag!($self_) == 1 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
    }};
}

macro_rules! jr_nc_n {
    ($self_: ident, $param: expr) => {{
        if get_c_flag!($self_) == 0 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
    }};
}

macro_rules! jr_c_n {
    ($self_: ident, $param: expr) => {{
        if get_c_flag!($self_) == 1 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
    }};
}
