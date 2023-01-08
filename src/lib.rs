use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, AudioContext};
use crate::drawing::startup::start;
use crate::interpreter::interpreter::{interpret,render,play_audios,compare_audios,Shape,Oscillator,Engine};

pub mod canvas;
pub mod drawing;
pub mod enums;
pub mod error;
pub mod fft;
pub mod interpreter;

#[wasm_bindgen]
pub fn clear(gl: WebGl2RenderingContext){
start(&gl);
}
#[wasm_bindgen]
pub fn set(mut code: String, audio: AudioContext){
}
  
