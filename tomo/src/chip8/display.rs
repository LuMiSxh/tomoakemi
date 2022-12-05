use std::fmt;
use crate::prelude::*;

use super::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Display {
    vram: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}


impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.vram.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                write!(f, "{}", if self.get_pixel(i, j) { "⬜" } else { "□" }).expect("Could not read vram");
            }
            write!(f, "\n").expect("Could not write");
        }

        Ok(())
    }
}


#[wasm_bindgen]
impl Display {
    pub fn new() -> Self {
        Display {
            vram: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn set_pixel(&mut self, y: usize, x: usize, state: bool) {
        self.vram[y][x] = if state { 1 } else { 0 };
    }

    pub fn get_pixel(&self, y: usize, x: usize) -> bool {
        self.vram[y][x] == 1
    }


    pub fn cls(&mut self) {
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                self.vram[y][x] = 0;
            }
        }
    }
}
