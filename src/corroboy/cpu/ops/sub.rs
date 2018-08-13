// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// SUB n

pub fn sub(val: u8, reg: &mut u8, flags: &mut u8) {
    let old_reg = *reg;
    *reg -= val;
    *flags |= 0b01000000;
    if *reg > old_reg {
        *flags |= 0b00010000;
    } else {
        *flags &= 0b11100000;
    }

    if old_reg & 0b00001111 >= val & 0b00001111 {
        *flags &= 0b11011111;
    } else {
        *flags |= 0b00100000;
    }

    if *reg == 0 {
        *flags |= 0b10000000;
    } else {
        *flags &= 0b01111111;
    }
}
