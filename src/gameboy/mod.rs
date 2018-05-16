use std::io::{stdin,stdout,Write};

use piston::input::*;
use piston_window::PistonWindow as Window;

mod cpu;

mod mmu;

mod gpu;

mod joypad;

mod timer;

mod disassembler;

pub struct Gameboy {
    mem: mmu::Mmu,
    cpu: cpu::Cpu,
    gpu: gpu::Gpu,
    timer: timer::Timer,
    p1: joypad::Joypad,
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
            p1: joypad::Joypad::new(),
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
        self.p1.update(&mut self.mem);
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

    pub fn run_game_debug(&mut self) {
        let mut i = 0;
        let mut input_text = String::new();
        let mut get_input = true;
        let mut break_addr: i32 = -1;

        disassembler::display_disassembly(&self.mem, self.cpu.get_pc() as usize);
        loop {
            self.p1.update(&mut self.mem);
            print!("debugger> ");
            let mut input_text = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut input_text).expect("failed to read from stdin");
            let input_split = input_text.trim().to_string();
            let input_split = input_split.split(" ").collect::<Vec<&str>>();
            if input_split[0] == "step" || input_text == "\n" {
                if input_split.len() == 1 {
                    self.step(1);
                }
                else {
                    self.step(u64::from_str_radix(input_split[1], 10).unwrap());
                }
                disassembler::display_disassembly(&self.mem, self.cpu.get_pc() as usize);
                stdout().flush().unwrap();
            }

            else if input_split[0] == "show" {
                if input_split[1] == "regs" {
                    self.cpu.print_regs();
                }
                else if input_split[1] == "flags" {
                    self.cpu.print_flags();
                }
                else if input_split[1] == "disasm" {
                    disassembler::display_disassembly(&self.mem, usize::from_str_radix(input_split[2], 16).unwrap());
                }
                else if input_split[1] == "mem" {
                    println!("val: {:02x}", self.mem.get_mem_u8(usize::from_str_radix(input_split[2], 16).unwrap()));
                }
                stdout().flush().unwrap();
            }

            else if input_split[0] == "break" {
                break_addr = i32::from_str_radix(input_split[1], 16).unwrap();
            }

            else if input_split[0] == "run" {
                self.run_to_break(break_addr);
                disassembler::display_disassembly(&self.mem, self.cpu.get_pc() as usize);
                stdout().flush().unwrap();
            }

            else if input_split[0] == "help" {
                println!("available commands: 'step', 'show regs', 'show flags'");
            }
        }
    }

    fn step(&mut self, steps: u64) {
        let mut i: u64 = 0;
        while i < steps {
            self.burn_count = self.cpu.exec_next(&mut self.mem) - 1;
            while self.burn_count > 0 {
                self.burn_count -= 1;

                self.gpu.update(&mut self.mem);
                self.timer.update(&mut self.mem);
            }
            i += 1;
        }
    }

    fn run_to_break(&mut self, break_addr: i32) {
        if break_addr < 0 {
            loop {
                if self.burn_count == 0 {
                    self.burn_count = self.cpu.exec_next(&mut self.mem) - 1;
                }
                if self.burn_count > 0 {
                    self.burn_count -= 1;

                    self.gpu.update(&mut self.mem);
                    self.timer.update(&mut self.mem);
                }
            }
        }

        if break_addr >= 0 {
            loop {
                if self.burn_count == 0 {
                    if (self.cpu.get_pc() == break_addr as u16) {
                        return;
                    }
                    self.burn_count = self.cpu.exec_next(&mut self.mem) - 1;
                }
                if self.burn_count > 0 {
                    self.burn_count -= 1;

                    self.gpu.update(&mut self.mem);
                    self.timer.update(&mut self.mem);
                }
            }
        }
    }

    pub fn render(&mut self, window: &mut Window, e: &Event) {
        self.gpu.render(window, e, &mut self.mem);
    }

    pub fn press_input(&mut self, but: Button) {
        self.p1.press_input(but);
    }

    pub fn release_input(&mut self, but: Button) {
        self.p1.release_input(but);
    }

}
