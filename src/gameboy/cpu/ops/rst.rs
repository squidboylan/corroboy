// RST nn

macro_rules! rst_nn {
    ($self_: ident, $param: expr) => {{
        set_pc!($self_, $param);
        return 8;
    }};
}
