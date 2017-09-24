
pub struct Ram {
    mem: [u8; 4096],
}


impl Ram {
    pub fn new() -> Ram {
        Ram { mem: [0; 4096] }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {}

    pub fn read_byte(&mut self, address: u16, value: u8) {}
}
