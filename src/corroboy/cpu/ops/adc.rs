// ADC A,n

pub fn adc(val: u8, a: &mut u8, flags: &mut u8) {
    let old_a = *a;
    let c = (*flags & 0b00010000) >> 4;
    let val_c = val + c;
    *a += val_c;

    *flags = 0;

    if *a == 0 {
        *flags |= 0b10000000;
    }

    if val == 0xFF && c == 1 {
        *flags |= 0b00010000;
    } else if old_a > *a {
        *flags |= 0b00010000;
    }

    if val & 0x0F == 0x0F && c == 1 {
        *flags |= 0b00100000;
    } else if ((old_a & 0x0F) + (val_c & 0x0F)) & 0xF0 != 0 {
        *flags |= 0b00100000;
    }
}
