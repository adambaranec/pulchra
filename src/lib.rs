use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, AudioContext, Window};
use js_sys::{Object, Function};

pub mod utils;
#[wasm_bindgen]
pub fn start(gl: WebGl2RenderingContext){
utils::webgl::startup::start(&gl);
}
#[wasm_bindgen]
pub fn draw(){
}
#[wasm_bindgen]
pub fn set(mut code: String, context: WebGl2RenderingContext, audio: AudioContext){
    if code.contains(';'){
      if code.chars().last().unwrap() == ';'{
        code.pop();
      }
      for expr in code.split(';'){
        utils::interpret::interpret::interpret(expr, &context, &audio);
      }
      }
     else {
        utils::interpret::interpret::interpret(&*code, &context, &audio);
    }
  }
  
