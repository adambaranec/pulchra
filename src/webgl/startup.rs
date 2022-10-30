use web_sys::*;
use wasm_bindgen::{JsCast,JsValue};

pub fn start(gl: &WebGl2RenderingContext){
    gl.clear_color(0.0,0.0,0.0,1.0);
    gl.clear_depth(0.5);
  }

  pub fn draw(gl: &WebGl2RenderingContext){
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
 }