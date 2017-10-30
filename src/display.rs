
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut erased = false;
        let mut coord_x = x as usize;
        let mut coord_y = y as usize;
        let mut b = byte;
        
        for _ in 0..8 {
            coord_x %= WIDTH;
            coord_y %= HEIGHT;
            let index = Display::get_index_from_coords(coord_x, coord_y);
            let bit = (b & 0b1000_0000) >> 7;
            let prev_value = self.screen[index];
            self.screen[index] ^= bit;

            if prev_value == 1 && self.screen[index] == 0 {
                erased = true;
            }

            coord_x += 1;
            b <<= 1;
        }

        erased
    }

    pub fn clear(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        &self.screen
    }
}
