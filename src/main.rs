extern crate minifb;
extern crate rand;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use display::Display;
use std::time::{Duration, Instant};
use std::env;

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
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        0 | 1 => "data/TETRIS",
        _ => args.get(1).unwrap(),
    };
    let mut file = File::open(file_name).unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");

    let width = 640;
    let height = 320;

    //ARGB buffer
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Rust Chip8 emulator",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Window creation failed: {:?}", e);
    });

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
        let key = match keys_pressed {
            Some(keys) => if !keys.is_empty() {
                Some(keys[0])
            } else {
                None
            },
            None => None,
        };

        let chip8_key = get_chip8_keycode_for(key);
        if chip8_key.is_some()
            || Instant::now() - last_key_update_time >= Duration::from_millis(200)
        {
            last_key_update_time = Instant::now();
            chip8.set_key_pressed(chip8_key);
        }

        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip8.run_instruction();
            last_instruction_run_time = Instant::now();
        }

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            let chip8_buffer = chip8.get_display_buffer();

            for y in 0..height {
                let y_coord = y / 10;
                let offset = y * width;
                for x in 0..width {
                    let index = Display::get_index_from_coords(x / 10, y_coord);
                    let pixel = chip8_buffer[index];
                    let color_pixel = match pixel {
                        0 => 0x0,
                        1 => 0xffffff,
                        _ => unreachable!(),
                    };
                    buffer[offset + x] = color_pixel;
                }
            }

            window.update_with_buffer(&buffer);
            last_display_time = Instant::now();
        }
    }
}
