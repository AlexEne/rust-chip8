
pub struct Keyboard {

}


impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard{}
    }
    
    //Todo implement proper key handling
    pub fn key_pressed(&self, key_code: u8) -> bool {
        if key_code == 4 {
            false
        } else {
            true
        }
    }
}