use crate::prelude::*;

use super::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Display {
    vram: [[u8; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
}


#[wasm_bindgen]
impl Display {
    pub fn new() -> Self {
        Display {
            vram: [[0; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
        }
    }

    pub fn set_pixel(&mut self, y: usize, x: usize, state: bool) {
        self.vram[y][x] = if state {1} else {0};
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
