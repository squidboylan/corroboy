// SWAP n

pub fn swap(reg: &mut u8, flags: &mut u8) {
    *flags &= 0;
    *reg = (*reg >> 4) + (*reg << 4);
    if *reg == 0 { *flags |= 0b10000000 }
}

macro_rules! swap_hl_val {
    ($self_: ident, $mem: ident) => {{
        let old_val = $mem.get_mem_u8(get_hl!($self_) as usize);
        let new_val = (old_val >> 4) + (old_val << 4);
        $mem.set_mem_u8(get_hl!($self_) as usize, new_val);
        if new_val == 0 { set_z_flag!($self_); }
        else { unset_z_flag!($self_); }
        unset_n_flag!($self_);
        unset_h_flag!($self_);
        unset_c_flag!($self_);
        4
    }};
}
