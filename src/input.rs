pub struct Input {
    keys: [bool; 16],
}

impl Input {
    pub fn key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }
    pub fn set_key_pressed(&mut self, key: u8, pressed: bool) {
        self.keys[key as usize] = pressed
    }
}
