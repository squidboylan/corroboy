// Copyright (c) 2018 Caleb Boylan

// XOR n

#[inline(always)]
pub fn xor(reg: u8, a: &mut u8, flags: &mut u8) {
    *a = *a ^ reg;
    if *a == 0 {
        *flags = *flags | 0b10000000;
    } else {
        *flags = *flags & 0b01111111;
    }
    *flags = *flags & 0b10001111;
}
