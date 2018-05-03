use std::time::{Duration, Instant};
use std::thread;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;

mod cpu;

mod mmu;

mod gpu;

pub struct Gameboy {
    mem: mmu::Mmu,
    cpu: cpu::Cpu,
    gpu: gpu::Gpu,
    clock_speed: u64,
    nanos_per_cycle: Duration,
    cycles_per_frame: u64,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            mem: mmu::Mmu::new(),
            cpu: cpu::Cpu::new(),
            gpu: gpu::Gpu::new(),
            clock_speed: 4194304,
            nanos_per_cycle: Duration::from_nanos((1_000_000_000/(4194304/4))),
            cycles_per_frame: (4194304/4)/60,
        }
    }

    pub fn load_bios(&mut self, file_name: &str) {
        self.mem.load_bios(file_name);
    }

    pub fn load_rom(&mut self, file_name: &str) {
        self.mem.load_rom(file_name);
    }

    pub fn run_game(&mut self) {
        // run the machine cycles for this frame
        for i in 0..(70224/4) {
            let start = Instant::now();
            self.cpu.exec_next(&mut self.mem);

            let elapsed = start.elapsed();
            if cfg!(debug_assertions = true) {
                println!("elapsed nanos: {}", elapsed.subsec_nanos());
            }

            self.gpu.update(&mut self.mem);
        }
    }

    pub fn render(&mut self, window: &mut Window, e: &Event) {
        self.gpu.render(window, e);
    }

}
