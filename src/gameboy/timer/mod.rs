use super::mmu::Mmu;

#[cfg(test)]
mod tests;

const DIV_INC_RATE: u16 = 64;

pub struct Timer {
    machine_clocks_per_inc: u16,
    clock_count: u16,
    div_clock_count: u16,
    state: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { machine_clocks_per_inc: 0, clock_count: 0, div_clock_count: 0, state: 0 }
    }

    pub fn update(&mut self, mem: &mut Mmu) {
        self.div_clock_count += 1;

        let div = mem.get_mem_u8(0xFF04);
        let tima = mem.get_mem_u8(0xFF05);
        let tma = mem.get_mem_u8(0xFF06);
        let tac = mem.get_mem_u8(0xFF07);

        if self.div_clock_count % DIV_INC_RATE == 0 {
            mem.set_mem_u8(0xFF04, div + 1);
            self.div_clock_count = 0;
        }

        self.state = (tac & 0b00000100) >> 2;

        if self.state == 1 {
            self.clock_count += 1;
            self.set_freq(mem, &tac, &tma);
            if self.clock_count % self.machine_clocks_per_inc == 0 {
                if tima == 0xFF {
                    mem.set_mem_u8(0xFF05, tma);
                    let interrupts = mem.get_mem_u8(0xFF0F);
                    mem.set_mem_u8(0xFF0F, interrupts | 0b00000100);
                    self.clock_count = 0;
                }
                else {
                    mem.set_mem_u8(0xFF05, tima + 1);
                }
            }
        }
    }

    fn set_freq(&mut self, mem: &mut Mmu, tac: &u8, tma: &u8) {
        if *tac & 0b00000011 == 0 && self.machine_clocks_per_inc != (1024/4) {
            self.machine_clocks_per_inc = 1024/4;
            mem.set_mem_u8(0xFF05, *tma);
        }
        else if *tac & 0b00000011 == 1 && self.machine_clocks_per_inc != (16/4) {
            self.machine_clocks_per_inc = 16/4;
            mem.set_mem_u8(0xFF05, *tma);
        }
        else if *tac & 0b00000011 == 2 && self.machine_clocks_per_inc != (64/4) {
            self.machine_clocks_per_inc = 64/4;
            mem.set_mem_u8(0xFF05, *tma);
        }
        else if *tac & 0b00000011 == 3 && self.machine_clocks_per_inc != (256/4) {
            self.machine_clocks_per_inc = 256/4;
            mem.set_mem_u8(0xFF05, *tma);
        }
    }
}
