pub mod interpreter;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
#[wasm_bindgen]
pub fn start(){
interpreter::start()
}
#[wasm_bindgen]
pub fn set(code: String, context: WebGl2RenderingContext){
interpreter::set(code, context)
}
