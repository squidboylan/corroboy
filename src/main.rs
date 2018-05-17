pub mod gameboy;

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate cpuprofiler;
extern crate getopts;
extern crate gfx_device_gl;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
use opengl_graphics::{ OpenGL };
#[allow(unused_imports)]
use std::time::{Duration, Instant};
use cpuprofiler::PROFILER;
use getopts::Options;

use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -b PATH -R PATH [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opengl = OpenGL::V3_2;
    let mut opts = Options::new();
    let program = args[0].clone();

    // Arg parsing
    opts.optopt("b", "bios", "Bios rom file", "PATH");
    opts.optopt("R", "rom", "Game rom file", "PATH");
    opts.optflag("d", "debug", "debug mode");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut bios_path_option: Option<String> = matches.opt_str("b");
    let mut rom_path_option: Option<String> = matches.opt_str("R");

    let mut bios_path: String = String::new();
    if bios_path_option == None {
        print_usage(&program, opts);
        return;
    }
    else {
        bios_path = bios_path_option.unwrap();
    }

    let mut rom_path: String = String::new();
    if rom_path_option == None {
        print_usage(&program, opts);
        return;
    }
    else {
        rom_path = rom_path_option.unwrap();
    }

    // Setup graphics window
    let mut window: Window =  WindowSettings::new(
            "gameboy-emu",
            [432, 480]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Get a gameboy object
    let mut gb = gameboy::Gameboy::new(&mut window);

    gb.load_bios(&bios_path);
    gb.load_rom(&rom_path);

    // If -d was passed run in debug mode
    if matches.opt_present("d") {
        gb.run_game_debug();
    }
    // If we're not in debug mode run the normal way
    else {
        let mut events: Events = Events::new(EventSettings::new());
        events.set_ups(60);
        events.set_max_fps(60);
        PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                gb.render(&mut window, &e);
            }

            if let Some(u) = e.update_args() {
                gb.run_game();
            }

            if let Some(inp) = e.press_args() {
                gb.press_input(inp);
            }

            if let Some(inp) = e.release_args() {
                gb.release_input(inp);
            }
        }
        PROFILER.lock().unwrap().stop().unwrap();
    }
}
