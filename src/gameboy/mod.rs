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

mod timer;

pub struct Gameboy {
    mem: mmu::Mmu,
    cpu: cpu::Cpu,
    gpu: gpu::Gpu,
    timer: timer::Timer,
    // This counts the number of cycles we have to burn after the last instruction for proper timing
    burn_count: u8,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            mem: mmu::Mmu::new(),
            cpu: cpu::Cpu::new(),
            gpu: gpu::Gpu::new(),
            timer: timer::Timer::new(),
            burn_count: 0,
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
            if self.burn_count == 0 {
                self.burn_count = self.cpu.exec_next(&mut self.mem) - 1;
            }
            else {
                self.burn_count -= 1;
            }

            //let elapsed = start.elapsed();
            if cfg!(debug_assertions = true) {
                //println!("elapsed nanos: {}", elapsed.subsec_nanos());
            }

            self.gpu.update(&mut self.mem);
            self.timer.update(&mut self.mem);
        }
    }

    pub fn render(&mut self, window: &mut Window, e: &Event) {
        self.gpu.render(window, e);
    }

}
