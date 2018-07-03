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
        Timer {
            machine_clocks_per_inc: 0,
            clock_count: 0,
            div_clock_count: 0,
            state: 0,
        }
    }

    /// Process the timer for the current cycle
    /// Always increment DIV
    /// Update the timer state based on the values of the TMA and TAC registers
    /// Increment TIMA if the timer is enabled and the number of cycles has passed to update it
    pub fn update(&mut self, mem: &mut Mmu) {
        self.div_clock_count += 1;

        let div = mem.get_div();
        let tima = mem.get_tima();
        let tma = mem.get_tma();
        let tac = mem.get_tac();

        if self.div_clock_count % DIV_INC_RATE == 0 {
            mem.set_div(div + 1);
            self.div_clock_count = 0;
        }

        self.state = (tac & 0b00000100) >> 2;

        if self.state == 1 {
            self.clock_count += 1;
            self.set_freq(mem, &tac, &tma);
            if self.clock_count % self.machine_clocks_per_inc == 0 {
                if tima == 0xFF {
                    mem.set_tima(tma);
                    let interrupts = mem.get_interrupts();
                    mem.set_interrupts(interrupts | 0b00000100);
                    self.clock_count = 0;
                } else {
                    mem.set_tima(tima + 1);
                }
            }
        }
    }

    /// Update the timer state based on the tac and tma registers
    fn set_freq(&mut self, mem: &mut Mmu, tac: &u8, tma: &u8) {
        if *tac & 0b00000011 == 0 && self.machine_clocks_per_inc != (1024 / 4) {
            self.machine_clocks_per_inc = 1024 / 4;
            mem.set_tima(*tma);
        } else if *tac & 0b00000011 == 1 && self.machine_clocks_per_inc != (16 / 4) {
            self.machine_clocks_per_inc = 16 / 4;
            mem.set_tima(*tma);
        } else if *tac & 0b00000011 == 2 && self.machine_clocks_per_inc != (64 / 4) {
            self.machine_clocks_per_inc = 64 / 4;
            mem.set_tima(*tma);
        } else if *tac & 0b00000011 == 3 && self.machine_clocks_per_inc != (256 / 4) {
            self.machine_clocks_per_inc = 256 / 4;
            mem.set_tima(*tma);
        }
    }
}
