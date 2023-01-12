use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::*;
use crate::drawing::startup::start;
use crate::renderer::renderer::render;
use crate::interpreter::interpreter::{interpret,play_audios,compare_audios};

pub mod canvas;
pub mod drawing;
pub mod enums;
pub mod error;
pub mod fft;
pub mod renderer;
pub mod interpreter;

#[wasm_bindgen]
pub fn clear(gl: WebGl2RenderingContext){
start(&gl);
}
#[wasm_bindgen]
pub fn set(mut code: String, audio: AudioContext)->Object{
interpret(&code, &audio)
}
  
