use rand_os::rand_core::RngCore;
use rand_os::OsRng;
use wasm_bindgen::prelude::*;

use crate::display::Display;
use crate::font::FONT_SET;
use crate::keypad::Keypad;

use crate::MEM_SIZE;

#[wasm_bindgen]
pub struct CPU {
  // Index register
  i: usize,
  // Program counter
  pc: usize,
  // registers
  v: [u8; 16],
  stack: [usize; 16],
  // Stack pointer
  sp: usize,
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

  pub fn reset(&mut self) {
    let mut memory = [0; MEM_SIZE];

    memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);

    self.i = 0;
    self.pc = 0x200;
    self.memory = memory;
    self.display.cls();
    self.v = [0; 16];
    self.stack = [0; 16];
    self.sp = 0;
    self.dt = 0;
    self.st = 0;
  }

  pub fn load(&mut self, data: &[u8]) {
    self.memory[0x200..0x200 + data.len()].copy_from_slice(&data);
  }

  pub fn key_down(&mut self, key: u8) {
    self.keypad.key_down(key);
  }

  pub fn key_up(&mut self, key: u8) {
    self.keypad.key_up(key);
  }

  pub fn emulate_cycle(&mut self) -> Output {
    let pc = self.pc;
    let opcode = CPU::get_opcode(self.memory[pc], self.memory[pc + 1]);
    let parts = Opcode::new(opcode);
    self.decrement_timers();
    self.run_instruction(&parts);

    Output::new(self.display.copy_vram_to_vec(), self.st > 0)
  }

  fn run_instruction(&mut self, opcode: &Opcode) {
    let program_count = match opcode.nibbles {
      (0, 0, 0xE, 0x0) => self.op_00E0(),
      (0, 0, 0xE, 0xE) => self.op_00EE(),
      (0x1, _, _, _) => self.op_1NNN(opcode.nnn),
      (0x2, _, _, _) => self.op_2NNN(opcode.nnn),
      (0x3, _, _, _) => self.op_3XKK(opcode.x, opcode.kk),
      (0x4, _, _, _) => self.op_4XKK(opcode.x, opcode.kk),
      (0x5, _, _, _) => self.op_5XY0(opcode.x, opcode.y),
      (0x6, _, _, _) => self.op_6XKK(opcode.x, opcode.kk),
      (0x7, _, _, _) => self.op_7XKK(opcode.x, opcode.kk),
      (0x8, _, _, 0x0) => self.op_8XY0(opcode.x, opcode.y),
      (0x8, _, _, 0x1) => self.op_8XY1(opcode.x, opcode.y),
      (0x8, _, _, 0x2) => self.op_8XY2(opcode.x, opcode.y),
      (0x8, _, _, 0x3) => self.op_8XY3(opcode.x, opcode.y),
      (0x8, _, _, 0x4) => self.op_8XY4(opcode.x, opcode.y),
      (0x8, _, _, 0x5) => self.op_8XY5(opcode.x, opcode.y),
      (0x8, _, _, 0x6) => self.op_8XY6(opcode.x),
      (0x8, _, _, 0x7) => self.op_8XY7(opcode.x, opcode.y),
      (0x8, _, _, 0xE) => self.op_8XYE(opcode.x),
      (0x9, _, _, _) => self.op_9XY0(opcode.x, opcode.y),
      (0xA, _, _, _) => self.op_ANNN(opcode.nnn),
      (0xB, _, _, _) => self.op_BNNN(opcode.nnn),
      (0xC, _, _, _) => self.op_CXKK(opcode.x, opcode.kk),
      (0xD, _, _, _) => self.op_DXYN(opcode.x, opcode.y, opcode.n),
      (0xE, _, _, 0xE) => self.op_EX9E(opcode.x),
      (0xE, _, _, 0x1) => self.op_EXA1(opcode.x),
      (0xF, _, 0x0, 0x7) => self.op_FX07(opcode.x),
      (0xF, _, 0x0, 0xA) => self.op_FX0A(opcode.x),
      (0xF, _, 0x1, 0x5) => self.op_FX15(opcode.x),
      (0xF, _, 0x1, 0x8) => self.op_FX18(opcode.x),
      (0xF, _, 0x1, 0xE) => self.op_FX1E(opcode.x),
      (0xF, _, 0x2, 0x9) => self.op_FX29(opcode.x),
      (0xF, _, 0x3, 0x3) => self.op_FX33(opcode.x),
      (0xF, _, 0x5, 0x5) => self.op_FX55(opcode.x),
      (0xF, _, 0x6, 0x5) => self.op_FX65(opcode.x),
      _ => ProgramCounter::Next,
    };

    match program_count {
      ProgramCounter::Next => self.pc += 2,
      ProgramCounter::Skip => self.pc += 4,
      ProgramCounter::Jump(address) => self.pc = address,
    }
  }

  // CLS
  fn op_00E0(&mut self) -> ProgramCounter {
    self.display.cls();
    ProgramCounter::Next
  }

  // RET
  fn op_00EE(&mut self) -> ProgramCounter {
    self.sp -= 1;
    ProgramCounter::Jump(self.stack[self.sp])
  }

  // JP addr
  fn op_1NNN(&mut self, nnn: usize) -> ProgramCounter {
    ProgramCounter::Jump(nnn)
  }

  // CALL addr
  fn op_2NNN(&mut self, nnn: usize) -> ProgramCounter {
    self.stack[self.sp] = self.pc + 2;
    self.sp += 1;

    ProgramCounter::Jump(nnn)
  }

  // SE Vx, byte
  fn op_3XKK(&mut self, x: usize, kk: u8) -> ProgramCounter {
    ProgramCounter::skip_if(self.v[x] == kk)
  }

  // SNE Vx, byte
  fn op_4XKK(&mut self, x: usize, kk: u8) -> ProgramCounter {
    ProgramCounter::skip_if(self.v[x] != kk)
  }

  // SE Vx, Vy
  fn op_5XY0(&mut self, x: usize, y: usize) -> ProgramCounter {
    ProgramCounter::skip_if(self.v[x] == self.v[y])
  }

  // LD Vx, byte
  fn op_6XKK(&mut self, x: usize, kk: u8) -> ProgramCounter {
    self.v[x] = kk;
    ProgramCounter::Next
  }

  // ADD Vx, byte
  fn op_7XKK(&mut self, x: usize, kk: u8) -> ProgramCounter {
    self.v[x] += kk;
    ProgramCounter::Next
  }

  // LD Vx, Vy
  fn op_8XY0(&mut self, x: usize, y: usize) -> ProgramCounter {
    self.v[x] = self.v[y];
    ProgramCounter::Next
  }

  // OR Vx, Vy
  fn op_8XY1(&mut self, x: usize, y: usize) -> ProgramCounter {
    self.v[x] |= self.v[y];
    ProgramCounter::Next
  }

  // AND Vx, Vy
  fn op_8XY2(&mut self, x: usize, y: usize) -> ProgramCounter {
    self.v[x] &= self.v[y];
    ProgramCounter::Next
  }

  //XOR Vx, Vy
  fn op_8XY3(&mut self, x: usize, y: usize) -> ProgramCounter {
    self.v[x] ^= self.v[y];
    ProgramCounter::Next
  }

  // ADD Vx, Vy
  fn op_8XY4(&mut self, x: usize, y: usize) -> ProgramCounter {
    let result = self.v[x] as u16 + self.v[y] as u16;
    self.v[x] = result as u8;
    self.v[0xF] = if result > 0xFF { 1 } else { 0 };

    ProgramCounter::Next
  }

  // SUB Vx, Vy
  fn op_8XY5(&mut self, x: usize, y: usize) -> ProgramCounter {
    let result = self.v[x] as i8 - self.v[y] as i8;
    self.v[x] = result as u8;
    self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };

    ProgramCounter::Next
  }

  // SHR Vx, {, Vy}
  fn op_8XY6(&mut self, x: usize) -> ProgramCounter {
    self.v[0xF] = self.v[x] & 0x1;
    self.v[x] >>= 1;
    ProgramCounter::Next
  }

  // SUBN Vx, Vy
  fn op_8XY7(&mut self, x: usize, y: usize) -> ProgramCounter {
    let result = self.v[x] as i8 - self.v[y] as i8;
    self.v[x] = result as u8;
    self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };

    ProgramCounter::Next
  }

  // SHL Vx {, Vy}
  fn op_8XYE(&mut self, x: usize) -> ProgramCounter {
    self.v[0xF] = (self.v[x] & 0x80) >> 7;
    self.v[x] <<= 1;
    ProgramCounter::Next
  }

  // SNE Vx, Vy
  fn op_9XY0(&mut self, x: usize, y: usize) -> ProgramCounter {
    ProgramCounter::skip_if(self.v[x] != self.v[y])
  }

  // LD I, addr
  fn op_ANNN(&mut self, nnn: usize) -> ProgramCounter {
    self.i = nnn;
    ProgramCounter::Next
  }

  // JP V0, addr
  fn op_BNNN(&mut self, nnn: usize) -> ProgramCounter {
    ProgramCounter::Jump((self.v[0] as usize) + nnn)
  }

  // RND Vx, byte
  fn op_CXKK(&mut self, x: usize, kk: u8) -> ProgramCounter {
    let mut rng = OsRng::new().unwrap();
    let mut key = [0u8; 16];
    rng.fill_bytes(&mut key);
    self.v[x] = (rng.next_u32() as u8) & kk;

    ProgramCounter::Next
  }

  // DRW Vx, Vy, nibble
  fn op_DXYN(&mut self, x: usize, y: usize, n: usize) -> ProgramCounter {
    let vx = self.v[x] as usize;
    let vy = self.v[y] as usize;
    let i = self.i;
    let sprite = &self.memory[i..i + n];
    let collision = self.display.draw(vx, vy, sprite);
    self.v[0xF] = if collision { 1 } else { 0 };
    ProgramCounter::Next
  }

  // SKP Vx
  fn op_EX9E(&mut self, x: usize) -> ProgramCounter {
    ProgramCounter::skip_if(self.keypad.is_key_pressed(self.v[x]))
  }

  // SKNP Vx
  fn op_EXA1(&mut self, x: usize) -> ProgramCounter {
    ProgramCounter::skip_if(!self.keypad.is_key_pressed(self.v[x]))
  }

  // LD Vx, DT
  fn op_FX07(&mut self, x: usize) -> ProgramCounter {
    self.v[x] = self.dt;
    ProgramCounter::Next
  }

  // LD Vx, K
  fn op_FX0A(&mut self, x: usize) -> ProgramCounter {
    self.wait_for_keypress(x);

    ProgramCounter::Next
  }

  // LD DT, Vx
  fn op_FX15(&mut self, x: usize) -> ProgramCounter {
    self.dt = self.v[x];
    ProgramCounter::Next
  }

  // LD ST, Vx
  fn op_FX18(&mut self, x: usize) -> ProgramCounter {
    self.st = self.v[x];
    ProgramCounter::Next
  }

  // ADD I, Vx
  fn op_FX1E(&mut self, x: usize) -> ProgramCounter {
    self.i += self.v[x] as usize;
    ProgramCounter::Next
  }

  // LD F, Vx
  fn op_FX29(&mut self, x: usize) -> ProgramCounter {
    self.i = (self.v[x] as usize) * 5;
    ProgramCounter::Next
  }

  // LD B, Vx
  fn op_FX33(&mut self, x: usize) -> ProgramCounter {
    self.memory[self.i] = self.v[x] / 100;
    self.memory[self.i + 1] = (self.v[x] / 10) % 10;
    self.memory[self.i + 2] = (self.v[x] % 100) % 10;
    ProgramCounter::Next
  }

  // LD [I], Vx
  fn op_FX55(&mut self, x: usize) -> ProgramCounter {
    for i in 0..=x {
      self.memory[self.i + i] = self.v[i];
    }

    ProgramCounter::Next
  }

  // LD Vx, [I]
  fn op_FX65(&mut self, x: usize) -> ProgramCounter {
    for i in 0..=x {
      self.v[i] = self.memory[self.i + i];
    }

    ProgramCounter::Next
  }

  fn decrement_timers(&mut self) {
    if self.st > 0 {
      self.st -= 1;
    }

    if self.dt > 0 {
      self.dt -= 1;
    }
  }

  fn wait_for_keypress(&mut self, x: usize) {
    for (i, key) in self.keypad.keys.iter().enumerate() {
      if *key == true {
        self.v[x] = i as u8;
        break;
      }
    }
    self.pc -= 2;
  }

  fn get_opcode(first: u8, second: u8) -> u16 {
    (first as u16) << 8 | second as u16
  }
}

enum ProgramCounter {
  Next,
  Skip,
  Jump(usize),
}

impl ProgramCounter {
  fn skip_if(skip: bool) -> ProgramCounter {
    if skip {
      ProgramCounter::Skip
    } else {
      ProgramCounter::Next
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

#[wasm_bindgen]
pub struct Output {
  vram: Vec<u8>,
  beep: bool,
}

#[wasm_bindgen]
impl Output {
  pub fn new(vram: Vec<u8>, beep: bool) -> Output {
    Output { vram, beep }
  }

  pub fn get_vram(&self) -> Vec<u8> {
    self.vram.clone()
  }
}
