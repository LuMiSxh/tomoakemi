//! Crate prelude

// Formatierer als "f"
pub use std::format as f;

#[cfg(target_arch = "wasm32")]
use js_sys::Math;
#[cfg(not(target_arch = "wasm32"))]
use rand::{Rng, thread_rng};
// Wasm_bindgen
pub use wasm_bindgen::prelude::*;

// Funktion, um einen zufälligen byte zu bekommen | Funktion wird je nack Kompilationsziel verwendet
#[cfg(target_arch = "wasm32")]
pub fn random_byte() -> u8 {
    (Math::random() * 255.0) as u8
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_byte() -> u8 {
    let mut rng = thread_rng();
    (rng.gen::<f32>() * 255.0) as u8
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(msg: &str);
}
