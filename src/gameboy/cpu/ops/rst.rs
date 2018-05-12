// RST nn

macro_rules! rst_nn {
    ($self_: ident, $mem: ident, $param: expr) => {{
        $mem.push_u16(get_sp_mut!($self_), get_pc!($self_));
        set_pc!($self_, $param);
        8
    }};
}
