
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
//use glutin_window::GlutinWindow as Window;
use piston_window::PistonWindow as Window;
use piston_window::Texture;
use piston_window;
use piston_window::TextureSettings;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;
//use graphics::image::*;
use image::*;
use gfx;

use super::mmu::Mmu;

// TERMS:
// OAM: Object Attribute Memory or sprite attribute table

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

struct Tile {
    // color nums that represent the tile, this + palette data each line represent how the tile is
    // drawn
    pub raw_val: [[u8; 8]; 8],
}

impl Tile {
    pub fn new() -> Tile {
        Tile{
            raw_val: [[0 ;8]; 8],
        }
    }
    fn display_ascii(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{} ", self.raw_val[i][j]);
            }
            println!("");
        }
        println!("");
    }
}

pub struct Gpu {
    //tiles: Vec<Tile>,
    cycle: u32,
    line_rendering: u16,
    lcd_w: u8,
    lcd_h: u8,
    state: u8,

    background_data_bot: usize,
    background_data_top: usize,
    bg_tiles: Vec<Tile>,

    tile_map_bot: usize,
    tile_map_top: usize,
    tile_map: [[u8; 32]; 32],

    // colors 0 - 3
    bg_palette: [usize; 4],

    // Color representation of each pixel
    // 0 - white
    // 1 - light gray
    // 2 - dark gray
    // 3 - black
    pixel_map: [[usize; 160]; 144],

    // This represents the number of (machine) cycles we are into rendering the current line
    count: u16,

    scx: u8,
    scy: u8,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            cycle: 0,
            line_rendering: 0,
            lcd_w: 160,
            lcd_h: 144,
            state: 0,
            background_data_bot: 0,
            background_data_top: 0,
            bg_tiles: Vec::new(),
            tile_map_bot: 0,
            tile_map_top: 0,
            tile_map: [[0; 32]; 32],
            bg_palette: [0; 4],
            pixel_map: [[1; 160]; 144],
            count: 0,
            scx: 0,
            scy: 0,
        }
    }

    fn print_tile_map(&self) {
        for i in 0..32 {
            for j in 0..32 {
                print!("{} ", self.tile_map[i][j]);
            }
            println!("");
        }
    }

    fn display_ascii(&self) {
        if self.bg_tiles.len() == 256{
            for i in 0..32 {
                for k in 0..8 {
                    for j in 0..32 {
                        for l in 0..8{
                            print!("{} ", self.bg_tiles[self.tile_map[i][j] as usize].raw_val[k][l]);
                        }
                    }
                    println!("");
                }
            }
            println!("");
        }
    }

    pub fn render(&mut self, window: &mut Window, e: &Event)  {
        const screen_size_x: u32 = 160;
        const screen_size_y: u32 = 144;
        let mut img: RgbaImage = ImageBuffer::new(screen_size_x * 3, screen_size_y * 3);

        let colors = [[255, 255, 255, 255], [169, 169, 169, 255], [128, 128, 128, 255], [0, 0, 0, 255]];

        for y in 0..screen_size_y as usize {
            for x in 0..screen_size_x as usize {
                let color = colors[self.pixel_map[y][x]];
                for i in 0..3 as usize {
                    for j in 0..3 as usize {
                        img.put_pixel((x * 3 + i) as u32, (y * 3 + j) as u32, Rgba { data: color});
                    }
                }
            }
        }

        let factory = window.factory.clone();
        let mut tex_settings = TextureSettings::new();
        tex_settings.set_mag(piston_window::Filter::Nearest);
        let tex = Texture::from_image(&mut window.factory, &img, &tex_settings).unwrap();

        if self.state == 0 {
            window.draw_2d(e, |c, g| {
                clear([0.0; 4], g);
            });
        }
        else {
            window.draw_2d(e, |c, g| {
                clear([1.0; 4], g);

                image(&tex, c.transform, g);
            });
        }
    }

    fn build_tile_map(&mut self, mem: &mut Mmu) {
        for i in 0..32 {
            for j in 0..32 {
                self.tile_map[i][j] = mem.get_mem_u8(self.tile_map_bot + (i * 32) + j);
            }
        }
    }

    pub fn update(&mut self, mem: &mut Mmu) {
        if self.state == 0 {
            if self.get_current_state(mem) == 1 {
                self.state = 1;
                self.initialize(mem);
                self.count = 0;
                self.set_curr_line(mem, 0);
                self.set_mode(mem, 2);
            }
            else {
                self.state = 0;
                self.set_mode(mem, 0b01);
                return;
            }
        }
        else {
            if self.get_current_state(mem) == 0 {
                self.state = 0;
                self.set_mode(mem, 0b01);
                return;
            }
        }

        // Do stuff here that happens once per frame
        if self.count == 0 && self.get_curr_line(mem) == 0 {
            self.build_tile_map(mem);
            self.build_tile_data(mem);
        }

        match self.get_mode(mem) {
            // H-blank
            0b00 => self.h_blank(mem),
            // V-blank
            0b01 => self.v_blank(mem),
            // Reading OAM
            0b10 => self.setup_line(mem),
            // Drawing Line
            0b11 => self.render_line(mem),
            _ => panic!("This should never happen, LCD mode not 0 - 3, it is {}", self.get_mode(mem)),
        }
    }

    // H-blank we just increase our cycles until we are in the next mode
    fn h_blank(&mut self, mem: &mut Mmu) {
        //println!("h-blanking");
        if self.count < 113 {
            self.count += 1;
        }
        else {
            if self.get_curr_line(mem) <= 143 {
                self.count = 0;
                self.set_mode(mem, 2);
            }
            // If we have rendered everything, V-Blank
            else {
                self.count = 0;
                self.set_mode(mem, 1);
                let interrupts = mem.get_mem_u8(0xFF0F);
                mem.set_mem_u8(0xFF0F, interrupts | 0b00000001);
            }
        }
    }

    // V-blank we hblanks for 113 cycles 10 times, increasing the curr_line after each 113
    // cycles
    fn v_blank(&mut self, mem: &mut Mmu) {
        //println!("v-blanking");
        if self.count < 113 {
            self.count += 1;
        }
        else {
            let line = self.get_curr_line(mem);
            if line < 153 {
                self.count = 0;
                self.set_curr_line(mem, line + 1);
            }
            // If we are at the end of V-blank, go to setup_line for the next frame
            else {
                self.count = 0;
                self.set_mode(mem, 2);
                self.set_curr_line(mem, 0);
            }
        }
    }

    // Do stuff here that has to happen on every line render
    fn setup_line(&mut self, mem: &mut Mmu) {
        //println!("setting up line {}", self.get_curr_line(mem));
        // get the palette, etc
        if self.count == 0 {
            self.set_bg_palette(mem);
            self.scy = mem.get_mem_u8(0xFF42);
            self.scx = mem.get_mem_u8(0xFF43);
        }
        if self.count < 19 {
            self.count += 1;
        }
        else {
            self.set_mode(mem, 3);
        }
    }

    fn render_line(&mut self, mem: &mut Mmu) {
        let line_lcd = self.get_curr_line(mem);
        if self.count == 19 {
            let line = line_lcd + self.scy;
            let tile_y = ((line / 8) % 32) as usize;
            let line_in_tile = (line % 8) as usize;
            for i in 0..160 {
                let x = i + self.scx;
                let tile_x = ((x/8) % 32) as usize;
                let x_in_tile = (x % 8) as usize;
                let tile_num = self.tile_map[tile_y][tile_x] as usize;
                let palette_num = self.bg_tiles[tile_num].raw_val[line_in_tile][x_in_tile] as usize;
                let pixel_val = self.bg_palette[palette_num];
                self.pixel_map[line_lcd as usize][x as usize] = pixel_val;
            }
        }
        if self.count < 62 {
            self.count += 1;
        }
        else {
            self.set_mode(mem, 0);
            self.set_curr_line(mem, line_lcd + 1);
        }
    }

    pub fn initialize(&mut self, mem: &mut Mmu) {
        println!("initializing screen");
        let ff40 = mem.get_mem_u8(0xFF40);
        if (ff40 & 0b00010000) >> 4 == 0 {
            self.background_data_bot = 0x8800;
            self.background_data_top = 0x97FF;
        }
        else {
            self.background_data_bot = 0x8000;
            self.background_data_top = 0x8FFF;
        }

        if (ff40 & 0b00001000) >> 3 == 0 {
            self.tile_map_bot = 0x9800;
            self.tile_map_top = 0x9BFF;
        }
        else {
            self.tile_map_bot = 0x9C00;
            self.tile_map_top = 0x9FFF;
        }

        self.build_tile_data(mem);
    }

    pub fn set_bg_palette(&mut self, mem: &mut Mmu) {
        let ff47 = mem.get_mem_u8(0xFF47);
        self.bg_palette[0] = (ff47 & 0b00000011) as usize;
        self.bg_palette[1] = ((ff47 & 0b00001100) >> 2) as usize;
        self.bg_palette[2] = ((ff47 & 0b00110000) >> 4) as usize;
        self.bg_palette[3] = ((ff47 & 0b11000000) >> 6) as usize;
    }

    fn build_tile_data(&mut self, mem: &mut Mmu) {
        self.bg_tiles = Vec::new();
        let mut curr = self.background_data_bot;
        for i in 0..256 {
            let mut new = Tile::new();
            for j in 0..8 {
                let left = mem.get_mem_u8(self.background_data_bot + (i*16) + (j * 2));
                let right = mem.get_mem_u8(self.background_data_bot + (i*16) + 1 + (j * 2));
                new.raw_val[j as usize][0] = ((right & 0b10000000) >> 6) + ((left & 0b10000000) >> 7);
                new.raw_val[j as usize][1] = ((right & 0b01000000) >> 5) + ((left & 0b01000000) >> 6);
                new.raw_val[j as usize][2] = ((right & 0b00100000) >> 4) + ((left & 0b00100000) >> 5);
                new.raw_val[j as usize][3] = ((right & 0b00010000) >> 3) + ((left & 0b00010000) >> 4);
                new.raw_val[j as usize][4] = ((right & 0b00001000) >> 2) + ((left & 0b00001000) >> 3);
                new.raw_val[j as usize][5] = ((right & 0b00000100) >> 1) + ((left & 0b00000100) >> 2);
                new.raw_val[j as usize][6] = (right & 0b00000010) + ((left & 0b00000010) >> 1);
                new.raw_val[j as usize][7] = ((right & 0b00000001) << 1) + (left & 0b00000001);
            }
            //new.display_ascii();
            self.bg_tiles.push(new);
        }
    }

    #[inline(always)]
    fn get_mode(&self, mem: &mut Mmu) -> u8 {
        mem.get_mem_u8(0xFF41) & 0b00000011
    }

    #[inline(always)]
    // Mode must be 0 - 3, other values will break the game
    fn set_mode(&self, mem: &mut Mmu, mode: u8) {
        let tmp = mem.get_mem_u8(0xFF41) & 0b11111100;
        mem.set_mem_u8(0xFF41, tmp + mode);
    }

    #[inline(always)]
    fn get_curr_line(&self, mem: &mut Mmu) -> u8 {
        mem.get_mem_u8(0xFF44)
    }

    #[inline(always)]
    fn set_curr_line(&self, mem: &mut Mmu, line_num: u8) {
        mem.set_mem_u8(0xFF44, line_num);
    }

    #[inline(always)]
    fn get_current_state(&self, mem: &mut Mmu) -> u8 {
        (mem.get_mem_u8(0xFF40) & 0b10000000) >> 7
    }

}