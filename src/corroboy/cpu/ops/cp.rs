// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// CP n

#[inline(always)]
pub fn cp(val: u8, a: u8, flags: &mut u8) {
    let tmp = a - val;
    if tmp == 0 {
        *flags |= 0b10000000;
    } else {
        *flags &= 0b01111111;
    }
    // Set n flag
    *flags = *flags | 0b01000000;
    if a < val {
        *flags |= 0b00010000;
    } else {
        *flags &= 0b11101111;
    }
    if (a & 0b00001111) >= (val & 0b00001111) {
        *flags &= 0b11011111;
    } else {
        *flags |= 0b00100000;
    }
}
