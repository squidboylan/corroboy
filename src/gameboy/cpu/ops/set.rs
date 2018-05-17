// SET n,A

macro_rules! set_0_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b00000001);
        2
    }};
}

macro_rules! set_1_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b00000010);
        2
    }};
}

macro_rules! set_2_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b00000100);
        2
    }};
}

macro_rules! set_3_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b00001000);
        2
    }};
}

macro_rules! set_4_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b00010000);
        2
    }};
}

macro_rules! set_5_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b00100000);
        2
    }};
}

macro_rules! set_6_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b01000000);
        2
    }};
}

macro_rules! set_7_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_) | 0b10000000);
        2
    }};
}

// SET n,B

macro_rules! set_0_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b00000001);
        2
    }};
}

macro_rules! set_1_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b00000010);
        2
    }};
}

macro_rules! set_2_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b00000100);
        2
    }};
}

macro_rules! set_3_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b00001000);
        2
    }};
}

macro_rules! set_4_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b00010000);
        2
    }};
}

macro_rules! set_5_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b00100000);
        2
    }};
}

macro_rules! set_6_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b01000000);
        2
    }};
}

macro_rules! set_7_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_) | 0b10000000);
        2
    }};
}

// SET n,C

macro_rules! set_0_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b00000001);
        2
    }};
}

macro_rules! set_1_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b00000010);
        2
    }};
}

macro_rules! set_2_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b00000100);
        2
    }};
}

macro_rules! set_3_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b00001000);
        2
    }};
}

macro_rules! set_4_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b00010000);
        2
    }};
}

macro_rules! set_5_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b00100000);
        2
    }};
}

macro_rules! set_6_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b01000000);
        2
    }};
}

macro_rules! set_7_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_) | 0b10000000);
        2
    }};
}

// SET n,D

macro_rules! set_0_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b00000001);
        2
    }};
}

macro_rules! set_1_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b00000010);
        2
    }};
}

macro_rules! set_2_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b00000100);
        2
    }};
}

macro_rules! set_3_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b00001000);
        2
    }};
}

macro_rules! set_4_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b00010000);
        2
    }};
}

macro_rules! set_5_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b00100000);
        2
    }};
}

macro_rules! set_6_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b01000000);
        2
    }};
}

macro_rules! set_7_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_) | 0b10000000);
        2
    }};
}


// SET n,E

macro_rules! set_0_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b00000001);
        2
    }};
}

macro_rules! set_1_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b00000010);
        2
    }};
}

macro_rules! set_2_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b00000100);
        2
    }};
}

macro_rules! set_3_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b00001000);
        2
    }};
}

macro_rules! set_4_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b00010000);
        2
    }};
}

macro_rules! set_5_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b00100000);
        2
    }};
}

macro_rules! set_6_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b01000000);
        2
    }};
}

macro_rules! set_7_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_) | 0b10000000);
        2
    }};
}

// SET n,H

macro_rules! set_0_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b00000001);
        2
    }};
}

macro_rules! set_1_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b00000010);
        2
    }};
}

macro_rules! set_2_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b00000100);
        2
    }};
}

macro_rules! set_3_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b00001000);
        2
    }};
}

macro_rules! set_4_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b00010000);
        2
    }};
}

macro_rules! set_5_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b00100000);
        2
    }};
}

macro_rules! set_6_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b01000000);
        2
    }};
}

macro_rules! set_7_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_) | 0b10000000);
        2
    }};
}

// SET n,L

macro_rules! set_0_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b00000001);
        2
    }};
}

macro_rules! set_1_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b00000010);
        2
    }};
}

macro_rules! set_2_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b00000100);
        2
    }};
}

macro_rules! set_3_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b00001000);
        2
    }};
}

macro_rules! set_4_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b00010000);
        2
    }};
}

macro_rules! set_5_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b00100000);
        2
    }};
}

macro_rules! set_6_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b01000000);
        2
    }};
}

macro_rules! set_7_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_) | 0b10000000);
        2
    }};
}

// SET n,(HL)

macro_rules! set_0_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b00000001;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! set_1_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b00000010;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! set_2_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b00000100;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! set_3_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b00001000;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! set_4_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b00010000;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! set_5_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b00100000;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! set_6_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b01000000;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

macro_rules! set_7_hl_val {
    ($self_: ident, $mem: ident) => {{
        let new_val = $mem.get_mem_u8(get_hl!($self_) as usize) | 0b10000000;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        4
    }};
}

