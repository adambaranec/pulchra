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

use regex::Regex;
use std::error::Error;
use three_d::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

/*
ENUMS
 */

#[derive(PartialEq)]
  pub enum Medium{
    Visuals,
    Background,
    Audio,
    Mixed,
    Effect,
    Unknown
  }
  #[derive(PartialEq)]
  pub enum Variant{
    Screen,
    Cube,
    Sphere,
    Quad,
    Circle,
    Line,
    Point,
    Arc,
    SinOsc,
    SawOsc,
    SqrOsc,
    TriOsc,
    NoiseOsc,
    Multiplication,
    Unknown
  }
fn medium(word: &str)->Medium{
    if Regex::new("screen").unwrap().is_match(word){Medium::Background}
    else if Regex::new("cube").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("sphere").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("sin").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("saw").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("sqr").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("tri").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("rnd").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("mul").unwrap().is_match(word){Medium::Effect}
    else if Regex::new("(lo|mi|hi)/g").unwrap().is_match(word){Medium::Mixed}
    else {Medium::Unknown}
}
/*
INTERPRETING
 */

fn floats_from(word: &str)->Result<Vec<f32>,&'static str>{
    let mut floats:Vec<f32> = vec![];
    for float in Regex::new(r"(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))").unwrap().find_iter(word){
      match String::from(float.as_str()).parse::<f32>(){
        Ok(val)=>floats.push(val),
        Err(err)=>todo!(),
      }
    };
    if floats.len() == 0{
      Err("There are no floats in this parameter.")
    } else {Ok(floats) }
  }

fn ra(word: &str)->Result<f32,()>{
    match word{
        "small"=>Ok(0.25),
        "medium"=>Ok(0.5),
        "large"=>Ok(0.75),
        "full"=>Ok(1.0),
        _=>Err(())
    }
}

fn color(word: &str)->Result<[f32; 3],()>{
    match word{
        "red"=>Ok([1.0,0.0,0.0]),
        "green"=>Ok([0.0,1.0,0.0]),
        "yellow"=>Ok([1.0,1.0,0.0]),
        "magenta"=>Ok([1.0,0.0,1.0]),
        "cyan"=>Ok([0.0,1.0,1.0]),
        "orange"=>Ok([1.0,0.45,0.0]),
        "pink"=>Ok([1.0,0.6,0.8]),
        "purple"=>Ok([0.2,0.0,0.5]),
        "brown"=>Ok([0.3,0.2,0.1]),
        "beige"=>Ok([0.5,0.4,0.3]),
        "black"=>Ok([0.0,0.0,0.0]),
        "white"=>Ok([1.0,1.0,1.0]),
        "grey"=>Ok([0.3,0.3,0.3]),
        "gray"=>Ok([0.3,0.3,0.3]),
        _=>Err(())
    }
}

fn uv(word: &str)->Result<[f32; 2],()>{
match word{
    "mid"=>Ok([0.0,0.0]),
    _=>Err(())
}
}

fn send_err(p: &Element, error: &str){
    p.set_inner_html(error);
}

fn interpret(code: String){
let freq_regex = Regex::new(r"[\d]+\.[\d]+|[\d]+").unwrap();
let note_regex = Regex::new(r"((c|d|e|f|g|a|b)(\d)|(c|d|e|f|g|a|b)(is|isis|es|eses)(\d))").unwrap();
let gain_regex = Regex::new(r"((0.(\d+)|1|0)").unwrap();
let pan_regex = Regex::new(r"\[(((0.(\d+)|1|0))|(l)|(c)|(r))\]").unwrap();
let ra_regex = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
let rgb_regex = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)|[a-z]+").unwrap();
let uv_regex = Regex::new(r"\[(((0.(\d+)|1|0))|\-(0.(\d+)|1|0)),(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\]|[a-z]+").unwrap();
let rot_regex = Regex::new(r"rotate\((((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\)").unwrap();
let correct_commands: Vec<Vec<Regex>> = vec![];
let document = web_sys::window().unwrap().document().unwrap();
let error_p = document.get_element_by_id("error").unwrap();
let comms: Vec<&str> = code.split(';').collect();
for comm in comms{
    let words: Vec<&str> = comm.split_whitespace().collect();
    for word in &words{
     if freq_regex.is_match(word){}
     else if note_regex.is_match(word){}
     else if gain_regex.is_match(word){}
     else if pan_regex.is_match(word){}
     else if ra_regex.is_match(word){}
     else if rgb_regex.is_match(word){}
     else if uv_regex.is_match(word){}
     else if rot_regex.is_match(word){}
     else {send_err(&error_p, "Invalid parameters.")}
    }
    if error_p.inner_html() == ""{
        match medium(words[0]){
            Medium::Background=>{},
            Medium::Visuals=>{},
            Medium::Audio=>{},
            Medium::Effect=>{},
            Medium::Unknown=>{},
            _=>todo!()
        }
    }
}
}

/*
OPERATIONS WITH WEBGL
 */
fn clear(gl: &WebGl2RenderingContext, color: [f32; 3]){
gl.clear_color(color[0], color[1], color[2], 1.0);
gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

fn render(){
    let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
    .dyn_into::<HtmlCanvasElement>().unwrap();
    let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    let window = three_d::Window::new(WindowSettings {
      canvas: Some(canvas),
     ..Default:: default()
     }).unwrap();
     let context = window.gl();
     let camera:Camera = Camera::new_perspective(
        window.viewport(),
        vec3(1.0, 1.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 3.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0
        );
    let light = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0,0.0,0.0));
    let objects:Vec<&dyn Object> = vec![];
    window.render_loop(move |frame_input: FrameInput|{
        frame_input.screen().clear(ClearState::color(0.0,0.0,0.0,1.0));
        FrameOutput::default()
    });
}
/*
OPERATIONS WITH AUDIOCONTEXT
 */
fn play(sound: Variant, ac: &AudioContext){
let gain = GainNode::new(&ac).unwrap();
let stereo = StereoPannerNode::new(&ac).unwrap();
if sound != Variant::NoiseOsc{
  let osc = OscillatorNode::new(&ac).unwrap();
  osc.start();
} else {
  let osc = AudioBufferSourceNode::new(&ac).unwrap();
  osc.start();
}
}
/*
MAIN FUNCTIONS FOR WASM
 */
#[wasm_bindgen]
pub fn set(mut code: String, audio: AudioContext){
}
#[wasm_bindgen]
pub fn draw(){
    render();
}
  
