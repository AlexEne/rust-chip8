
const WIDTH: usize = 64;
const HEIGHT: usize = 64;

pub struct Display {
    screen: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut flipped = false;
        let mut coord_x = x as usize;
        let coord_y = y as usize;
        let mut b = byte;

        for _ in 0..8 {
            match (b & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[coord_y][coord_x] == 1 {
                        flipped = true;
                    }
                    self.screen[coord_y][coord_x] = 0;
                }
                1 => self.screen[coord_y][coord_x] = 1,
                _ => unreachable!(),
            };
            coord_x += 1;
            b = b << 1;
        }
        flipped
    }

    pub fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.screen[y][x] = 0;
            }
        }
    }

    pub fn present(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.screen[y][x] == 0 {
                    print!("_");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
    }
}
