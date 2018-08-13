// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use gfx_device_gl;
use image::*;
use piston_window;
use piston_window::PistonWindow as Window;
use piston_window::TextureSettings;
use piston_window::*;
use sdl2_window::Sdl2Window;
use std::collections::HashMap;

use corroboy::mmu::Mmu;

//#[derive(Clone, Copy)]
pub struct Sprite {
    // Pattern num for the sprite
    pub pattern: u8,
    pub x: u8,
    pub y: u8,

    // Flags
    pub priority: u8,
    pub x_flip: u8,
    pub y_flip: u8,
    pub palette_num: u8,

    pub dirty: bool,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            pattern: 0,
            x: 0,
            y: 0,
            priority: 0,
            x_flip: 0,
            y_flip: 0,
            palette_num: 0,
            dirty: true,
        }
    }
}

pub struct SpritePattern {
    // color nums that represent the sprite, this + palette data each line represent how the sprite is
    // drawn
    pub raw_val: [[u8; 8]; 16],
    pub dirty: bool,
    pub textures: HashMap<[usize; 4], Texture<gfx_device_gl::Resources>>,
}

impl SpritePattern {
    pub fn new() -> SpritePattern {
        SpritePattern {
            raw_val: [[0; 8]; 16],
            dirty: true,
            textures: HashMap::with_capacity(256),
        }
    }

    /// Return the texture for the sprite pattern and palette
    pub fn get_texture(
        &self,
        palettes: &[[usize; 4]; 2],
        palette_num: &u8,
    ) -> &Texture<gfx_device_gl::Resources> {
        return self.textures.get(&palettes[*palette_num as usize]).unwrap();
    }

    /// Generate the texture for this sprite pattern with the palette passed in as an argument
    /// then store the texture in a hashmap for quick access later.
    pub fn generate_texture(
        &mut self,
        window: &mut Window<Sdl2Window>,
        palettes: &[[usize; 4]; 2],
        palette_num: &u8,
        height: &u8,
    ) {
        if self.textures.contains_key(&palettes[*palette_num as usize]) == false {
            let colors = [
                [255, 255, 255, 255],
                [169, 169, 169, 255],
                [128, 128, 128, 255],
                [0, 0, 0, 255],
            ];

            let mut img: RgbaImage = ImageBuffer::new(8, *height as u32);

            let mut x = 0;
            let mut y = 0;
            while y < *height as usize {
                while x < 8 as usize {
                    if self.raw_val[y][x] == 0 {
                        img.put_pixel(x as u32, y as u32, Rgba { data: [0, 0, 0, 0] });
                    } else {
                        let color = colors
                            [palettes[*palette_num as usize][self.raw_val[y][x] as usize] as usize];
                        img.put_pixel(x as u32, y as u32, Rgba { data: color });
                    }
                    x += 1;
                }
                x = 0;
                y += 1;
            }

            //let factory = window.factory.clone();
            let mut tex_settings = TextureSettings::new();
            tex_settings.set_mag(piston_window::Filter::Nearest);
            self.textures.insert(
                palettes[*palette_num as usize],
                Texture::from_image(&mut window.factory, &img, &tex_settings).unwrap(),
            );
        }
    }
}

pub struct SpriteManager {
    // Info about sprite rendering
    pub sprite_height: u8,
    pub sprites_enabled: bool,

    pub sprite_patterns: Vec<SpritePattern>,
    pub sprites: Vec<Sprite>,

    pub sprite_pattern_bot: usize,

    // colors 0 - 3
    pub sprite_palettes: [[usize; 4]; 2],
}

impl SpriteManager {
    pub fn new() -> SpriteManager {
        let mut patterns = Vec::with_capacity(256);
        for _i in 0..256 {
            patterns.push(SpritePattern::new());
        }

        let mut sprites = Vec::with_capacity(40);
        for _i in 0..40 {
            sprites.push(Sprite::new());
        }

        SpriteManager {
            sprite_height: 0,
            sprites_enabled: false,
            sprite_patterns: patterns,
            sprites: sprites,
            sprite_pattern_bot: 0x8000,
            sprite_palettes: [[0; 4]; 2],
        }
    }

    /// Loop through each sprite and generate the SpritePattern's texture that corresponds to it
    pub fn generate_sprite_textures(&mut self, window: &mut Window<Sdl2Window>) {
        const SCREEN_SIZE_X: u32 = 160;
        const SCREEN_SIZE_Y: u32 = 144;

        for i in self.sprites.iter() {
            if i.x > 0
                && i.y > 0
                && (i.x as u32) < SCREEN_SIZE_X + 8
                && (i.y as u32) < SCREEN_SIZE_Y + 16
            {
                if self.sprite_height == 16 {
                    self.sprite_patterns[(i.pattern >> 1) as usize].generate_texture(
                        window,
                        &self.sprite_palettes,
                        &i.palette_num,
                        &self.sprite_height,
                    );
                }
                self.sprite_patterns[i.pattern as usize].generate_texture(
                    window,
                    &self.sprite_palettes,
                    &i.palette_num,
                    &self.sprite_height,
                );
            }
        }
    }

    /// Return the texture for the sprite number passed in as an argument
    pub fn get_texture(&self, num: usize) -> &Texture<gfx_device_gl::Resources> {
        let i = &self.sprites[num];
        if self.sprite_height == 16 {
            return self.sprite_patterns[(i.pattern >> 1) as usize]
                .get_texture(&self.sprite_palettes, &i.palette_num);
        }
        return self.sprite_patterns[i.pattern as usize]
            .get_texture(&self.sprite_palettes, &i.palette_num);
    }

    /// Update the sprite objects for each sprite
    pub fn build_sprites(&mut self, mem: &mut Mmu) {
        let sprite_bot = 0xFE00;
        for i in 0..40 {
            let y = mem.get_mem_u8(sprite_bot + (i * 4));
            let x = mem.get_mem_u8(sprite_bot + (i * 4) + 1);
            let pattern = mem.get_mem_u8(sprite_bot + (i * 4) + 2);
            let flags = mem.get_mem_u8(sprite_bot + (i * 4) + 3);
            self.sprites[i].x = x;
            self.sprites[i].y = y;
            self.sprites[i].pattern = pattern;
            self.sprites[i].priority = (flags & 0b10000000) >> 7;
            self.sprites[i].y_flip = (flags & 0b01000000) >> 6;
            self.sprites[i].x_flip = (flags & 0b00100000) >> 5;
            self.sprites[i].palette_num = (flags & 0b00010000) >> 4;
        }
    }

    /// Update the sprite palettes
    pub fn set_sprite_palettes(&mut self, mem: &mut Mmu) {
        let ff48 = mem.get_io_register(0xFF48);
        let ff49 = mem.get_io_register(0xFF49);

        self.sprite_palettes[0][0] = (ff48 & 0b00000011) as usize;
        self.sprite_palettes[0][1] = ((ff48 & 0b00001100) >> 2) as usize;
        self.sprite_palettes[0][2] = ((ff48 & 0b00110000) >> 4) as usize;
        self.sprite_palettes[0][3] = ((ff48 & 0b11000000) >> 6) as usize;

        self.sprite_palettes[1][0] = (ff49 & 0b00000011) as usize;
        self.sprite_palettes[1][1] = ((ff49 & 0b00001100) >> 2) as usize;
        self.sprite_palettes[1][2] = ((ff49 & 0b00110000) >> 4) as usize;
        self.sprite_palettes[1][3] = ((ff49 & 0b11000000) >> 6) as usize;
    }

    /// Update the SpritePattern objects
    pub fn build_pattern_data(&mut self, mem: &mut Mmu) {
        if self.sprite_height == 8 {
            for i in 0..256 {
                for j in 0..8 {
                    self.sprite_patterns[i].textures.clear();
                    let left = mem.get_vram(self.sprite_pattern_bot + (i * 16) + (j * 2));
                    let right = mem.get_vram(self.sprite_pattern_bot + (i * 16) + 1 + (j * 2));
                    self.sprite_patterns[i].raw_val[j as usize][0] =
                        ((right & 0b10000000) >> 6) + ((left & 0b10000000) >> 7);
                    self.sprite_patterns[i].raw_val[j as usize][1] =
                        ((right & 0b01000000) >> 5) + ((left & 0b01000000) >> 6);
                    self.sprite_patterns[i].raw_val[j as usize][2] =
                        ((right & 0b00100000) >> 4) + ((left & 0b00100000) >> 5);
                    self.sprite_patterns[i].raw_val[j as usize][3] =
                        ((right & 0b00010000) >> 3) + ((left & 0b00010000) >> 4);
                    self.sprite_patterns[i].raw_val[j as usize][4] =
                        ((right & 0b00001000) >> 2) + ((left & 0b00001000) >> 3);
                    self.sprite_patterns[i].raw_val[j as usize][5] =
                        ((right & 0b00000100) >> 1) + ((left & 0b00000100) >> 2);
                    self.sprite_patterns[i].raw_val[j as usize][6] =
                        (right & 0b00000010) + ((left & 0b00000010) >> 1);
                    self.sprite_patterns[i].raw_val[j as usize][7] =
                        ((right & 0b00000001) << 1) + (left & 0b00000001);
                }
            }
        } else {
            for i in 0..128 {
                for j in 0..16 {
                    self.sprite_patterns[i].textures.clear();
                    let left = mem.get_vram(self.sprite_pattern_bot + (i * 32) + (j * 2));
                    let right = mem.get_vram(self.sprite_pattern_bot + (i * 32) + 1 + (j * 2));
                    self.sprite_patterns[i].raw_val[j as usize][0] =
                        ((right & 0b10000000) >> 6) + ((left & 0b10000000) >> 7);
                    self.sprite_patterns[i].raw_val[j as usize][1] =
                        ((right & 0b01000000) >> 5) + ((left & 0b01000000) >> 6);
                    self.sprite_patterns[i].raw_val[j as usize][2] =
                        ((right & 0b00100000) >> 4) + ((left & 0b00100000) >> 5);
                    self.sprite_patterns[i].raw_val[j as usize][3] =
                        ((right & 0b00010000) >> 3) + ((left & 0b00010000) >> 4);
                    self.sprite_patterns[i].raw_val[j as usize][4] =
                        ((right & 0b00001000) >> 2) + ((left & 0b00001000) >> 3);
                    self.sprite_patterns[i].raw_val[j as usize][5] =
                        ((right & 0b00000100) >> 1) + ((left & 0b00000100) >> 2);
                    self.sprite_patterns[i].raw_val[j as usize][6] =
                        (right & 0b00000010) + ((left & 0b00000010) >> 1);
                    self.sprite_patterns[i].raw_val[j as usize][7] =
                        ((right & 0b00000001) << 1) + (left & 0b00000001);
                }
            }
        }
    }
}
