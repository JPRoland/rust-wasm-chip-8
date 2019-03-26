use wasm_bindgen::prelude::*;

use crate::display::Display;
use crate::font::FONT_SET;
use crate::keypad::Keypad;

use crate::MEM_SIZE;

#[wasm_bindgen]
pub struct CPU {
  // Index register
  i: u16,
  // Program counter
  pc: u16,
  // registers
  v: [u8; 16],
  stack: [u16; 16],
  // Stack pointer
  sp: u8,
  // Delay timer
  dt: u8,
  // Sound timer
  st: u8,
  memory: [u8; MEM_SIZE],
  display: Display,
  keypad: Keypad,
}

#[wasm_bindgen]
impl CPU {
  pub fn new() -> CPU {
    let mut memory = [0; MEM_SIZE];

    for i in 0..FONT_SET.len() {
      memory[i] = FONT_SET[i];
    }

    CPU {
      i: 0,
      pc: 0x200,
      memory: memory,
      display: Display::new(),
      keypad: Keypad::new(),
      v: [0; 16],
      stack: [0; 16],
      sp: 0,
      dt: 0,
      st: 0,
    }
  }
}

struct Opcode {
  nibbles: (u8, u8, u8, u8),
  x: usize,
  y: usize,
  n: usize,
  nnn: usize,
  kk: u8,
}

impl Opcode {
  fn new(opcode: u16) -> Opcode {
    let nibbles = (
      ((opcode & 0xF000) >> 12) as u8,
      ((opcode & 0x0F00) >> 8) as u8,
      ((opcode & 0x00F0) >> 4) as u8,
      (opcode & 0x000F) as u8,
    );

    let nnn = (opcode & 0x0FFF) as usize;
    let kk = (opcode & 0x00FF) as u8;

    Opcode {
      nibbles,
      x: nibbles.1 as usize,
      y: nibbles.2 as usize,
      n: nibbles.3 as usize,
      nnn,
      kk,
    }
  }
}
