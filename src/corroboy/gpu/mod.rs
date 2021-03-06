// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use graphics::*;
use piston::input::*;
use piston_window::PistonWindow as Window;
use sdl2_window::Sdl2Window;

use super::mmu::Mmu;

mod sprite;

mod background;

// TERMS:
// OAM: Object Attribute Memory or sprite attribute table

// From: http://gbdev.gg8.se/wiki/articles/Video_Display#FF41_-_STAT_-_LCDC_Status_.28R.2FW.29
// Mode 0: The LCD controller is in the H-Blank period and
//         the CPU can access both the display RAM (8000h-9FFFh)
//         and OAM (FE00h-FE9Fh)
//
// Mode 1: The LCD controller is in the V-Blank period (or the
//         display is disabled) and the CPU can access both the
//         display RAM (8000h-9FFFh) and OAM (FE00h-FE9Fh)
//
// Mode 2: The LCD controller is reading from OAM memory.
//         The CPU <cannot> access OAM memory (FE00h-FE9Fh)
//         during this period.
//
// Mode 3: The LCD controller is reading from both OAM and VRAM,
//         The CPU <cannot> access OAM and VRAM during this period.
//         CGB Mode: Cannot access Palette Data (FF69,FF6B) either.

// Mode 2: 80 clocks (20 machine cycles)
// Mode 3: 172 clocks (43 machine cycles)
// Mode 0: 204 clocks (51 machine cycles)
// Each line draw takes 456 clocks (114 machine cycles)
// 144 * 456 clocks = 65664 clocks (16416 machine cycles) to draw the whole screen

// Vblank, mode 1, lasts 4560 clocks (1140) between draw end and draw beginning

// screen refresh happens every 70224 clocks (17556)

pub struct Gpu {
    state: u8,

    // This represents the number of (machine) cycles we are into rendering the current line
    count: u16,

    sprite_manager: sprite::SpriteManager,

    background: background::Background,

    zoom: u32,
}

impl Gpu {
    pub fn new(window: &mut Window<Sdl2Window>, zoom: u32) -> Gpu {
        Gpu {
            state: 0,
            count: 0,
            sprite_manager: sprite::SpriteManager::new(),
            background: background::Background::new(window),
            zoom: zoom,
        }
    }

    /// Render the current frame, including background and sprites
    pub fn render(&mut self, window: &mut Window<Sdl2Window>, e: &Event, mem: &mut Mmu) {
        const SCREEN_SIZE_X: u32 = 160;
        const SCREEN_SIZE_Y: u32 = 144;

        self.background.generate_tex(window);

        if get_current_state(mem) == 0 {
            window.draw_2d(e, |_c, g| {
                clear([0.0; 4], g);
            });
        } else {
            if self.sprite_manager.sprites_enabled == true {
                self.sprite_manager.generate_sprite_textures(window);
            }

            // This closure is loosely based on the examples in
            // https://github.com/PistonDevelopers/piston-examples
            // which is available under the MIT license, a copy of which
            // is available in the LICENSE file at the root of the repo
            window.draw_2d(e, |c, g| {
                clear([1.0; 4], g);

                image(
                    &self.background.base_tex,
                    c.transform.zoom(self.zoom as f64),
                    g,
                );

                if self.sprite_manager.sprites_enabled == true {
                    for (num, i) in self.sprite_manager.sprites.iter().enumerate() {
                        if i.x > 0
                            && i.y > 0
                            && (i.x as u32) < SCREEN_SIZE_X + 8
                            && (i.y as u32) < SCREEN_SIZE_Y + 16
                            && i.priority == 1
                        {
                            let mut context = c.transform;
                            if i.x_flip == 1 && i.y_flip == 1 {
                                context = context
                                    .trans(
                                        (i.x as isize * self.zoom as isize) as f64,
                                        (i.y as isize * self.zoom as isize) as f64,
                                    )
                                    .zoom(self.zoom as f64);
                                context = context.flip_hv();
                            } else if i.x_flip == 1 {
                                context = context
                                    .trans(
                                        (i.x as isize * self.zoom as isize) as f64,
                                        ((i.y as isize - 16) * self.zoom as isize) as f64,
                                    )
                                    .zoom(self.zoom as f64);
                                context = context.flip_h();
                            } else if i.y_flip == 1 {
                                context = context
                                    .trans(
                                        ((i.x as isize - 8) * self.zoom as isize) as f64,
                                        (i.y as isize * self.zoom as isize) as f64,
                                    )
                                    .zoom(self.zoom as f64);
                                context = context.flip_v();
                            } else {
                                context = context
                                    .trans(
                                        ((i.x as isize - 8) * self.zoom as isize) as f64,
                                        ((i.y as isize - 16) * self.zoom as isize) as f64,
                                    )
                                    .zoom(self.zoom as f64);
                            }
                            image(self.sprite_manager.get_texture(num), context, g);
                        }
                    }
                }

                image(&self.background.tex, c.transform.zoom(self.zoom as f64), g);

                if self.sprite_manager.sprites_enabled == true {
                    for (num, i) in self.sprite_manager.sprites.iter().enumerate() {
                        if i.x > 0
                            && i.y > 0
                            && (i.x as u32) < SCREEN_SIZE_X + 8
                            && (i.y as u32) < SCREEN_SIZE_Y + 16
                            && i.priority == 0
                        {
                            let mut context = c.transform;
                            if i.x_flip == 1 && i.y_flip == 1 {
                                context = context
                                    .trans(
                                        (i.x as isize * self.zoom as isize) as f64,
                                        ((i.y as isize
                                            - (16 - self.sprite_manager.sprite_height) as isize)
                                            * self.zoom as isize)
                                            as f64,
                                    )
                                    .zoom(self.zoom as f64);
                                context = context.flip_hv();
                            } else if i.x_flip == 1 {
                                context = context
                                    .trans(
                                        (i.x as isize * self.zoom as isize) as f64,
                                        ((i.y as isize - 16) * self.zoom as isize) as f64,
                                    )
                                    .zoom(self.zoom as f64);
                                context = context.flip_h();
                            } else if i.y_flip == 1 {
                                context = context
                                    .trans(
                                        ((i.x as isize - 8) * self.zoom as isize) as f64,
                                        ((i.y as isize
                                            - (16 - self.sprite_manager.sprite_height as isize))
                                            * self.zoom as isize)
                                            as f64,
                                    )
                                    .zoom(self.zoom as f64);
                                context = context.flip_v();
                            } else {
                                context = context
                                    .trans(
                                        ((i.x as isize - 8) * self.zoom as isize) as f64,
                                        ((i.y as isize - 16) * self.zoom as isize) as f64,
                                    )
                                    .zoom(self.zoom as f64);
                            }
                            image(self.sprite_manager.get_texture(num), context, g);
                        }
                    }
                }
            });
        }
    }

    /// Update the gpu based on the current mode of the gpu
    pub fn update(&mut self, mem: &mut Mmu) {
        // If the gpu is changing from off to on initialize the screen and
        // set the gpu to the correct mode
        if self.state == 0 && get_current_state(mem) == 1 {
            self.state = 1;
            self.count = 0;
            set_curr_line(mem, 0);
            set_mode(mem, 0);
        }
        // If the gpu is changing from on to off set the gpu off
        else if self.state == 1 && get_current_state(mem) == 0 {
            self.state = 0;
            set_mode(mem, 0b01);
            return;
        }

        // Do stuff here that happens once per frame
        if self.count == 0 && get_curr_line(mem) == 0 {
            self.initialize_sprites(mem);
        }

        // Based on the current gpu mode call the correct function to update
        // the gpu
        let mode = get_mode(mem);
        if mode == 0 {
            self.h_blank(mem);
        } else if mode == 1 {
            self.v_blank(mem);
        } else if mode == 2 {
            self.setup_line(mem);
        } else if mode == 3 {
            self.render_line(mem);
        }
    }

    /// H-blank we just increase our cycles until we are in the next mode
    fn h_blank(&mut self, mem: &mut Mmu) {
        //println!("h-blanking");
        if self.count < 113 {
            self.count += 1;
        } else {
            if get_curr_line(mem) <= 143 {
                self.count = 0;
                set_mode(mem, 2);
            }
            // If we have rendered everything, V-Blank
            else {
                self.count = 0;
                set_mode(mem, 1);
                let interrupts = mem.get_mem_u8(0xFF0F);
                mem.set_mem_u8(0xFF0F, interrupts | 0b00000001);
            }
        }
    }

    /// V-blank we h-blank for 113 cycles 10 times, increasing the curr_line after each 113
    /// cycles
    fn v_blank(&mut self, mem: &mut Mmu) {
        // Check for lyc = ly even during v_blank
        if self.count == 0 {
            if mem.get_io_register(0xFF45) == get_curr_line(mem) {
                let stat = mem.get_io_register(0xFF41);
                mem.set_io_register(0xFF41, stat | 0b01000000);
                let interrupts = mem.get_mem_u8(0xFF0F);
                mem.set_mem_u8(0xFF0F, interrupts | 0b00000010);
            }
        }
        if self.count < 113 {
            self.count += 1;
        } else {
            let line = get_curr_line(mem);
            if line < 153 {
                self.count = 0;
                set_curr_line(mem, line + 1);
            }
            // If we are at the end of V-blank, go to setup_line for the next frame
            else {
                self.count = 0;
                set_mode(mem, 2);
                set_curr_line(mem, 0);
            }
        }
    }

    /// Do stuff here that has to happen on every line render
    fn setup_line(&mut self, mem: &mut Mmu) {
        //println!("setting up line {}", self.get_curr_line(mem));
        // get the palette, etc
        if self.count == 0 {
            if get_curr_line(mem) == 0 {
                self.sprite_manager.set_sprite_palettes(mem);
                self.sprite_manager.build_sprites(mem);
                self.sprite_manager.build_pattern_data(mem);
            }
            self.background.initialize(mem);
            self.background.build_bg_tile_map(mem);
            self.background.build_window_tile_map(mem);
            self.background.set_bg_palette(mem);
            // Check for lyc = ly interrupt
            if mem.get_io_register(0xFF45) == get_curr_line(mem) {
                let stat = mem.get_io_register(0xFF41);
                mem.set_io_register(0xFF41, stat | 0b01000000);
                let interrupts = mem.get_mem_u8(0xFF0F);
                mem.set_mem_u8(0xFF0F, interrupts | 0b00000010);
            }
        }
        if self.count < 19 {
            self.count += 1;
        } else {
            set_mode(mem, 3);
        }
    }

    /// Render the current line of the background
    fn render_line(&mut self, mem: &mut Mmu) {
        let line_lcd = get_curr_line(mem);
        // Render this line of the background
        if self.count == 19 {
            self.background.update_background_line(line_lcd, mem);
        }
        // Render this line of the window
        if self.count == 20 {
            self.background.update_window_line(line_lcd);
        }
        if self.count < 62 {
            self.count += 1;
        } else {
            set_mode(mem, 0);
            set_curr_line(mem, line_lcd + 1);
        }
    }

    /// Initialize the gpu and all its components
    pub fn initialize_sprites(&mut self, mem: &mut Mmu) {
        let ff40 = mem.get_io_register(0xFF40);
        if ff40 & 0b00000100 == 0 {
            self.sprite_manager.sprite_height = 8;
        } else {
            self.sprite_manager.sprite_height = 16;
        }

        if ff40 & 0b00000010 == 0 {
            self.sprite_manager.sprites_enabled = false;
        } else {
            self.sprite_manager.sprites_enabled = true;
        }
    }
}

#[inline(always)]
fn get_mode(mem: &mut Mmu) -> u8 {
    mem.get_io_register(0xFF41) & 0b00000011
}

#[inline(always)]
// Mode must be 0 - 3, other values will break the game
fn set_mode(mem: &mut Mmu, mode: u8) {
    let tmp = mem.get_io_register(0xFF41) & 0b11111100;
    mem.set_io_register(0xFF41, tmp + mode);
}

#[inline(always)]
fn get_curr_line(mem: &mut Mmu) -> u8 {
    mem.get_io_register(0xFF44)
}

#[inline(always)]
fn set_curr_line(mem: &mut Mmu, line_num: u8) {
    mem.set_io_register(0xFF44, line_num);
}

#[inline(always)]
fn get_current_state(mem: &mut Mmu) -> u8 {
    (mem.get_io_register(0xFF40) & 0b10000000) >> 7
}
