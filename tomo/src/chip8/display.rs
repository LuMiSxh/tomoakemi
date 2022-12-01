use std::fmt;
use crate::log;
use crate::prelude::*;

use super::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

#[wasm_bindgen(getter_with_clone)]
pub struct DisplayOutput {
    pub collision: bool,
    pub pixel_x: Vec<usize>,
    pub pixel_y: Vec<usize>,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Display {
    vram: [[u8; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..(DISPLAY_HEIGHT - 1)  {
            for x in 0..(DISPLAY_WIDTH - 1) {
                write!(f, "{}", if self.get_pixel(y, x) { "⬜" } else { "□" }).expect("Could not read vram");
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
            vram: [[0; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
        }
    }

    pub fn set_pixel(&mut self, y: usize, x: usize, state: bool) {
        self.vram[y][x] = if state { 1 } else { 0 };
    }

    pub fn get_pixel(&self, y: usize, x: usize) -> bool {
        self.vram[y][x] == 1
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> DisplayOutput {
        let mut pixel_x: Vec<usize> = vec![];
        let mut pixel_y: Vec<usize> = vec![];
        let rows = sprite.len();
        let mut collision = false;
        for j in 0..rows {
            let row = sprite[j];
            for i in 0..8 {
                let new_value = row >> (7 - i) & 0x01;
                let xi = (x + i) % DISPLAY_WIDTH;
                let yj = (y + j) % DISPLAY_HEIGHT;
                let old_value = self.get_pixel(xi, yj);
                if old_value {
                    collision = true;
                }
                self.set_pixel(yj, xi, (new_value == 1) ^ old_value);
                pixel_x.push(xi);
                pixel_y.push(yj);
            }
        }
        log!("{}", self);
        return DisplayOutput { collision, pixel_x, pixel_y };
    }

    pub fn cls(&mut self) {
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                self.vram[y][x] = 0;
            }
        }
    }
}
