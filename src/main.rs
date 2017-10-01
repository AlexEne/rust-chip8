extern crate minifb;

use minifb::{KeyRepeat, Key, WindowOptions, Window};
use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use display::Display;

mod ram;
mod cpu;
mod chip8;
mod display;
mod keyboard;
mod bus;

fn get_chip8_keycode_for(key: Option<Key>) -> Option<u8> {
    match key {
        Some(Key::Key1) => Some(0x1),
        Some(Key::Key2) => Some(0x2),
        Some(Key::Key3) => Some(0x3),
        Some(Key::Key4) => Some(0xC),

        Some(Key::Q) => Some(0x4),
        Some(Key::W) => Some(0x5),
        Some(Key::E) => Some(0x6),
        Some(Key::R) => Some(0xD),

        Some(Key::A) => Some(0x7),
        Some(Key::S) => Some(0x8),
        Some(Key::D) => Some(0x9),
        Some(Key::F) => Some(0xE),

        Some(Key::Z) => Some(0xA),
        Some(Key::X) => Some(0x0),
        Some(Key::C) => Some(0xB),
        Some(Key::V) => Some(0xF),
        _ => None,
    }
}

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let WIDTH = 640;
    let HEIGHT = 320;

    //ARGB buffer
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Rust Chip8 emulator",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No);
        let key = match keys_pressed {
            Some(keys) => {
                if keys.len() > 0 {
                    Some(keys[0])
                } else {
                    None
                }
            }
            None => None
        };

        let chip8_key = get_chip8_keycode_for(key);
        chip8.set_key_pressed(chip8_key);

        chip8.run_instruction();        
        let chip8_buffer = chip8.get_display_buffer();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = Display::get_index_from_coords(x/10, y/10);
                let pixel = chip8_buffer[index];
                let color_pixel = match pixel {
                     0 => 0x0,
                     1 => 0xffffff,
                     _ => unreachable!()
                };
                buffer[y*WIDTH + x] = color_pixel;
            }
        }
        //std::thread::sleep_ms(16);

        window.update_with_buffer(&buffer).unwrap();
    }    
}
