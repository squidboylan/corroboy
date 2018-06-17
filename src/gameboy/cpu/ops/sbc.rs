// SBC A,n

// TODO FIX THIS
pub fn sbc(val: u8, a: &mut u8, flags: &mut u8) {
    let old_a = *a;
    let c = (*flags & 0b00010000) >> 4;
    *a = *a - val - c;

    *flags |= 0b01000000;

    if *a == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }

    if old_a < *a { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }

    if (old_a & 0x0F) - (val & 0x0F) - c <= 0x0F { *flags &= 0b11011111; }
    else { *flags |= 0b00100000; }
}
