use crate::DISPLAY_HEIGHT;
use crate::DISPLAY_WIDTH;

const VRAM_SIZE: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;

pub struct Display {
  pub vram: [bool; VRAM_SIZE],
}

impl Display {
  pub fn new() -> Display {
    Display {
      vram: [false; VRAM_SIZE],
    }
  }

  pub fn cls(&mut self) {
    self.vram = [false; VRAM_SIZE];
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, is_on: bool) {
    self.vram[x + y * DISPLAY_WIDTH] = is_on;
  }

  pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
    self.vram[x + y * DISPLAY_WIDTH] == true
  }

  pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
    let rows = sprite.len();
    let mut collision = false;
    for j in 0..rows {
      let row = sprite[j];
      for i in 0..8 {
        let new_val = row >> (7 - i) & 0x01;
        if new_val == 1 {
          let xi = (x + i) % DISPLAY_WIDTH;
          let yj = (y + j) % DISPLAY_HEIGHT;
          let old_val = self.get_pixel(xi, yj);
          if old_val {
            collision = true;
          }
          let is_on = (new_val == 1) ^ old_val;
          self.set_pixel(xi, yj, is_on);
        }
      }
    }

    collision
  }

  pub fn copy_vram_to_vec(&self) -> Vec<u8> {
    self
      .vram
      .into_iter()
      .map(|&x| if x { 1 } else { 0 })
      .collect()
  }
}
