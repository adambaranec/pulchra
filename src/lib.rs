pub mod interpreter;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, AudioContext};
#[wasm_bindgen]
pub fn start(gl: WebGl2RenderingContext){
interpreter::start(&gl);
}
#[wasm_bindgen]
pub fn set(code: String, context: WebGl2RenderingContext, audio: AudioContext){
interpreter::set(code, context, audio);
}
#[wasm_bindgen]
pub fn draw(gl: WebGl2RenderingContext){
interpreter::draw(&gl);
}