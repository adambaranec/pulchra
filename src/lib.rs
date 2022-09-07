pub mod interpreter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start(){
interpreter::start()
}
#[wasm_bindgen]
pub fn set(code: String){
interpreter::set(code)
}
