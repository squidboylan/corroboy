pub mod gameboy;

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate gfx;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::time::{Duration, Instant};


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let opengl = OpenGL::V3_2;

    let mut window: Window =  WindowSettings::new(
            "gameboy-emu",
            [432, 480]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gb = gameboy::Gameboy::new();

    if args.len() == 3 {
        gb.load_bios(&args[1]);
        gb.load_rom(&args[2]);
    }

    if args.len() == 2 {
        gb.load_bios(&args[1]);
    }

    if args.len() == 1 {
        panic!("pass bios as arg 1 and a rom as arg 2");
    }

    let mut events: Events = Events::new(EventSettings::new());
    events.set_ups(60);
    events.set_max_fps(60);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gb.render(&mut window, &e);
        }

        if let Some(u) = e.update_args() {
            gb.run_game();
        }
    }
}
