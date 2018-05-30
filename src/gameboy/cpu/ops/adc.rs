// ADC A,n

// TODO FIX THIS
pub fn adc(val: u8, a: &mut u8, flags: &mut u8) {
    let old_a = *a;
    *a += val + ((*flags & 0b00010000) >> 4);
    *flags &= 0b10111111;
    if *a == 0 { *flags |= 0b10000000; }
    else { *flags &= 0b01111111; }
    if old_a > *a { *flags |= 0b00010000; }
    else { *flags &= 0b11101111; }
    if (*a + ((*flags & 0b00010000) >> 4)) & 0b00001111 + old_a & 0b00001111 >= 0b00010000 { *flags &= 0b11011111; }
    else { *flags |= 0b00100000; }
}
