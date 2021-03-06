// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::corroboy::mmu::Mmu;

/// Disassemble the opcode at the location passed in as an argument
pub fn display_disassembly(mem: &Mmu, loc: usize) {
    let mut opcode: u16 = mem.get_mem_u8(loc) as u16;

    // Get 2 byte opcode if it starts with 0xCB
    if opcode == 0xCB {
        opcode = ((opcode) << 8) + mem.get_mem_u8(loc + 1) as u16;
    }
    print!("0x{:04x}: ", loc);
    match opcode {
        0x00 => println!("nop"),
        0x01 => println!("ld    BC,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0x08 => println!("ld    (0x{:04x}),SP", mem.get_mem_u16(loc + 1)),
        0x11 => println!("ld    DE,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0x21 => println!("ld    HL,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0x31 => println!("ld    SP,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xFA => println!(
            "ld    A,(0x{:04x})    ; val = 0x{:04x}",
            mem.get_mem_u16(loc + 1),
            mem.get_mem_u8(mem.get_mem_u16(loc + 1) as usize)
        ),
        0xCD => println!("call    0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xC4 => println!("call    NZ,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xCC => println!("call    Z,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xD4 => println!("call    NC,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xDC => println!("call    C,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xC3 => println!("jp    0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xC2 => println!("jp    NZ,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xCA => println!("jp    Z,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xD2 => println!("jp    NC,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xDA => println!("jp    C,0x{:04x}", mem.get_mem_u16(loc + 1)),
        0xEA => println!("ld    (0x{:04x}),A", mem.get_mem_u16(loc + 1)),

        0x06 => println!("ld    B,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0x0E => println!("ld    C,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0x16 => println!("ld    D,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0x1E => println!("ld    E,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0x26 => println!("ld    H,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0x2E => println!("ld    L,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0x3E => println!("ld    A,0x{:02x}", mem.get_mem_u8(loc + 1)),

        0x18 => println!("jr    0x{:02x}", mem.get_mem_u8(loc + 1) as i8),
        0x20 => println!("jr    NZ,0x{:02x}", mem.get_mem_u8(loc + 1) as i8),
        0x28 => println!("jr    Z,0x{:02x}", mem.get_mem_u8(loc + 1) as i8),
        0x30 => println!("jr    NC,0x{:02x}", mem.get_mem_u8(loc + 1) as i8),
        0x38 => println!("jr    C,0x{:02x}", mem.get_mem_u8(loc + 1) as i8),

        0x36 => println!("ld    (HL),0x{:02x}", mem.get_mem_u8(loc + 1)),
        0xC6 => println!("add    A,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0xE0 => println!("ld    (0xFF00 + 0x{:02x}),A", mem.get_mem_u8(loc + 1)),
        0xE6 => println!("and    0x{:02x}", mem.get_mem_u8(loc + 1)),
        0xEE => println!("xor    0x{:02x}", mem.get_mem_u8(loc + 1)),
        0xF0 => println!("ld    A,(0xFF00 + 0x{:02x})", mem.get_mem_u8(loc + 1)),
        0xF6 => println!("or    0x{:02x}", mem.get_mem_u8(loc + 1)),
        0xFE => println!("cp    0x{:02x}", mem.get_mem_u8(loc + 1)),

        0xE8 => println!("add    SP,0x{:02x}", mem.get_mem_u8(loc + 1)),
        0x10 => println!("stop"),

        0x3C => println!("inc   A"),
        0x04 => println!("inc   B"),
        0x0C => println!("inc   C"),
        0x14 => println!("inc   D"),
        0x1C => println!("inc   E"),
        0x24 => println!("inc   H"),
        0x2C => println!("inc   L"),
        0x34 => println!("inc   (HL)"),

        0xBE => println!("cp    (HL)"),
        0xBF => println!("cp    A"),
        0xB8 => println!("cp    B"),
        0xB9 => println!("cp    C"),
        0xBA => println!("cp    D"),
        0xBB => println!("cp    E"),
        0xBC => println!("cp    H"),
        0xBD => println!("cp    L"),

        0x35 => println!("dec    (HL)"),
        0x3D => println!("dec    A"),
        0x05 => println!("dec    B"),
        0x0D => println!("dec    C"),
        0x15 => println!("dec    D"),
        0x1D => println!("dec    E"),
        0x25 => println!("dec    H"),
        0x2D => println!("dec    L"),

        0x87 => println!("add   A,A"),
        0x80 => println!("add   A,B"),
        0x81 => println!("add   A,C"),
        0x82 => println!("add   A,D"),
        0x83 => println!("add   A,E"),
        0x84 => println!("add   A,H"),
        0x85 => println!("add   A,L"),
        0x86 => println!("add   A,(HL)"),

        0x09 => println!("add   HL,BC"),
        0x19 => println!("add   HL,DE"),
        0x29 => println!("add   HL,HL"),
        0x39 => println!("add   HL,SP"),

        0x03 => println!("inc   BC"),
        0x13 => println!("inc   DE"),
        0x23 => println!("inc   HL"),
        0x33 => println!("inc   SP"),

        0x0B => println!("dec   BC"),
        0x1B => println!("dec   DE"),
        0x2B => println!("dec   HL"),
        0x3B => println!("dec   SP"),

        0xCB37 => println!("swap    A"),
        0xCB30 => println!("swap    B"),
        0xCB31 => println!("swap    C"),
        0xCB32 => println!("swap    D"),
        0xCB33 => println!("swap    E"),
        0xCB34 => println!("swap    H"),
        0xCB35 => println!("swap    L"),
        0xCB36 => println!("swap    (HL)"),

        0x27 => println!("daa"),
        0x2F => println!("cpl"),

        0x37 => println!("ccf"),
        0x3F => println!("scf"),

        // JP (HL)
        0xE9 => println!("jp    hl"),

        // RST nn
        0xC7 => println!("rst    0x00"),
        0xCF => println!("rst    0x08"),
        0xD7 => println!("rst    0x10"),
        0xDF => println!("rst    0x18"),
        0xE7 => println!("rst    0x20"),
        0xEF => println!("rst    0x28"),
        0xF7 => println!("rst    0x30"),
        0xFF => println!("rst    0x38"),

        0x7F => println!("ld    A,A"),
        0x78 => println!("ld    A,B"),
        0x79 => println!("ld    A,C"),
        0x7A => println!("ld    A,D"),
        0x7B => println!("ld    A,E"),
        0x7C => println!("ld    A,H"),
        0x7D => println!("ld    A,L"),
        0x7E => println!("ld    A,(HL)"),
        0x47 => println!("ld    B,A"),
        0x40 => println!("ld    B,B"),
        0x41 => println!("ld    B,C"),
        0x42 => println!("ld    B,D"),
        0x43 => println!("ld    B,E"),
        0x44 => println!("ld    B,H"),
        0x45 => println!("ld    B,L"),
        0x46 => println!("ld    B,(HL)"),
        0x4F => println!("ld    C,A"),
        0x48 => println!("ld    C,B"),
        0x49 => println!("ld    C,C"),
        0x4A => println!("ld    C,D"),
        0x4B => println!("ld    C,E"),
        0x4C => println!("ld    C,H"),
        0x4D => println!("ld    C,L"),
        0x4E => println!("ld    C,(HL)"),
        0x57 => println!("ld    D,A"),
        0x50 => println!("ld    D,B"),
        0x51 => println!("ld    D,C"),
        0x52 => println!("ld    D,D"),
        0x53 => println!("ld    D,E"),
        0x54 => println!("ld    D,H"),
        0x55 => println!("ld    D,L"),
        0x56 => println!("ld    D,(HL)"),
        0x5F => println!("ld    E,A"),
        0x58 => println!("ld    E,B"),
        0x59 => println!("ld    E,C"),
        0x5A => println!("ld    E,D"),
        0x5B => println!("ld    E,E"),
        0x5C => println!("ld    E,H"),
        0x5D => println!("ld    E,L"),
        0x5E => println!("ld    E,(HL)"),
        0x67 => println!("ld    H,A"),
        0x60 => println!("ld    H,B"),
        0x61 => println!("ld    H,C"),
        0x62 => println!("ld    H,D"),
        0x63 => println!("ld    H,E"),
        0x64 => println!("ld    H,H"),
        0x65 => println!("ld    H,L"),
        0x66 => println!("ld    H,(HL)"),
        0x6F => println!("ld    L,A"),
        0x68 => println!("ld    L,B"),
        0x69 => println!("ld    L,C"),
        0x6A => println!("ld    L,D"),
        0x6B => println!("ld    L,E"),
        0x6C => println!("ld    L,H"),
        0x6D => println!("ld    L,L"),
        0x6E => println!("ld    L,(HL)"),

        0x0A => println!("ld    A,(BC)"),
        0x1A => println!("ld    A,(DE)"),

        0x77 => println!("ld    (HL),A"),
        0x70 => println!("ld    (HL),B"),
        0x71 => println!("ld    (HL),C"),
        0x72 => println!("ld    (HL),D"),
        0x73 => println!("ld    (HL),E"),
        0x74 => println!("ld    (HL),H"),
        0x75 => println!("ld    (HL),L"),
        0x02 => println!("ld    (BC),A"),
        0x12 => println!("ld    (DE),A"),

        0xA7 => println!("and    A"),
        0xA0 => println!("and    B"),
        0xA1 => println!("and    C"),
        0xA2 => println!("and    D"),
        0xA3 => println!("and    E"),
        0xA4 => println!("and    H"),
        0xA5 => println!("and    L"),
        0xA6 => println!("and    (HL)"),

        0xB7 => println!("or    A"),
        0xB0 => println!("or    B"),
        0xB1 => println!("or    C"),
        0xB2 => println!("or    D"),
        0xB3 => println!("or    E"),
        0xB4 => println!("or    H"),
        0xB5 => println!("or    L"),
        0xB6 => println!("or    (HL)"),

        0xAF => println!("xor    A"),
        0xA8 => println!("xor    B"),
        0xA9 => println!("xor    C"),
        0xAA => println!("xor    D"),
        0xAB => println!("xor    E"),
        0xAC => println!("xor    H"),
        0xAD => println!("xor    L"),
        0xAE => println!("xor    (HL)"),

        0xF5 => println!("push  AF"),
        0xC5 => println!("push  BC"),
        0xD5 => println!("push  DE"),
        0xE5 => println!("push  HL"),

        0x97 => println!("sub    A"),
        0x90 => println!("sub    B"),
        0x91 => println!("sub    C"),
        0x92 => println!("sub    D"),
        0x93 => println!("sub    E"),
        0x94 => println!("sub    H"),
        0x95 => println!("sub    L"),
        0x96 => println!("sub    (HL)"),

        0xF9 => println!("LD    SP,HL"),

        0xE2 => println!("LD    (0xFF00+C),A"),
        0xF2 => println!("LD    A,(0xFF00+C)"),

        0x22 => println!("ldi    (HL),A"),
        0x2A => println!("ldi    A,(HL)"),

        0x32 => println!("ldd    (HL),A"),
        0x3A => println!("ldd    A,(HL)"),

        0xF1 => println!("pop  AF"),
        0xC1 => println!("pop  BC"),
        0xD1 => println!("pop  DE"),
        0xE1 => println!("pop  HL"),

        // RET
        0xC9 => println!("ret"),
        0xC0 => println!("ret   NZ"),
        0xC8 => println!("ret   Z"),
        0xD0 => println!("ret   NC"),
        0xD8 => println!("ret   C"),

        0x8F => println!("adc    A"),
        0x88 => println!("adc    B"),
        0x89 => println!("adc    C"),
        0x8A => println!("adc    D"),
        0x8B => println!("adc    E"),
        0x8C => println!("adc    H"),
        0x8D => println!("adc    L"),
        0x8E => println!("adc    (HL)"),

        0x9F => println!("sbc    A"),
        0x98 => println!("sbc    B"),
        0x99 => println!("sbc    C"),
        0x9A => println!("sbc    D"),
        0x9B => println!("sbc    E"),
        0x9C => println!("sbc    H"),
        0x9D => println!("sbc    L"),
        0x9E => println!("sbc    (HL)"),

        // Interrupt disabling and enabling
        0xF3 => println!("di"),
        0xFB => println!("ei"),

        0xD9 => println!("reti"),

        0x07 => println!("rlca"),
        0x17 => println!("rla"),

        0x0F => println!("rrca"),
        0x1F => println!("rra"),

        // HALT
        0x76 => println!("halt"),

        0xCB17 => println!("rl  A"),
        0xCB10 => println!("rl  B"),
        0xCB11 => println!("rl  C"),
        0xCB12 => println!("rl  D"),
        0xCB13 => println!("rl  E"),
        0xCB14 => println!("rl  H"),
        0xCB15 => println!("rl  L"),

        0xCB1F => println!("rr  A"),
        0xCB18 => println!("rr  B"),
        0xCB19 => println!("rr  C"),
        0xCB1A => println!("rr  D"),
        0xCB1B => println!("rr  E"),
        0xCB1C => println!("rr  H"),
        0xCB1D => println!("rr  L"),

        0xCB7C => println!("bit     7,H"),

        0xCB87 => println!("res     0,A"),

        _ => println!("opcode: {:x} ; disassembler broke", opcode),
    };
}
