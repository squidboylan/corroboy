// RES n,A

macro_rules! res_0_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b11111110);
        2
    }};
}

macro_rules! res_1_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b11111101);
        2
    }};
}

macro_rules! res_2_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b11111011);
        2
    }};
}

macro_rules! res_3_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b11110111);
        2
    }};
}

macro_rules! res_4_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b11101111);
        2
    }};
}

macro_rules! res_5_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b11011111);
        2
    }};
}

macro_rules! res_6_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b10111111);
        2
    }};
}

macro_rules! res_7_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) & 0b01111111);
        2
    }};
}

// RES n,B

macro_rules! res_0_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b11111110);
        2
    }};
}

macro_rules! res_1_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b11111101);
        2
    }};
}

macro_rules! res_2_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b11111011);
        2
    }};
}

macro_rules! res_3_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b11110111);
        2
    }};
}

macro_rules! res_4_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b11101111);
        2
    }};
}

macro_rules! res_5_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b11011111);
        2
    }};
}

macro_rules! res_6_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b10111111);
        2
    }};
}

macro_rules! res_7_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) & 0b01111111);
        2
    }};
}

// RES n,C

macro_rules! res_0_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b11111110);
        2
    }};
}

macro_rules! res_1_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b11111101);
        2
    }};
}

macro_rules! res_2_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b11111011);
        2
    }};
}

macro_rules! res_3_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b11110111);
        2
    }};
}

macro_rules! res_4_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b11101111);
        2
    }};
}

macro_rules! res_5_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b11011111);
        2
    }};
}

macro_rules! res_6_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b10111111);
        2
    }};
}

macro_rules! res_7_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) & 0b01111111);
        2
    }};
}

// RES n,D

macro_rules! res_0_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b11111110);
        2
    }};
}

macro_rules! res_1_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b11111101);
        2
    }};
}

macro_rules! res_2_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b11111011);
        2
    }};
}

macro_rules! res_3_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b11110111);
        2
    }};
}

macro_rules! res_4_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b11101111);
        2
    }};
}

macro_rules! res_5_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b11011111);
        2
    }};
}

macro_rules! res_6_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b10111111);
        2
    }};
}

macro_rules! res_7_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) & 0b01111111);
        2
    }};
}


// RES n,E

macro_rules! res_0_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b11111110);
        2
    }};
}

macro_rules! res_1_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b11111101);
        2
    }};
}

macro_rules! res_2_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b11111011);
        2
    }};
}

macro_rules! res_3_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b11110111);
        2
    }};
}

macro_rules! res_4_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b11101111);
        2
    }};
}

macro_rules! res_5_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b11011111);
        2
    }};
}

macro_rules! res_6_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b10111111);
        2
    }};
}

macro_rules! res_7_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) & 0b01111111);
        2
    }};
}

// RES n,H

macro_rules! res_0_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b11111110);
        2
    }};
}

macro_rules! res_1_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b11111101);
        2
    }};
}

macro_rules! res_2_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b11111011);
        2
    }};
}

macro_rules! res_3_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b11110111);
        2
    }};
}

macro_rules! res_4_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b11101111);
        2
    }};
}

macro_rules! res_5_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b11011111);
        2
    }};
}

macro_rules! res_6_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b10111111);
        2
    }};
}

macro_rules! res_7_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) & 0b01111111);
        2
    }};
}

// RES n,L

macro_rules! res_0_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b11111110);
        2
    }};
}

macro_rules! res_1_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b11111101);
        2
    }};
}

macro_rules! res_2_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b11111011);
        2
    }};
}

macro_rules! res_3_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b11110111);
        2
    }};
}

macro_rules! res_4_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b11101111);
        2
    }};
}

macro_rules! res_5_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b11011111);
        2
    }};
}

macro_rules! res_6_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b10111111);
        2
    }};
}

macro_rules! res_7_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) & 0b01111111);
        2
    }};
}

// RES n,HL

macro_rules! res_0_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b11111110;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! res_1_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b11111101;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! res_2_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b11111011;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! res_3_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b11110111;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! res_4_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b11101111;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! res_5_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b11011111;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! res_6_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b10111111;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! res_7_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) & 0b01111111;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}
