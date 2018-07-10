use std::io::{stdin, stdout, Write};

use piston::input::*;
use piston_window::PistonWindow as Window;
use sdl2_window::Sdl2Window;

mod cpu;

mod mmu;

mod gpu;

mod joypad;

mod timer;

mod disassembler;

pub struct Emulator {
    mem: mmu::Mmu,
    cpu: cpu::Cpu,
    gpu: gpu::Gpu,
    timer: timer::Timer,
    p1: joypad::Joypad,
    // This counts the number of cycles we have to burn after the last instruction for proper timing
    burn_count: u8,
}

impl Emulator {
    pub fn new(window: &mut Window<Sdl2Window>, zoom: u32, save_file: Option<String>) -> Emulator {
        Emulator {
            mem: mmu::Mmu::new(save_file),
            cpu: cpu::Cpu::new(),
            gpu: gpu::Gpu::new(window, zoom),
            timer: timer::Timer::new(),
            p1: joypad::Joypad::new(),
            burn_count: 0,
        }
    }

    /// Load the bios rom data
    pub fn load_bios(&mut self, file_name: &str) {
        self.mem.load_bios(file_name);
    }

    /// Load the game rom data
    pub fn load_rom(&mut self, file_name: &str) {
        self.mem.load_rom(file_name);
    }

    /// Run one frame of the emulator
    pub fn run_game(&mut self) {
        // run the machine cycles for this frame
        loop {
            if self.burn_count == 0 {
                self.p1.update(&mut self.mem);
                self.burn_count = self.cpu.exec_next(&mut self.mem) - 1;
            } else {
                self.burn_count -= 1;
            }

            //let elapsed = start.elapsed();
            if cfg!(debug_assertions = true) {
                //println!("elapsed nanos: {}", elapsed.subsec_nanos());
            }

            let pre_gpu_state = self.mem.get_io_register(0xFF41) & 0b00000011;

            self.gpu.update(&mut self.mem);
            self.timer.update(&mut self.mem);

            let post_gpu_state = self.mem.get_io_register(0xFF41) & 0b00000011;

            if pre_gpu_state == 0 && post_gpu_state == 1 {
                return;
            }
        }
    }

    /// Run the emulator in debug mode, allows stepping through instructions one by one or breaking
    /// at a certain location
    pub fn run_game_debug(&mut self) {
        let mut break_addr: i32 = -1;

        disassembler::display_disassembly(&self.mem, self.cpu.get_pc() as usize);
        loop {
            print!("debugger> ");
            let mut input_text = String::new();
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            let input_split = input_text.trim().to_string();
            let input_split = input_split.split(" ").collect::<Vec<&str>>();
            if input_split[0] == "step" || input_text == "\n" {
                if input_split.len() == 1 {
                    self.step(1);
                } else {
                    self.step(u64::from_str_radix(input_split[1], 10).unwrap());
                }
                disassembler::display_disassembly(&self.mem, self.cpu.get_pc() as usize);
                stdout().flush().unwrap();
            } else if input_split[0] == "show" {
                if input_split[1] == "regs" {
                    self.cpu.print_regs();
                } else if input_split[1] == "flags" {
                    self.cpu.print_flags();
                } else if input_split[1] == "disasm" {
                    disassembler::display_disassembly(
                        &self.mem,
                        usize::from_str_radix(input_split[2], 16).unwrap(),
                    );
                } else if input_split[1] == "mem" {
                    println!(
                        "val: {:02x}",
                        self.mem
                            .get_mem_u8(usize::from_str_radix(input_split[2], 16).unwrap())
                    );
                }
                stdout().flush().unwrap();
            } else if input_split[0] == "break" {
                break_addr = i32::from_str_radix(input_split[1], 16).unwrap();
            } else if input_split[0] == "run" {
                self.run_to_break(break_addr);
                disassembler::display_disassembly(&self.mem, self.cpu.get_pc() as usize);
                stdout().flush().unwrap();
            } else if input_split[0] == "help" {
                println!("available commands: 'step', 'show regs', 'show flags'");
            }
        }
    }

    /// Step X number of instructions
    fn step(&mut self, steps: u64) {
        let mut i: u64 = 0;
        while i < steps {
            self.burn_count = self.cpu.exec_next(&mut self.mem) - 1;
            while self.burn_count > 0 {
                self.burn_count -= 1;

                self.gpu.update(&mut self.mem);
                self.timer.update(&mut self.mem);
                self.p1.update(&mut self.mem);
            }
            i += 1;
        }
    }

    /// Run the emulator until the PC hits a certain location
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
                    self.p1.update(&mut self.mem);
                }
            }
        }

        if break_addr >= 0 {
            loop {
                if self.burn_count == 0 {
                    if self.cpu.get_pc() == break_addr as u16 {
                        return;
                    }
                    self.burn_count = self.cpu.exec_next(&mut self.mem) - 1;
                }
                if self.burn_count > 0 {
                    self.burn_count -= 1;

                    self.gpu.update(&mut self.mem);
                    self.timer.update(&mut self.mem);
                    self.p1.update(&mut self.mem);
                }
            }
        }
    }

    /// Render the current data in the gpu
    pub fn render(&mut self, window: &mut Window<Sdl2Window>, e: &Event) {
        self.gpu.render(window, e, &mut self.mem);
        self.save_cart_ram();
    }

    /// Process an input press
    pub fn press_input(&mut self, but: Button) {
        self.p1.press_input(but);
    }

    /// Process an input release
    pub fn release_input(&mut self, but: Button) {
        self.p1.release_input(but);
    }

    pub fn skip_bios(&mut self) {
        self.cpu.skip_bios();
        self.mem.skip_bios();
    }

    fn save_cart_ram(&mut self) {
        self.mem.save_cart_ram();
    }
}
