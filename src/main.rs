extern crate minifb;

use minifb::{Key, WindowOptions, Window};
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

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
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


        window.update_with_buffer(&buffer).unwrap();
    }    
}
