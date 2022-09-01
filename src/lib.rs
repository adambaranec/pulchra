pub mod interpreter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start(){
interpreter::start()
}
