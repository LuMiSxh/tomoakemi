//! Crate prelude

// Wasm_bindgen
pub use wasm_bindgen::prelude::*;

// Formatierer als "f"
pub use std::format as f;

pub use crate::log;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn js_log(msg: &str);
}
