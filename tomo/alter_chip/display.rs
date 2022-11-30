use crate::prelude::*;

// Konstanten für Pixelhöhe und Pixelbreite des Display
pub const HOEHE: usize = 32;
pub const BREITE: usize = 64;

// Konstante für die Schriftart (Standard Schrift in Hex-Format)
pub const FONT: [u8; 80] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

// Display, um jeden Pixel im Frontend anzuzeigen. Pixel können entweder eingeschaltet "1" oder ausgeschaltet "0" sein
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Display {
    mem: [u8; HOEHE * BREITE],
}

// Implementation für die Klasse und seine Methoden
#[wasm_bindgen]
impl Display {
    // Konstruktor
    pub fn new() -> Self {
        Display {
            mem: [0; HOEHE * BREITE],
        }
    }

    pub fn get_pixel_state(&self, position: usize) -> bool {
        self.mem[position] == 1
    }

    pub fn set_pixel_state(&mut self, position: usize, state: bool) {
        self.mem[position] = if state {1} else {0};
    }

    // Bildschirm komplett leeren
    pub fn clear_display(&mut self) {
        for x in 0..BREITE {
            for y in 0..HOEHE {
                //self.set_pixel_new(x, y, false);
                self.set_pixel_state(x +y * BREITE, false);
            }
        }
    }

    // Zeichnet einzelne Pixel oder Sprites (Ansammlung mehrerer Pixel) auf den Display
    // Wenn der Sprite auf einen Rand trifft, wird der Sprite auf der anderen Seite fortgeführt ("Wrapping")
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;
        for j in 0..rows {
            let row = sprite[j];
            for i in 0..8 {
                let new_value = row >> (7 - i) & 0x01;
                if new_value == 1 {
                    let xi = (x + i) % BREITE;
                    let yj = (y + j) % HOEHE;
                    let old_value = self.get_pixel_state(xi + yj * BREITE);
                    if old_value {
                        collision = true;
                    }
                    self.set_pixel_state(xi + yj * BREITE, (new_value == 1) ^ old_value);
                }
            }
        }
        return collision;
    }
}
