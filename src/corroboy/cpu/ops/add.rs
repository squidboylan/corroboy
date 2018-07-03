// ADD HL,n

pub fn add_16bit(val: u16, reg: &mut u16, flags: &mut u8) {
    let old_reg = *reg;
    *reg += val;
    *flags &= 0b10111111;
    if old_reg > *reg {
        *flags |= 0b00010000;
    } else {
        *flags &= 0b11101111;
    }
    if (old_reg & 0b0000111111111111) + (val & 0b0000111111111111) >= 0b0001000000000000 {
        *flags |= 0b00100000;
    } else {
        *flags &= 0b11011111;
    }
}

// ADD A, n

pub fn add(val: u8, reg: &mut u8, flags: &mut u8) {
    let old_reg = *reg;
    *reg += val;
    *flags &= 0b10111111;
    if old_reg > *reg {
        *flags |= 0b00010000;
    } else {
        *flags &= 0b11101111;
    }
    if (old_reg & 0b00001111) + (val & 0b00001111) >= 0b00010000 {
        *flags |= 0b00100000;
    } else {
        *flags &= 0b11011111;
    }
    if *reg != 0 {
        *flags &= 0b01111111;
    } else {
        *flags |= 0b10000000;
    }
}

// ADD sp,n

pub fn add_sp_param(val: u8, sp: &mut u16, flags: &mut u8) {
    let old_sp = *sp;
    let val_i16 = val as i8 as i16;
    *sp = (*sp as i16 + val_i16) as u16;

    *flags = 0;

    if (val_i16 as u16 ^ old_sp ^ *sp) & 0x10 != 0 {
        *flags |= 0b00100000;
    }
    if (val_i16 as u16 ^ old_sp ^ *sp) & 0x100 != 0 {
        *flags |= 0b00010000;
    }
}
