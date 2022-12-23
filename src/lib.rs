use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, AudioContext, Window};
use js_sys::{Object, Function};

pub mod utils;
pub mod drawing;
pub mod fft_options;
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
        utils::interpret::interpret::interpret(expr, &audio);
      }
      }
     else {
        utils::interpret::interpret::interpret(&*code, &audio);
    }
  }
  
