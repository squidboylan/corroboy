pub mod gameboy;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    gameboy::run_game(&args[1]);
}
