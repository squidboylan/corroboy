pub mod gameboy;

extern crate piston;
extern crate piston_window;
extern crate sdl2_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate getopts;
extern crate gfx_device_gl;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
use opengl_graphics::{ OpenGL };
#[allow(unused_imports)]
use std::time::{Duration, Instant};
use getopts::Options;
use sdl2_window::Sdl2Window;

use std::env;

/// Print help message info
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
    opts.optopt("z", "zoom", "Factor to scale the gameboy graphics by", "INT");
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

    let bios_path_option: Option<String> = matches.opt_str("b");
    let rom_path_option: Option<String> = matches.opt_str("R");
    let zoom_option: Option<String> = matches.opt_str("z");

    // Default zoom by 3
    let mut zoom: u32 = 3;
    if let Some(i) = zoom_option {
        zoom = i.parse::<u32>().unwrap();
    }

    let bios_path: String;
    if bios_path_option == None {
        print_usage(&program, opts);
        return;
    }
    bios_path = bios_path_option.unwrap();

    let rom_path: String;// = String::new();
    if rom_path_option == None {
        print_usage(&program, opts);
        return;
    }
    rom_path = rom_path_option.unwrap();

    // Setup graphics window
    let mut window: Window<Sdl2Window> = WindowSettings::new("gameboy-emu", [160 * zoom, 144 * zoom])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    // Get a gameboy object
    let mut gb = gameboy::Gameboy::new(&mut window, zoom);

    gb.load_bios(&bios_path);
    gb.load_rom(&rom_path);

    // If -d was passed run in debug mode
    if matches.opt_present("d") {
        gb.run_game_debug();
    }
    // If we're not in debug mode run the normal way
    else {
        window.set_ups(60);
        window.set_max_fps(60);
        while let Some(e) = window.next() {
            if let Some(_r) = e.render_args() {
                let now = Instant::now();
                gb.render(&mut window, &e);
                println!("{}", now.elapsed().subsec_micros());
            }

            if let Some(_u) = e.update_args() {
                gb.run_game();
            }

            if let Some(inp) = e.press_args() {
                gb.press_input(inp);
            }

            if let Some(inp) = e.release_args() {
                gb.release_input(inp);
            }
        }
    }
}
