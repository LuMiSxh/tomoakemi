#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (println!($($t)*))
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! err {
    ($($t:tt)*) => (error(&format!($($t)*)))
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! err {
    ($($t:tt)*) => (eprintln!($($t)*))
}
