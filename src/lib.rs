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
  enum Medium{
    Visuals,
    Background,
    Audio,
    Mixed,
    Effect,
    Multiplication,
    Unknown
  }
  #[derive(PartialEq)]
  enum Variant{
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
  #[derive(PartialEq)]
  enum TimeDomain{
   Red,
   Green,
   Blue,
   Scale,
   Rotation,
   Transform(f32,f32,f32)
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
    else if Regex::new("mul").unwrap().is_match(word){Medium::Multiplication}
    else if Regex::new("(lo|mi|hi)/g").unwrap().is_match(word){Medium::Mixed}
    else {Medium::Unknown}
}
/*
INTERPRETING
 */

fn floats_from(word: &str)->Option<Vec<f32>>{
    let mut floats:Vec<f32> = vec![];
    let mut errors:u8 = 0;
    for float in Regex::new(r"(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))").unwrap().find_iter(word){
      match String::from(float.as_str()).parse::<f32>(){
        Ok(val)=>floats.push(val),
        Err(err)=>errors += 1,
      }
    };
    if floats.len() == 0 || errors > 0{
      None
    } else {Some(floats) }
  }

fn color(word: &str)->Option<[f32; 3]>{
    match word{
        "red"=>Some([1.0,0.0,0.0]),
        "green"=>Some([0.0,1.0,0.0]),
        "yellow"=>Some([1.0,1.0,0.0]),
        "magenta"=>Some([1.0,0.0,1.0]),
        "cyan"=>Some([0.0,1.0,1.0]),
        "orange"=>Some([1.0,0.45,0.0]),
        "pink"=>Some([1.0,0.6,0.8]),
        "purple"=>Some([0.2,0.0,0.5]),
        "brown"=>Some([0.3,0.2,0.1]),
        "beige"=>Some([0.5,0.4,0.3]),
        "black"=>Some([0.0,0.0,0.0]),
        "white"=>Some([1.0,1.0,1.0]),
        "grey"=>Some([0.3,0.3,0.3]),
        "gray"=>Some([0.3,0.3,0.3]),
        _=>None
    }
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

fn uv(word: &str)->Result<[f32; 2],()>{
match word{
    "mid"=>Ok([0.0,0.0]),
    _=>Err(())
}
}

fn send_err(p: &HtmlParagraphElement, error: &str){
    let content = format!("{} {}", p.inner_html(), error);
    p.set_inner_html(&content);
}

fn interpret(){
let freq_regex = Regex::new(r"[\d]+\.[\d]+|[\d]+").unwrap();
let note_regex = Regex::new(r"((c|d|e|f|g|a|b)(\d)|(c|d|e|f|g|a|b)(is|isis|es|eses)(\d))").unwrap();
let gain_regex = Regex::new(r"((0.(\d+)|1|0)").unwrap();
let pan_regex = Regex::new(r"\[(((0.(\d+)|1|0))|(l)|(c)|(r))\]").unwrap();
let ra_regex = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
let rgb_regex = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)|[a-z]+").unwrap();
let uv_regex = Regex::new(r"\[(((0.(\d+)|1|0))|\-(0.(\d+)|1|0)),(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\]|[a-z]+").unwrap();
let rot_regex = Regex::new(r"rotate\((((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\)").unwrap();

let mut freq:Option<f32> = None;
let mut gain:Option<f32> = None;
let mut pan:Option<f32> = None;
let mut ra:Option<f32> = None;
let mut rgb:Option<[f32; 3]> = None;
let mut uv:Option<[f32; 2]> = None;
let mut rot:Option<f32> = None;

let document = web_sys::window().unwrap().document().unwrap();
let error_p = document.get_element_by_id("error").unwrap().dyn_into::<HtmlParagraphElement>().unwrap();
let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
let code = document.get_element_by_id("input").unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value();
let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();

error_p.set_inner_html("");

if !code.is_empty(){
  let comms: Vec<&str> = code.split(';').collect();
  for comm in comms{
      let expr: Vec<&str> = comm.split_whitespace().collect();
      if medium(expr[0]) == Medium::Unknown{ send_err(&error_p, "Unknown media. Allowed: screen, cube, saw... More in help"); }
        else if medium(expr[0]) == Medium::Background && expr.len() == 1{ send_err(&error_p, "Missing color"); } 
        else if medium(expr[0]) == Medium::Background && expr.len() > 2{ send_err(&error_p, "Too many parameters, color is enough"); } 
        else if medium(expr[0]) == Medium::Visuals && expr.len() > 5{ send_err(&error_p, "More than 4 parameters are not provided for models"); } 
        else if medium(expr[0]) == Medium::Audio && expr.len() == 1{ send_err(&error_p, "Missing frequency for the oscillator."); } 
        else if medium(expr[0]) == Medium::Audio && expr.len() > 3{ send_err(&error_p, "More than 3 parameters are not provided for oscillators"); } 
        else if medium(expr[0]) == Medium::Multiplication && expr.len() == 1{ send_err(&error_p, "Number of rows and number of columns is needed"); } 
        else if medium(expr[0]) == Medium::Multiplication && expr.len() > 3{ send_err(&error_p, "Other parameters are not needed for scissoring the canvas"); } 
        else {
        for word in &expr{
          if freq_regex.is_match(word){ match String::from(*word).parse::<f32>(){ Ok(f)=>{freq = Some(f);}, Err(e)=>{}} }
          else if note_regex.is_match(word){}
          else if gain_regex.is_match(word){ match String::from(*word).parse::<f32>(){ Ok(g)=>{gain = Some(g);}, Err(e)=>{}} }
          else if pan_regex.is_match(word){ match String::from(*word).parse::<f32>(){ Ok(p)=>{pan = Some(p);}, Err(e)=>{}} }
          else if ra_regex.is_match(word){ match String::from(*word).parse::<f32>(){ Ok(r)=>{ra = Some(r);}, Err(e)=>{}} }
          else if rgb_regex.is_match(word){ 
            if rgb_regex.is_match(r"[a-z]+") && rgb_regex.find(r"(0.(\d+)|1|0))") == None{
              if color(word) != None {rgb = color(word);}
            } else {
              let arr = floats_from(*word); if arr != None {rgb = Some([arr.clone().unwrap()[0], arr.clone().unwrap()[1], arr.clone().unwrap()[2]]);} else {} 
            }
          }
          else if uv_regex.is_match(word){ let arr = floats_from(*word); if arr != None {uv = Some([arr.clone().unwrap()[0], arr.clone().unwrap()[1]]);} else {} }
          else if rot_regex.is_match(word){ let arr = floats_from(*word); if arr != None {rot = Some(arr.clone().unwrap()[0]);} else {} }
          else {send_err(&error_p, "Invalid parameters.")}
         }
         if error_p.inner_html() == ""{
             match medium(expr[0]){
                 Medium::Background=>{ if rgb != None { clear(&gl, rgb.unwrap()); } else { send_err(&error_p, "Invalid screen color."); } },
                 Medium::Visuals=>{},
                 Medium::Audio=>{},
                 Medium::Effect=>{},
                 _=>todo!()
             }
         }
      }
  }
} else { clear(&gl, [0.0,0.0,0.0]); }
}

/*
OPERATIONS WITH WEBGL
 */
fn clear(gl: &WebGl2RenderingContext, color: [f32; 3]){
gl.clear_color(color[0], color[1], color[2], 1.0);
gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

fn render(models: &[&dyn Object]){
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

    window.render_loop(move |frame_input: FrameInput|{
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
  
