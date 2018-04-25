// POP nn

macro_rules! pop_af {
    ($self_: ident) => {{
        set_af!($self_, $self_.mem.pop_u16(get_sp_mut!($self_)));
    }};
}

macro_rules! pop_bc {
    ($self_: ident) => {{
        set_bc!($self_, $self_.mem.pop_u16(get_sp_mut!($self_)));
    }};
}

macro_rules! pop_de {
    ($self_: ident) => {{
        set_de!($self_, $self_.mem.pop_u16(get_sp_mut!($self_)));
    }};
}

macro_rules! pop_hl {
    ($self_: ident) => {{
        set_hl!($self_, $self_.mem.pop_u16(get_sp_mut!($self_)));
    }};
}

// RET

macro_rules! pop_pc {
    ($self_: ident) => {{
        set_pc!($self_, $self_.mem.pop_u16(get_sp_mut!($self_)));
    }};
}
