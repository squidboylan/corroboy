// SBC A,n

pub fn sbc(val: u8, a: &mut u8, flags: &mut u8) {
    let old_a = *a;
    let c = (*flags & 0b00010000) >> 4;
    *a = *a - val - c;

    *flags = 0b01000000;

    if *a == 0 { *flags |= 0b10000000; }

    if val == 0xFF && c == 1 { *flags |= 0b00010000; }
    else if old_a < *a { *flags |= 0b00010000; }

    if val & 0x0F == 0x0F && c == 1 { *flags |= 0b00100000; }
    else if old_a & 0x0F < *a & 0x0F { *flags |= 0b00100000; }
    //else if (old_a & 0x0F) - (val & 0x0F) - c > 0x0F { *flags |= 0b00100000; }
}
