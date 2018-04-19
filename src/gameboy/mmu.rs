pub struct Mmu {
    ram: [u8; 8192],
    cartridge: [u8; 32768],
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu{ ram: [0; 8192], cartridge: [0; 32768] }
    }

    pub fn get_ram_u8(&self, location: usize) -> u8 {
        self.ram[location]
    }

    pub fn set_ram_u8(&mut self, location: usize, val: u8) {
        self.ram[location] = val;
    }
}

#[test]
fn test_mmu_ram() {
    let mut derp = Mmu::new();
    derp.set_ram_u8(0, 255);
    assert_eq!(derp.get_ram_u8(0), 255);
    derp.set_ram_u8(1, 10);
    assert_eq!(derp.get_ram_u8(1), 10);
    derp.set_ram_u8(2, 1);
    assert_eq!(derp.get_ram_u8(2), 1);
}
