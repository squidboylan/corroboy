// INC n

#[inline(always)]
pub fn inc(reg: &mut u8, flags: &mut u8) {
    *flags &= 0b10110000;
    *reg += 1;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01110000; }
    if *reg & 0b00011111 == 0b00010000 { *flags |= 0b00100000; }
    else { *flags &= 0b11010000; }
}

macro_rules! inc_hl_val {
    ($self_: ident, $mem: ident) => {{
        unset_n_flag!($self_);
        let old_val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = old_val + 1;
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if new_val & 0b00011111 == 0b00010000 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        3
    }};
}


// INC nn

pub fn inc_16bit(reg: &mut u16) {
    *reg += 1;
}
