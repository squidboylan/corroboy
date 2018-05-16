// PUSH nn

macro_rules! push_af {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_af!($self_));
        4
    }};
}

macro_rules! push_bc {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_bc!($self_));
        4
    }};
}

macro_rules! push_de {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_de!($self_));
        4
    }};
}

macro_rules! push_hl {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_hl!($self_));
        4
    }};
}
