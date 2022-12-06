use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, AudioContext, Window};
use js_sys::{Object, Function};
pub mod utils;
#[wasm_bindgen]
extern "C" {
  fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
}
#[wasm_bindgen]
pub fn start(gl: WebGl2RenderingContext){
utils::webgl::startup::start(&gl);
}
#[wasm_bindgen]
pub fn draw(){
}
#[wasm_bindgen]
pub fn set(code: String, context: WebGl2RenderingContext, audio: AudioContext){
    if code.contains(';'){
      for expr in code.split(';'){
        utils::interpret::interpret::interpret(expr, &context, &audio);
      }
    } else {
        utils::interpret::interpret::interpret(&*code, &context, &audio);
    }
}