use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, AudioContext};
pub mod utils;
#[wasm_bindgen]
pub fn start(gl: WebGl2RenderingContext){
utils::webgl::startup::start(&gl);
}
#[wasm_bindgen]
pub fn set(code: String, context: WebGl2RenderingContext, audio: AudioContext){
utils::interpret::interpret::interpret(&code, &context, &audio);
}