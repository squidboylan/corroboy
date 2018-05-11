// BIT x,n
// Opposite of xth bit of the n register is written to the z flag, n is unset, h is set

macro_rules! bit_7_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if ((get_h!($self_) & 0b10000000) >> 7) == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}
