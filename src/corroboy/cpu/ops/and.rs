// AND n

#[inline(always)]
pub fn and(val: u8, a: &mut u8, flags: &mut u8) {
    *a = val & *a;
    *flags = 0b00000000;
    if *a == 0 { *flags = *flags | 0b10000000; }
    else { *flags = *flags & 0b01111111; }
    *flags = *flags | 0b00100000;
}
