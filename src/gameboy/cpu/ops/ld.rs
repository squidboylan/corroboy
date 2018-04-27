// LD a,r2

macro_rules! ld_a_a {
    ($self_: ident) => {{
        set_a!($self_, get_a!($self_));
    }};
}

macro_rules! ld_a_b {
    ($self_: ident) => {{
        set_a!($self_, get_b!($self_));
    }};
}

macro_rules! ld_a_c {
    ($self_: ident) => {{
        set_a!($self_, get_c!($self_));
    }};
}

macro_rules! ld_a_d {
    ($self_: ident) => {{
        set_a!($self_, get_d!($self_));
    }};
}

macro_rules! ld_a_e {
    ($self_: ident) => {{
        set_a!($self_, get_e!($self_));
    }};
}

macro_rules! ld_a_h {
    ($self_: ident) => {{
        set_a!($self_, get_h!($self_));
    }};
}

macro_rules! ld_a_l {
    ($self_: ident) => {{
        set_a!($self_, get_l!($self_));
    }};
}

macro_rules! ld_a_param {
    ($self_: ident, $param: expr) => {{
        set_a!($self_, $param);
    }};
}

macro_rules! ld_a_bc_val {
    ($self_: ident, $mem: ident) => {{
        set_a!($self_, $mem.get_mem_u8(get_bc!($self_) as usize));
    }};
}

macro_rules! ld_a_de_val {
    ($self_: ident, $mem: ident) => {{
        set_a!($self_, $mem.get_mem_u8(get_de!($self_) as usize));
    }};
}

macro_rules! ld_a_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_a!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
    }};
}

// LD b,r2

macro_rules! ld_b_a {
    ($self_: ident) => {{
        set_b!($self_, get_a!($self_));
    }};
}

macro_rules! ld_b_b {
    ($self_: ident) => {{
        set_b!($self_, get_b!($self_));
    }};
}

macro_rules! ld_b_c {
    ($self_: ident) => {{
        set_b!($self_, get_c!($self_));
    }};
}

macro_rules! ld_b_d {
    ($self_: ident) => {{
        set_b!($self_, get_d!($self_));
    }};
}

macro_rules! ld_b_e {
    ($self_: ident) => {{
        set_b!($self_, get_e!($self_));
    }};
}

macro_rules! ld_b_h {
    ($self_: ident) => {{
        set_b!($self_, get_h!($self_));
    }};
}

macro_rules! ld_b_l {
    ($self_: ident) => {{
        set_b!($self_, get_l!($self_));
    }};
}

macro_rules! ld_b_param {
    ($self_: ident, $param: expr) => {{
        set_b!($self_, $param);
    }};
}

macro_rules! ld_b_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_b!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
    }};
}

// LD c,r2

macro_rules! ld_c_a {
    ($self_: ident) => {{
        set_c!($self_, get_a!($self_));
    }};
}

macro_rules! ld_c_b {
    ($self_: ident) => {{
        set_c!($self_, get_b!($self_));
    }};
}

macro_rules! ld_c_c {
    ($self_: ident) => {{
        set_c!($self_, get_c!($self_));
    }};
}

macro_rules! ld_c_d {
    ($self_: ident) => {{
        set_c!($self_, get_d!($self_));
    }};
}

macro_rules! ld_c_e {
    ($self_: ident) => {{
        set_c!($self_, get_e!($self_));
    }};
}

macro_rules! ld_c_h {
    ($self_: ident) => {{
        set_c!($self_, get_h!($self_));
    }};
}

macro_rules! ld_c_l {
    ($self_: ident) => {{
        set_c!($self_, get_l!($self_));
    }};
}

macro_rules! ld_c_param {
    ($self_: ident, $param: expr) => {{
        set_c!($self_, $param);
    }};
}


macro_rules! ld_c_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_c!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
    }};
}

// LD d,r2

macro_rules! ld_d_a {
    ($self_: ident) => {{
        set_d!($self_, get_a!($self_));
    }};
}

macro_rules! ld_d_b {
    ($self_: ident) => {{
        set_d!($self_, get_b!($self_));
    }};
}

macro_rules! ld_d_c {
    ($self_: ident) => {{
        set_d!($self_, get_c!($self_));
    }};
}

macro_rules! ld_d_d {
    ($self_: ident) => {{
        set_d!($self_, get_d!($self_));
    }};
}

macro_rules! ld_d_e {
    ($self_: ident) => {{
        set_d!($self_, get_e!($self_));
    }};
}

macro_rules! ld_d_h {
    ($self_: ident) => {{
        set_d!($self_, get_h!($self_));
    }};
}

macro_rules! ld_d_l {
    ($self_: ident) => {{
        set_d!($self_, get_l!($self_));
    }};
}

macro_rules! ld_d_param {
    ($self_: ident, $param: expr) => {{
        set_d!($self_, $param);
    }};
}

macro_rules! ld_d_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_d!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
    }};
}

// LD e,r2

macro_rules! ld_e_a {
    ($self_: ident) => {{
        set_e!($self_, get_a!($self_));
    }};
}

macro_rules! ld_e_b {
    ($self_: ident) => {{
        set_e!($self_, get_b!($self_));
    }};
}

macro_rules! ld_e_c {
    ($self_: ident) => {{
        set_e!($self_, get_c!($self_));
    }};
}

macro_rules! ld_e_d {
    ($self_: ident) => {{
        set_e!($self_, get_d!($self_));
    }};
}

macro_rules! ld_e_e {
    ($self_: ident) => {{
        set_e!($self_, get_e!($self_));
    }};
}

macro_rules! ld_e_h {
    ($self_: ident) => {{
        set_e!($self_, get_h!($self_));
    }};
}

macro_rules! ld_e_l {
    ($self_: ident) => {{
        set_e!($self_, get_l!($self_));
    }};
}

macro_rules! ld_e_param {
    ($self_: ident, $param: expr) => {{
        set_e!($self_, $param);
    }};
}


macro_rules! ld_e_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_e!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
    }};
}

// LD h,r2

macro_rules! ld_h_a {
    ($self_: ident) => {{
        set_h!($self_, get_a!($self_));
    }};
}

macro_rules! ld_h_b {
    ($self_: ident) => {{
        set_h!($self_, get_b!($self_));
    }};
}

macro_rules! ld_h_c {
    ($self_: ident) => {{
        set_h!($self_, get_c!($self_));
    }};
}

macro_rules! ld_h_d {
    ($self_: ident) => {{
        set_h!($self_, get_d!($self_));
    }};
}

macro_rules! ld_h_e {
    ($self_: ident) => {{
        set_h!($self_, get_e!($self_));
    }};
}

macro_rules! ld_h_h {
    ($self_: ident) => {{
        set_h!($self_, get_h!($self_));
    }};
}

macro_rules! ld_h_l {
    ($self_: ident) => {{
        set_h!($self_, get_l!($self_));
    }};
}

macro_rules! ld_h_param {
    ($self_: ident, $param: expr) => {{
        set_h!($self_, $param);
    }};
}

macro_rules! ld_h_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_h!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
    }};
}

// LD l,r2

macro_rules! ld_l_a {
    ($self_: ident) => {{
        set_l!($self_, get_a!($self_));
    }};
}

macro_rules! ld_l_b {
    ($self_: ident) => {{
        set_l!($self_, get_b!($self_));
    }};
}

macro_rules! ld_l_c {
    ($self_: ident) => {{
        set_l!($self_, get_c!($self_));
    }};
}

macro_rules! ld_l_d {
    ($self_: ident) => {{
        set_l!($self_, get_d!($self_));
    }};
}

macro_rules! ld_l_e {
    ($self_: ident) => {{
        set_l!($self_, get_e!($self_));
    }};
}

macro_rules! ld_l_h {
    ($self_: ident) => {{
        set_l!($self_, get_h!($self_));
    }};
}

macro_rules! ld_l_l {
    ($self_: ident) => {{
        set_l!($self_, get_l!($self_));
    }};
}

macro_rules! ld_l_param {
    ($self_: ident, $param: expr) => {{
        set_l!($self_, $param);
    }};
}

macro_rules! ld_l_hl_val {
    ($self_: ident, $mem: ident) => {{
        set_l!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
    }};
}

// LD BC, nn

macro_rules! ld_bc_param {
    ($self_: ident, $param: expr) => {{
        set_bc!($self_, $param);
    }};
}

macro_rules! ld_de_param {
    ($self_: ident, $param: expr) => {{
        set_de!($self_, $param);
    }};
}

macro_rules! ld_hl_param {
    ($self_: ident, $param: expr) => {{
        set_hl!($self_, $param);
    }};
}

macro_rules! ld_sp_param {
    ($self_: ident, $param: expr) => {{
        set_sp!($self_, $param);
    }};
}

// LD (hl),r2

macro_rules! ld_hl_val_a {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_a!($self_));
    }};
}

macro_rules! ld_hl_val_b {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_a!($self_));
    }};
}

macro_rules! ld_hl_val_c {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_c!($self_));
    }};
}

macro_rules! ld_hl_val_d {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_d!($self_));
    }};
}

macro_rules! ld_hl_val_e {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_e!($self_));
    }};
}

macro_rules! ld_hl_val_h {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_h!($self_));
    }};
}

macro_rules! ld_hl_val_l {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_l!($self_));
    }};
}

macro_rules! ld_hl_val_n {
    ($self_: ident, $mem: ident, $param: expr) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, $param);
    }};
}

// LD (bc),a

macro_rules! ld_bc_val_a {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_bc!($self_) as usize, get_a!($self_))
    }};
}

// LD (de),a

macro_rules! ld_de_val_a {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_de!($self_) as usize, get_a!($self_))
    }};
}

// LD (nn),a

macro_rules! ld_nn_val_a {
    ($self_: ident, $mem: ident, $param: expr) => {{
        $mem.set_mem_u8($param as usize, get_a!($self_))
    }};
}

// LD sp, hl

macro_rules! ld_sp_hl {
    ($self_: ident) => {{
        set_sp!($self_, get_hl!($self_));
    }};
}

// LD (nn), sp

macro_rules! ld_param_val_sp {
    ($self_: ident, $mem: ident, $param: expr) => {{
        $mem.set_mem_u16($param as usize, get_sp!($self_));
    }};
}

// LD (C), a

macro_rules! ld_c_val_a {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8((get_c!($self_) as u16 + 0xFF00) as usize, get_a!($self_));
    }};
}

// LD a, (c)

macro_rules! ld_a_c_val {
    ($self_: ident, $mem: ident) => {{
        set_a!($self_, $mem.get_mem_u8((get_c!($self_) as u16 + 0xFF00) as usize));
    }};
}

// LD (n), a
// load A into (n + 0xFF00)

macro_rules! ld_n_val_a {
    ($self_: ident, $mem: ident, $param: expr) => {{
        $mem.set_mem_u8(($param as u16 + 0xFF00) as usize, get_a!($self_));
    }};
}

// LD a, (n)
// load (n + 0xFF00) into A

macro_rules! ld_a_n_val {
    ($self_: ident, $mem: ident, $param: expr) => {{
        set_a!($self_, $mem.get_mem_u8(($param as u16 + 0xFF00) as usize));
    }};
}

// LD a, (nn)

macro_rules! ld_a_nn_val {
    ($self_: ident, $mem: ident, $param: expr) => {{
        set_a!($self_, $mem.get_mem_u8($param.swap_bytes() as usize));
    }};
}

// LDI A, (HL)
// Load (HL) into A and then increment HL

macro_rules! ldi_a_hl {
    ($self_: ident, $mem: ident) => {{
        set_a!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
        set_hl!($self_, get_hl!($self_) + 1);
    }};
}

// LDI (HL), A
// Load A into (HL) and then increment HL

macro_rules! ldi_hl_a {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_a!($self_));
        set_hl!($self_, get_hl!($self_) + 1);
    }};
}

// LDD A, (HL)
// Load (HL) into A and then decrement HL

macro_rules! ldd_a_hl {
    ($self_: ident, $mem: ident) => {{
        set_a!($self_, $mem.get_mem_u8(get_hl!($self_) as usize));
        set_hl!($self_, get_hl!($self_) - 1);
    }};
}

// LDD (HL), A
// Load A into (HL) and then decrement HL

macro_rules! ldd_hl_a {
    ($self_: ident, $mem: ident) => {{
        $mem.set_mem_u8(get_hl!($self_) as usize, get_a!($self_));
        set_hl!($self_, get_hl!($self_) - 1);
    }};
}

// LDHL SP,n

macro_rules! ldhl_sp_n {
    ($self_: ident, $param: expr) => {{
        set_hl!($self_, (get_sp!($self_) as i16 + (($param as i8) as i16)) as u16);
        unset_z_flag!($self_);
        unset_n_flag!($self_);
        // This is fine because we care about the last 4 bits and not the sign? Docs are hard to
        // find on this
        if get_sp!($self_) & 0b0000000000001111 + ($param as u16) & 0b0000000000001111 > 0xF { set_h_flag!($self_); }
        else { set_h_flag!($self_); }
        if get_hl!($self_) < get_sp!($self_) { set_c_flag!($self_); }
        else { unset_c_flag!($self_); }
    }};
}
