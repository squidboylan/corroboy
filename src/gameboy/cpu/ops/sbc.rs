// SBC A,n

// TODO FIX THIS
pub fn sbc(val: u8, a: &mut u8, flags: &mut u8) {
    let old_a = *a;
    let val_c = val + ((*flags & 0b00010000) >> 4);
    *a -= val_c;

    *flags |= 0b01000000;

    if *a == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }

    if val < old_a { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }

    /*
    if (old_a ^ (!val_c + 1) ^ *a) & 0x10 == 0x10 { *flags &= 0b11011111; }
    else { *flags |= 0b00100000; }
    */
    if old_a & 0x0F < val_c & 0x0F { *flags &= 0b11011111; }
    else { *flags |= 0b00100000; }
}
