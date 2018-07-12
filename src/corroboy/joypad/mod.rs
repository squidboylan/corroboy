// Copyright (c) 2018 Caleb Boylan

use corroboy::mmu::Mmu;
use piston::input::Button;
use piston::input::Key;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct Joypad {
    left: bool,
    up: bool,
    right: bool,
    down: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Default::default()
    }

    /// Set the flag for whatever button is pressed
    pub fn press_input(&mut self, but: Button) {
        match but {
            Button::Keyboard(Key::Up) => {
                self.up = true;
            }
            Button::Keyboard(Key::Down) => {
                self.down = true;
            }
            Button::Keyboard(Key::Left) => {
                self.left = true;
            }
            Button::Keyboard(Key::Right) => {
                self.right = true;
            }
            Button::Keyboard(Key::Z) => {
                self.a = true;
            }
            Button::Keyboard(Key::X) => {
                self.b = true;
            }
            Button::Keyboard(Key::RShift) => {
                self.select = true;
            }
            Button::Keyboard(Key::Return) => {
                self.start = true;
            }
            _ => {}
        }
    }

    /// Unset the flag for whatever button is pressed
    pub fn release_input(&mut self, but: Button) {
        match but {
            Button::Keyboard(Key::Up) => {
                self.up = false;
            }
            Button::Keyboard(Key::Down) => {
                self.down = false;
            }
            Button::Keyboard(Key::Left) => {
                self.left = false;
            }
            Button::Keyboard(Key::Right) => {
                self.right = false;
            }
            Button::Keyboard(Key::Z) => {
                self.a = false;
            }
            Button::Keyboard(Key::X) => {
                self.b = false;
            }
            Button::Keyboard(Key::RShift) => {
                self.select = false;
            }
            Button::Keyboard(Key::Return) => {
                self.start = false;
            }
            _ => {}
        }
    }

    /// Use the flags of what buttons are pressed and what the FF00 register
    /// contains and to update the FF00 register
    pub fn update(&mut self, mem: &mut Mmu) {
        let ff00 = mem.get_io_register(0xFF00);
        let bit_4 = ff00 & 0b00010000;
        let bit_5 = ff00 & 0b00100000;

        let mut new_ff00 = 0b11001111 + bit_4 + bit_5;

        if bit_4 == 0 && bit_5 == 0 {
            if self.down == true || self.start == true {
                new_ff00 -= 0b00001000;
            }
            if self.up == true || self.select == true {
                new_ff00 -= 0b00000100;
            }
            if self.left == true || self.b == true {
                new_ff00 -= 0b00000010;
            }
            if self.right == true || self.a == true {
                new_ff00 -= 0b00000001;
            }
        } else if bit_5 == 0 {
            if self.start == true {
                new_ff00 -= 0b00001000;
            }
            if self.select == true {
                new_ff00 -= 0b00000100;
            }
            if self.b == true {
                new_ff00 -= 0b00000010;
            }
            if self.a == true {
                new_ff00 -= 0b00000001;
            }
        } else if bit_4 == 0 {
            if self.down == true {
                new_ff00 -= 0b00001000;
            }
            if self.up == true {
                new_ff00 -= 0b00000100;
            }
            if self.left == true {
                new_ff00 -= 0b00000010;
            }
            if self.right == true {
                new_ff00 -= 0b00000001;
            }
        }

        mem.set_io_register(0xFF00, new_ff00);
    }
}
