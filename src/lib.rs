use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, AudioContext};

pub mod canvas;
pub mod drawing;
pub mod enums;
pub mod error;
pub mod fft;
pub mod interpreter;

#[wasm_bindgen]
pub fn start(gl: WebGl2RenderingContext){
drawing::startup::start(&gl);
}
#[wasm_bindgen]
pub fn set(mut code: String, audio: AudioContext){
}
  
