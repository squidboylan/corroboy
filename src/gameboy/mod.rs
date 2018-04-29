use std::time::{Duration, Instant};
use std::thread;

mod cpu;

mod mmu;

pub fn run_game(file_name: &str) {
    let clock_speed: u64 = 4_194_000; // Hz
    let nanos_per_cycle = Duration::from_nanos((1_000_000_000/(clock_speed/4)));
    let mut mem = mmu::Mmu::new();
    let mut cpu = cpu::Cpu::new();
    mem.load_rom(file_name);
    loop {
        let start = Instant::now();
        cpu.exec_next(&mut mem);

        let elapsed = start.elapsed();
        if cfg!(debug_assertions = true) {
            println!("elapsed nanos: {}", elapsed.subsec_nanos());
        }

        if elapsed < nanos_per_cycle {
            thread::sleep(nanos_per_cycle - elapsed);
        }

    }
}
