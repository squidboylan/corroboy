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
}

