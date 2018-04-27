use std::time::{Duration, Instant};

pub struct Timer {
    last_instant: Instant,

    // freq in hz
    freq: u32,

    // How many ticks before the tma increments
    tma_div: u8,

    // Current tma value, generates interrupt on overflow
    tma_val: u8,
    count: u8,
    interrupt: bool,
    enabled: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { last_instant: Instant::now(), freq: 0, tma_div: 0, tma_val: 0, interrupt: false, enabled: false }
    }

    pub fn set_freq(&mut self, freq &u32) {
        if freq >= 4 {
            self.enabled = true;
            self.last_instant = Instant::now();
            if freq & 0b011 == 0b00 {
                self.freq = 4096;
            }
            else if freq & 0b011 == 0b01 {
                self.freq = 262144;
            }
            else if freq & 0b011 == 0b10 {
                self.freq = 65536;
            }
            else if freq & 0b011 == 0b11 {
                self.freq = 16384;
            }
        }

        else {
            self.enabled = false;
        }
        self.count = 0;
    }

    pub fn set_tma_div(&mut self, tma_div &u8) {
        self.tma_div = tma_div * -1;
        self.count = 0;
    }

    pub fn update(&mut self) -> bool {
        if enabled == true {
            let interval_nanos = 1000_000_000/freq;

            if last_instant.subsec_nanos() > tma_div * interval_nanos {
                tma_val += 1;

                self.last_instant = Instant::now();

                if tma_val == 0 {
                    return true;
                }
            }
        }

        return false;
    }
}
