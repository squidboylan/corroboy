use corroboy::mmu::Mmu;
// JP nn

pub fn jp(val: u16, pc: &mut u16) {
    *pc = val;
}

pub fn jp_nz(val: u16, flags: u8, pc: &mut u16) {
    if flags & 0b10000000 == 0 {
        *pc = val;
    }
}

pub fn jp_z(val: u16, flags: u8, pc: &mut u16) {
    if flags & 0b10000000 != 0 {
        *pc = val;
    }
}

pub fn jp_nc(val: u16, flags: u8, pc: &mut u16) {
    if flags & 0b00010000 == 0 {
        *pc = val;
    }
}

pub fn jp_c(val: u16, flags: u8, pc: &mut u16) {
    if flags & 0b00010000 != 0 {
        *pc = val;
    }
}

// JR n

pub fn jr(val: u8, pc: &mut u16) {
    *pc = (*pc as i16 + ((val as i8) as i16)) as u16;
}

pub fn jr_nz(val: u8, flags: u8, pc: &mut u16) {
    if flags & 0b10000000 == 0 {
        *pc = (*pc as i16 + ((val as i8) as i16)) as u16;
    }
}

pub fn jr_z(val: u8, flags: u8, pc: &mut u16) {
    if flags & 0b10000000 != 0 {
        *pc = (*pc as i16 + ((val as i8) as i16)) as u16;
    }
}

pub fn jr_nc(val: u8, flags: u8, pc: &mut u16) {
    if flags & 0b00010000 == 0 {
        *pc = (*pc as i16 + ((val as i8) as i16)) as u16;
    }
}

pub fn jr_c(val: u8, flags: u8, pc: &mut u16) {
    if flags & 0b00010000 != 0 {
        *pc = (*pc as i16 + ((val as i8) as i16)) as u16;
    }
}

pub fn call(val: u16, pc: &mut u16, sp: &mut u16, mem: &mut Mmu) {
    mem.push_u16(sp, *pc);
    *pc = val;
}

pub fn call_nz(val: u16, flags: u8, pc: &mut u16, sp: &mut u16, mem: &mut Mmu) {
    if flags & 0b10000000 == 0 {
        mem.push_u16(sp, *pc);
        *pc = val;
    }
}

pub fn call_z(val: u16, flags: u8, pc: &mut u16, sp: &mut u16, mem: &mut Mmu) {
    if flags & 0b10000000 != 0 {
        mem.push_u16(sp, *pc);
        *pc = val;
    }
}

pub fn call_nc(val: u16, flags: u8, pc: &mut u16, sp: &mut u16, mem: &mut Mmu) {
    if flags & 0b00010000 == 0 {
        mem.push_u16(sp, *pc);
        *pc = val;
    }
}

pub fn call_c(val: u16, flags: u8, pc: &mut u16, sp: &mut u16, mem: &mut Mmu) {
    if flags & 0b00010000 != 0 {
        mem.push_u16(sp, *pc);
        *pc = val;
    }
}
