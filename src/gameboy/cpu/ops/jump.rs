// JP nn

macro_rules! jp_nn {
    ($self_: ident, $param: expr) => {{
        set_pc!($self_, $param);
        3
    }};
}

// JP (HL)

macro_rules! jp_hl {
    ($self_: ident, $param: expr) => {{
        set_pc!($self_, get_hl!($self_));
        3
    }};
}

// JP cc,nn

macro_rules! jp_nz_nn {
    ($self_: ident, $param: expr) => {{
        if get_z_flag!($self_) == 0 {
            set_pc!($self_, $param);
        }
        3
    }};
}

macro_rules! jp_z_nn {
    ($self_: ident, $param: expr) => {{
        if get_z_flag!($self_) == 1 {
            set_pc!($self_, $param);
        }
        3
    }};
}

macro_rules! jp_nc_nn {
    ($self_: ident, $param: expr) => {{
        if get_c_flag!($self_) == 0 {
            set_pc!($self_, $param);
        }
        3
    }};
}

macro_rules! jp_c_nn {
    ($self_: ident, $param: expr) => {{
        if get_c_flag!($self_) == 1 {
            set_pc!($self_, $param);
        }
        3
    }};
}

// JR n

macro_rules! jr_n {
    ($self_: ident, $param: expr) => {{
        set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        2
    }};
}

// JR cc,n

macro_rules! jr_nz_n {
    ($self_: ident, $param: expr) => {{
        if get_z_flag!($self_) == 0 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
        2
    }};
}

macro_rules! jr_z_n {
    ($self_: ident, $param: expr) => {{
        if get_z_flag!($self_) == 1 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
        2
    }};
}

macro_rules! jr_nc_n {
    ($self_: ident, $param: expr) => {{
        if get_c_flag!($self_) == 0 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
        2
    }};
}

macro_rules! jr_c_n {
    ($self_: ident, $param: expr) => {{
        if get_c_flag!($self_) == 1 {
            set_pc!($self_, (get_pc!($self_) as i16 + (($param as i8) as i16)) as u16);
        }
        2
    }};
}

// CALL nn

macro_rules! call_nn {
    ($self_: ident, $mem: ident, $param: expr) => {{
        $mem.push_u16(get_sp_mut!($self_), get_pc!($self_));
        set_pc!($self_, $param);
        3
    }};
}

// CALL cc,nn

macro_rules! call_nz_nn {
    ($self_: ident, $mem: ident, $param: expr) => {{
        if get_z_flag!($self_) == 0 {
            $mem.push_u16(get_sp_mut!($self_), get_pc!($self_));
            set_pc!($self_, $param);
        }
        3
    }};
}

macro_rules! call_z_nn {
    ($self_: ident, $mem: ident, $param: expr) => {{
        if get_z_flag!($self_) == 1 {
            $mem.push_u16(get_sp_mut!($self_), get_pc!($self_));
            set_pc!($self_, $param);
        }
        3
    }};
}

macro_rules! call_nc_nn {
    ($self_: ident, $mem: ident, $param: expr) => {{
        if get_c_flag!($self_) == 0 {
            $mem.push_u16(get_sp_mut!($self_), get_pc!($self_));
            set_pc!($self_, $param);
        }
        3
    }};
}

macro_rules! call_c_nn {
    ($self_: ident, $mem: ident, $param: expr) => {{
        if get_c_flag!($self_) == 1 {
            $mem.push_u16(get_sp_mut!($self_), get_pc!($self_));
            set_pc!($self_, $param);
        }
        3
    }};
}

