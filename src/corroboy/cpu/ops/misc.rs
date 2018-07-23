// Copyright (c) 2018 Caleb Boylan

// DAA
// Decimal adjust register A

pub fn daa(a: &mut u8, flags: &mut u8) {
    let mut corr = 0;
    if *flags & 0b00100000 != 0 {
        corr |= 0x06;
    }
    if *flags & 0b00010000 != 0 {
        corr |= 0x60;
    }
    if *flags & 0b01000000 == 0 {
        if *a & 0b00001111 > 9 {
            corr |= 0x06;
        }
        if *a > 0x99 {
            corr |= 0x60;
            *flags |= 0b00010000;
        }

        *a += corr;
    } else {
        *a -= corr;
    }

    *flags &= 0b01011111;

    if *a == 0 {
        *flags |= 0b10000000;
    }
}

// CPL
// compliment A register

pub fn cpl(a: &mut u8, flags: &mut u8) {
    *a = !*a;
    *flags |= 0b01100000;
}

// CCF
// compliment carry flag

pub fn ccf(flags: &mut u8) {
    if *flags & 0b00010000 != 0 {
        *flags &= 0b11101111;
    } else {
        *flags |= 0b00010000;
    }
    //if *flags & 0b00100000 != 0 { *flags &= 0b11011111; }
    //else { *flags |= 0b00100000; }
    *flags &= 0b10010000;
}

// SCF
// set carry flag

pub fn scf(flags: &mut u8) {
    *flags &= 0b10010000;
    *flags |= 0b00010000;
}

// HALT - power down cpu until an interrupt occurs.

pub fn halt(halt: &mut u8) {
    *halt = 1;
}

// This should halt the GPU and CPU but currently only halts the CPU

pub fn stop(halt: &mut u8) {
    *halt = 1;
}

// Disable and enable interrupts

pub fn di(ime: &mut u8) {
    *ime = 0;
}

pub fn ei(ime: &mut u8) {
    *ime = 1;
}
