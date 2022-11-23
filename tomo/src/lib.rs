pub mod chip8;
pub mod prelude;
pub mod log;


// Wenn Feature "wee_alloc" ist aktiv, nutze "wee_alloc" als "globalen memory allocator"
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
