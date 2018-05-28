// DEC n

#[inline(always)]
pub fn dec(reg: &mut u8, flags: &mut u8) {
    *reg -= 1;
    *flags |= 0b01000000;
    if *reg == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01110000; }
    if *reg & 0b00001111 == 0b00001111 { *flags |= 0b00100000; }
    else { *flags &= 0b11010000; }
}

macro_rules! dec_hl_val {
    ($self_: ident, $mem: ident) => {{ let val = $mem.get_mem_u8(get_hl!($self_) as usize) - 1;
        $mem.set_mem_u8(get_hl!($self_) as usize, val);
        set_n_flag!($self_);
        if val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        if (val & 0b00001111) == 0b00001111 { set_h_flag!($self_); }
        else { unset_h_flag!($self_); }
        3
    }};
}

// DEC nn

pub fn dec_16bit(reg: &mut u16) {
    *reg -= 1;
}
