use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, AudioContext, Window};
use js_sys::{Object, Function};

pub mod utils;
pub mod drawing;
pub mod fft;
pub mod interpreter;
pub mod enums;
pub mod error;

#[wasm_bindgen]
pub fn start(gl: WebGl2RenderingContext){
utils::webgl::startup::start(&gl);
}
#[wasm_bindgen]
pub fn set(mut code: String, audio: AudioContext){
    if code.contains(';'){
      if code.chars().last().unwrap() == ';'{
        code.pop();
      }
      for expr in code.split(';'){
        interpreter::interpreter::interpret(expr, &audio);
      }
      }
     else {
        interpreter::interpreter::interpret(&*code, &audio);
    }
  }
  
