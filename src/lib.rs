/*use js_sys::Object;

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
}*/

use three_d::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

fn render(){
    let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
    .dyn_into::<HtmlCanvasElement>().unwrap();
    let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    let window = three_d::Window::new(WindowSettings {
      canvas: Some(canvas),
     ..Default:: default()
     }).unwrap();
     let context = window.gl();
     let mut camera:Camera = Camera::new_perspective(
        window.viewport(),
        vec3(1.0, 1.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 3.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0
        );
    window.render_loop(move |frame_input: FrameInput|{
        frame_input.screen().clear(ClearState::color(0.0,0.0,0.0,1.0));
        FrameOutput::default()
    });
}

#[wasm_bindgen]
pub fn set(mut code: String, audio: AudioContext){
}
#[wasm_bindgen]
pub fn draw(){
    render();
}
  
