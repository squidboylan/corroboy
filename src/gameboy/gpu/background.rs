use piston_window::PistonWindow as Window;
use piston_window::Texture;
use piston::input::*;
use piston_window;
use piston_window::TextureSettings;
use graphics::*;
use image::*;
use gfx_device_gl;

use gameboy::mmu::Mmu;

pub struct Tile {
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

    #[allow(dead_code)]
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

pub struct Background {
    pub background_data_bot: usize,
    pub background_data_top: usize,

    pub bg_tiles: Vec<Tile>,

    pub tile_map_bot: usize,
    pub tile_map_top: usize,
    pub tile_map: [[u8; 32]; 32],

    // colors 0 - 3
    pub bg_palette: [usize; 4],

    pub tex: Texture<gfx_device_gl::Resources>,

    // Color representation of each pixel
    // 0 - white
    // 1 - light gray
    // 2 - dark gray
    // 3 - black
    pub pixel_map: [[usize; 160]; 144],
    pub last_pixel_map: [[usize; 160]; 144],
}

impl Background {
    pub fn new(window: &mut Window) -> Background {
        let mut factory = window.factory.clone();

        Background {
            background_data_bot: 0,
            background_data_top: 0,
            bg_tiles: Vec::new(),
            tile_map_bot: 0,
            tile_map_top: 0,
            tile_map: [[0; 32]; 32],
            bg_palette: [0; 4],
            tex: Texture::empty(&mut factory).unwrap(),
            pixel_map: [[0; 160]; 144],
            last_pixel_map: [[0; 160]; 144],
        }
    }

    pub fn initialize(&mut self, mem: &mut Mmu) {
        let ff40 = mem.get_mem_u8(0xFF40);
        if (ff40 & 0b00010000) >> 4 == 0 {
            self.background_data_bot = 0x8800;
            self.background_data_top = 0x97FF;
            println!("background_data_bot: {:x}", self.background_data_bot);
        }
        else {
            self.background_data_bot = 0x8000;
            self.background_data_top = 0x8FFF;
            println!("background_data_bot: {:x}", self.background_data_bot);
        }

        if ff40 & 0b00001000 == 0 {
            self.tile_map_bot = 0x9800;
            self.tile_map_top = 0x9BFF;
        }
        else {
            self.tile_map_bot = 0x9C00;
            self.tile_map_top = 0x9FFF;
        }

        self.build_tile_data(mem);
    }

    #[allow(dead_code)]
    fn print_tile_map(&self) {
        for i in 0..32 {
            for j in 0..32 {
                print!("{} ", self.tile_map[i][j]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
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

    pub fn generate_tex(&mut self, window: &mut Window) {
        const SCREEN_SIZE_X: u32 = 160;
        const SCREEN_SIZE_Y: u32 = 144;
        let mut new_map = false;

        // Check if the new pixel_map is the same as last frame
        let mut x = 0;
        let mut y = 0;
        while y < SCREEN_SIZE_Y as usize && new_map == false {
            while x < SCREEN_SIZE_X as usize && new_map == false {
                if self.pixel_map[y][x] != self.last_pixel_map[y][x] {
                    new_map = true;
                }
                x += 1;
            }
            x = 0;
            y += 1;
        }

        if new_map == true {
            let mut img: RgbaImage = ImageBuffer::new(SCREEN_SIZE_X * 3, SCREEN_SIZE_Y * 3);

            let colors = [[255, 255, 255, 255], [169, 169, 169, 255], [128, 128, 128, 255], [0, 0, 0, 255]];

            let mut x = 0;
            let mut y = 0;
            while y < SCREEN_SIZE_Y as usize {
                while x < SCREEN_SIZE_X as usize {
                    let color = colors[self.pixel_map[y][x]];
                    self.last_pixel_map[y][x] = self.pixel_map[y][x];
                    for i in 0..3 as usize {
                        for j in 0..3 as usize {
                            img.put_pixel((x * 3 + i) as u32, (y * 3 + j) as u32, Rgba { data: color});
                        }
                    }
                    x += 1;
                }
                x = 0;
                y += 1;
            }

            let mut tex_settings = TextureSettings::new();
            tex_settings.set_mag(piston_window::Filter::Nearest);
            self.tex = Texture::from_image(&mut window.factory, &img, &tex_settings).unwrap();

        }

    }

    pub fn build_tile_map(&mut self, mem: &mut Mmu) {
        for i in 0..32 {
            for j in 0..32 {
                self.tile_map[i][j] = mem.get_mem_u8(self.tile_map_bot + (i * 32) + j);
            }
        }
    }

    pub fn set_bg_palette(&mut self, mem: &mut Mmu) {
        let ff47 = mem.get_mem_u8(0xFF47);
        self.bg_palette[0] = (ff47 & 0b00000011) as usize;
        self.bg_palette[1] = ((ff47 & 0b00001100) >> 2) as usize;
        self.bg_palette[2] = ((ff47 & 0b00110000) >> 4) as usize;
        self.bg_palette[3] = ((ff47 & 0b11000000) >> 6) as usize;
    }

    pub fn build_tile_data(&mut self, mem: &mut Mmu) {
        if self.background_data_bot == 0x8000 {
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
        else {
            self.bg_tiles = Vec::new();
            let mut curr = 0x9000;
            for i in 0..128 {
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

            curr = self.background_data_bot;
            for i in 0..128 {
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
    }


}
