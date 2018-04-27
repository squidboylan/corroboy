// PUSH nn

macro_rules! push_af {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_af!($self_))
    }};
}

macro_rules! push_bc {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_bc!($self_))
    }};
}

macro_rules! push_de {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_de!($self_))
    }};
}

macro_rules! push_hl {
    ($self_: ident, $mem: ident) => {{
        $mem.push_u16(get_sp_mut!($self_), get_hl!($self_))
    }};
}
