// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// BIT x,n
// Opposite of xth bit of the n register is written to the z flag, n is unset, h is set

#[inline(always)]
pub fn bit(val: u8, bit: u8, flags: &mut u8) {
    *flags &= 0b00110000;
    *flags |= 0b00100000;
    if val & (1 << bit) == 0 {
        *flags |= 0b10000000;
    }
}
