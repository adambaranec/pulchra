use web_sys::*;

pub fn start(gl: &WebGl2RenderingContext){
    gl.clear_color(0.0,0.0,0.0,1.0);
    gl.clear_depth(0.5);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
  }