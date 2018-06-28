use piston::input::Button;
use piston::input::Key;

use corroboy::mmu::Mmu;
use corroboy::joypad::Joypad;

#[test]
pub fn test_joypad() {
    let mut joypad = Joypad::new();
    let mut mem = Mmu::new(None);

    joypad.press_input(Button::Keyboard(Key::Up));
    joypad.press_input(Button::Keyboard(Key::Down));

    mem.set_mem_u8(0xFF00, 0b00010000);
    joypad.update(&mut mem);
    assert_eq!(mem.get_mem_u8(0xFF00), 0b11011111);

    mem.set_mem_u8(0xFF00, 0b00100000);
    joypad.update(&mut mem);
    assert_eq!(mem.get_mem_u8(0xFF00), 0b11100011);

    joypad.release_input(Button::Keyboard(Key::Up));
    joypad.release_input(Button::Keyboard(Key::Down));

    mem.set_mem_u8(0xFF00, 0b00100000);
    joypad.update(&mut mem);
    assert_eq!(mem.get_mem_u8(0xFF00), 0b11101111);

    joypad.press_input(Button::Keyboard(Key::Z));
    joypad.press_input(Button::Keyboard(Key::X));

    mem.set_mem_u8(0xFF00, 0b00010000);
    joypad.update(&mut mem);
    assert_eq!(mem.get_mem_u8(0xFF00), 0b11011100);

    joypad.release_input(Button::Keyboard(Key::Z));
    joypad.release_input(Button::Keyboard(Key::X));

    mem.set_mem_u8(0xFF00, 0b00010000);
    joypad.update(&mut mem);
    assert_eq!(mem.get_mem_u8(0xFF00), 0b11011111);
}
