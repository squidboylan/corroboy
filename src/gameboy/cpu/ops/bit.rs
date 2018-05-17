// BIT x,n
// Opposite of xth bit of the n register is written to the z flag, n is unset, h is set

macro_rules! bit_0_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_1_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_2_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_3_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_4_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_5_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_6_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_7_a {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_a!($self_) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_0_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_1_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_2_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_3_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_4_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_5_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_6_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_7_b {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_b!($self_) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}


macro_rules! bit_0_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_1_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_2_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_3_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_4_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_5_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_6_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_7_c {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_c!($self_) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_0_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_1_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_2_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_3_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_4_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_5_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_6_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_7_d {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_d!($self_) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_0_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_1_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_2_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_3_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_4_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_5_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_6_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_7_e {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_e!($self_) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_0_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_1_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_2_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_3_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_4_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_5_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_6_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_7_h {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_h!($self_) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_0_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_1_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_2_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_3_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_4_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_5_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_6_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_7_l {
    ($self_: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if get_l!($self_) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        2
    }};
}

macro_rules! bit_0_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b00000001 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

macro_rules! bit_1_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b00000010 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

macro_rules! bit_2_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b00000100 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

macro_rules! bit_3_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b00001000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

macro_rules! bit_4_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b00010000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

macro_rules! bit_5_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b00100000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

macro_rules! bit_6_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b01000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}

macro_rules! bit_7_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        set_h_flag!($self_);
        if $mem.get_mem_u8(get_hl!($self_) as usize) & 0b10000000 == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        4
    }};
}
