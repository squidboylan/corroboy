use super::Mmu;

#[test]
fn mmu_ram() {
    let mut derp = Mmu::new(None);
    derp.set_mem_u8(0xC000, 255);
    assert_eq!(derp.get_mem_u8(0xC000), 255);

    derp.set_mem_u8(0xC001, 10);
    assert_eq!(derp.get_mem_u8(0xC001), 10);

    derp.set_mem_u8(0xC002, 1);
    assert_eq!(derp.get_mem_u8(0xC002), 1);

    derp.set_mem_u16(0xC003, 0x1234);
    assert_eq!(derp.get_mem_u8(0xC003), 0x34);
    assert_eq!(derp.get_mem_u8(0xC004), 0x12);
    assert_eq!(derp.get_mem_u16(0xC003), 0x1234);
}

#[test]
fn stack_functions() {
    let mut derp = Mmu::new(None);
    let mut sp: u16 = 0xDFFF;

    derp.push_u16(&mut sp, 0x3210);
    assert_eq!(derp.pop_u16(&mut sp), 0x3210);
}
