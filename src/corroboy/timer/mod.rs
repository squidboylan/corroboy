use super::mmu::Mmu;

#[cfg(test)]
mod tests;

pub struct Timer {
    machine_clocks_per_inc: u16,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            machine_clocks_per_inc: 0,
        }
    }

    /// Process the timer for the current cycle
    /// Always increment DIV
    /// Update the timer state based on the values of the TMA and TAC registers
    /// Increment TIMA if the timer is enabled and the number of cycles has passed to update it
    pub fn update(&mut self, mem: &mut Mmu) {
        let mut system_counter = mem.get_system_counter();

        system_counter += 4;

        let tima = mem.get_tima();
        let tma = mem.get_tma();
        let tac = mem.get_tac();

        let state = (tac & 0b00000100) >> 2;

        if state == 1 {
            self.set_freq(&tac);
            if tima == 0 {
                mem.set_tima(tma);
                let interrupts = mem.get_interrupts();
                mem.set_interrupts(interrupts | 0b00000100);
            }
            if system_counter % self.machine_clocks_per_inc == 0 {
                mem.set_tima(tima + 1);
            }
        }
        mem.set_system_counter(system_counter);
    }

    /// Update the timer state based on the tac and tma registers
    fn set_freq(&mut self, tac: &u8) {
        if *tac & 0b00000011 == 0 {
            self.machine_clocks_per_inc = 1024;
        } else if *tac & 0b00000011 == 1 {
            self.machine_clocks_per_inc = 16;
        } else if *tac & 0b00000011 == 2 {
            self.machine_clocks_per_inc = 64;
        } else if *tac & 0b00000011 == 3 {
            self.machine_clocks_per_inc = 256;
        }
    }
}
