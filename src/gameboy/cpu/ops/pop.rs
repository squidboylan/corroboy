// POP nn

macro_rules! pop_af {
    ($self_: ident, $mem: ident) => {{
        set_af!($self_, $mem.pop_u16(get_sp_mut!($self_)));
        3
    }};
}

macro_rules! pop_bc {
    ($self_: ident, $mem: ident) => {{
        set_bc!($self_, $mem.pop_u16(get_sp_mut!($self_)));
        3
    }};
}

macro_rules! pop_de {
    ($self_: ident, $mem: ident) => {{
        set_de!($self_, $mem.pop_u16(get_sp_mut!($self_)));
        3
    }};
}

macro_rules! pop_hl {
    ($self_: ident, $mem: ident) => {{
        set_hl!($self_, $mem.pop_u16(get_sp_mut!($self_)));
        3
    }};
}

// RET

macro_rules! pop_pc {
    ($self_: ident, $mem: ident) => {{
        set_pc!($self_, $mem.pop_u16(get_sp_mut!($self_)));
        2
    }};
}
