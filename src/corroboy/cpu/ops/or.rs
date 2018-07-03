// OR n

#[inline(always)]
pub fn or(reg: u8, a: &mut u8, flags: &mut u8) {
    *a = *a | reg;
    if *a == 0 {
        *flags = *flags | 0b10000000;
    } else {
        *flags = *flags & 0b01111111;
    }
    *flags = *flags & 0b10001111;
}
