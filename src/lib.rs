extern crate cfg_if;
extern crate rand_os;
extern crate wasm_bindgen;

const MEM_SIZE: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub mod cpu;
pub mod display;
pub mod font;
pub mod keypad;
