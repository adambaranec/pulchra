pub mod interpreter;
pub mod renderer;
use wasm_bindgen::prelude::*;
use web_sys::{Document,Element,HtmlElement,Node,Window};

#[wasm_bindgen]
pub fn start(){
renderer::start();
}