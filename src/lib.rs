mod utils;
mod lsystem;

use wasm_bindgen::prelude::*;
use std::string::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



#[wasm_bindgen]
pub fn evaluate_system(axiom: String) -> String 
{
    return String::from("");
}
