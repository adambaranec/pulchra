pub mod interpreter;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, AudioContext};
use interpreter::Audio;
#[wasm_bindgen]
pub fn start(){
interpreter::start()
}
#[wasm_bindgen]
pub fn set(code: String, context: WebGl2RenderingContext, audio: AudioContext){
interpreter::set(code, context, audio)
}
