pub mod gameboy;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut derp = gameboy::Cpu::new();
    derp.load_rom(&args[1]);
    derp.run();
}
