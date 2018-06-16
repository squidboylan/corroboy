// ADD HL,n

pub fn add_16bit(val: u16, reg: &mut u16, flags: &mut u8) {
    let old_reg = *reg;
    *reg += val;
    *flags &= 0b10111111;
    if old_reg > *reg { *flags |= 0b00010000; }
    else{ *flags &= 0b11101111; }
    if (old_reg & 0b0000111111111111) + (val & 0b0000111111111111) >= 0b0001000000000000 { *flags |= 0b00100000; }
    else { *flags &= 0b11011111; }
}

// ADD A, n

pub fn add(val: u8, reg: &mut u8, flags: &mut u8) {
    let old_reg = *reg;
    *reg += val;
    *flags &= 0b10111111;
    if old_reg > *reg { *flags |= 0b00010000; }
    else{ *flags &= 0b11101111; }
    if (old_reg & 0b00001111) + (val & 0b00001111) >= 0b00010000 { *flags |= 0b00100000; }
    else { *flags &= 0b11011111; }
    if *reg != 0 { *flags &= 0b01111111; }
    else { *flags |= 0b10000000; }
}

// TODO check this and make sure it is actually correct
pub fn add_sp_param(val: u8, sp: &mut u16, flags: &mut u8) {
    let old_sp = *sp;
    *sp = (*sp as i16 + ((val as i8) as i16)) as u16;
    *flags &= 0b00111111;
    if old_sp > *sp { *flags |= 0b00010000; }
    else{ *flags &= 0b11101111; }
    if (old_sp & 0b0000111111111111) as i16 + (((val as i8) as i16) & 0b0000111111111111) >= 0b0001000000000000 { *flags |= 0b00100000; }
    else { *flags &= 0b11011111; }
}
