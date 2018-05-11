// BIT x,n
// Opposite of xth bit of the n register is written to the z flag, n is unset, h is set

macro_rules! res_7_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b01111111);
        2
    }};
}
