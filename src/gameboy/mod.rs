use std::time::{Duration, Instant};

mod cpu;

mod mmu;

pub fn run_game(file_name: &str) {
    let mut mem = mmu::Mmu::new();
    let mut cpu = cpu::Cpu::new();
    mem.load_rom(file_name);
    loop {
        let start = Instant::now();
        cpu.exec_next(&mut mem);

        if cfg!(debug_assertions = true) {
            println!("elapsed nanos: {}", start.elapsed().subsec_nanos());
        }
    }
}
