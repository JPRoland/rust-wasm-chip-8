pub struct Keypad {
  keys: [bool; 16],
}

impl Keypad {
  pub fn new() -> Keypad {
    Keypad { keys: [false; 16] }
  }

  pub fn key_down(&mut self, idx: u8) {
    self.keys[idx as usize] = true;
  }

  pub fn key_up(&mut self, idx: u8) {
    self.keys[idx as usize] = false;
  }

  pub fn is_key_pressed(&self, idx: u8) -> bool {
    self.keys[idx as usize]
  }
}
