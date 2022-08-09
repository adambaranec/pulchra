pub mod interpreter;
pub mod renderer;
use wasm_bindgen::prelude::*;
use web_sys::{Document,Element,HtmlElement,Node,Window};

#[wasm_bindgen]
pub fn draw(){
    match interpreter::interpret(""){
        Ok(input) => {},
        Err(err) => {},
    }
}