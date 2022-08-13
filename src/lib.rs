pub mod interpreter;
use wasm_bindgen::prelude::*;
use web_sys::{Document,Element,HtmlElement,Node,Window};

#[wasm_bindgen]
pub fn start(){
interpreter::start();
}