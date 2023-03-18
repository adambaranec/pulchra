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
  enum Domain{
   RedMat(f32),
   GreenMat(f32),
   BlueMat(f32),
   RedScr(f32),
   GreenScr(f32),
   BlueScr(f32),
   Scale(f32),
   Rotation(f32),
   Transform(f32,f32,f32)
  }
  impl Domain{
    fn q(self)->f32{
      match self{
        Domain::RedMat(v)=>v,
        Domain::GreenMat(v)=>v,
        Domain::BlueMat(v)=>v,
        Domain::RedScr(v)=>v,
        Domain::GreenScr(v)=>v,
        Domain::BlueScr(v)=>v,
        Domain::Scale(v)=>v,
        Domain::Rotation(v)=>v,
        _=>todo!()
      }
    }
    fn x(self)->f32{
      match self{
      Domain::Transform(x,y, z)=>x,
      _=>todo!()
    }
  }
  fn y(self)->f32{
    match self{
    Domain::Transform(x,y, z)=>y,
    _=>todo!()
  }
}
fn z(self)->f32{
  match self{
  Domain::Transform(x,y, z)=>z,
  _=>todo!()
}
}
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
fn variant(word: &str)->Variant{
  match word{
  "screen"=>Variant::Screen,
  "cube"=>Variant::Cube,
  "sphere"=>Variant::Sphere,
  "sin"=>Variant::SinOsc,
  "saw"=>Variant::SawOsc,
  "sqr"=>Variant::SqrOsc,
  "tri"=>Variant::TriOsc,
  "rnd"=>Variant::NoiseOsc,
  "mul"=>Variant::Multiplication,
  _=>Variant::Unknown
  }
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

fn domains(word: &str, m: Medium)->Option<Vec<Domain>>{
let mut index:i8 = -1;
let mut errors:u8 = 0;
let mut domains:Vec<Domain> = vec![];
let param_regex = Regex::new(r"(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))|amp|amp*(0.(\d+)|1|0)").unwrap();
let float_regex = Regex::new(r"(0.(\d+)|1|0)").unwrap();
let amp_regex = Regex::new(r"amp*(0.(\d+)|1|0)|amp").unwrap();
let finder = param_regex.find_iter(word);
if finder.count() > 1{
  for param in finder{
    index += 1;
    if amp_regex.is_match(param.as_str()){
      let value = float_regex.find(param.as_str()).unwrap().as_str().parse::<f32>().unwrap();
      match m{
        Medium::Background=>{
          match index{
            0=>domains.push(Domain::RedScr(value)),
            1=>domains.push(Domain::BlueScr(value)),
            2=>domains.push(Domain::GreenScr(value)),
            _=>todo!()
          }
        },
        Medium::Visuals=>{
          match index{
            0=>domains.push(Domain::RedMat(value)),
            1=>domains.push(Domain::BlueMat(value)),
            2=>domains.push(Domain::GreenMat(value)),
            _=>todo!()
          }
        },
        _=>todo!()
      }
    }
    else if param.as_str() == "amp"{
      match m{
        Medium::Background=>{
          match index{
            0=>domains.push(Domain::RedScr(1.0)),
            1=>domains.push(Domain::BlueScr(1.0)),
            2=>domains.push(Domain::GreenScr(1.0)),
            _=>todo!()
          }
        },
        Medium::Visuals=>{
          match index{
            0=>domains.push(Domain::RedMat(1.0)),
            1=>domains.push(Domain::BlueMat(1.0)),
            2=>domains.push(Domain::GreenMat(1.0)),
            _=>todo!()
          }
        },
        _=>todo!()
      }
    }
  }
} else if finder.count() == 1 {
  if amp_regex.is_match(word){
    let value = float_regex.find(finder.nth(0).unwrap().as_str()).unwrap().as_str().parse::<f32>().unwrap();
    match m{
      Medium::Background=>{
        domains.push(Domain::RedScr(value));
        domains.push(Domain::GreenScr(value));
        domains.push(Domain::BlueScr(value));
      },
      Medium::Visuals=>domains.push(Domain::Scale(value)),
      _=>todo!()
    }
  } else if word == "amp"{
    match m{
      Medium::Background=>{
        domains.push(Domain::RedScr(1.0));
        domains.push(Domain::GreenScr(1.0));
        domains.push(Domain::BlueScr(1.0));
      },
      Medium::Visuals=>domains.push(Domain::Scale(1.0)),
      _=>todo!()
    }
  }
}
if errors > 0 || domains.len() == 0{
  None
} else {
  Some(domains)
}
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

fn note(word: &str)->Option<f64>{
let a = 1.059463094359;
let mut power:i16 = 0;
let octave = String::from(word.chars().last().unwrap()).parse::<u16>().unwrap();
if word.contains("is"){power += 1;}
else if word.contains("es"){power -= 1;}
else if word.contains("isis"){power += 2;}
else if word.contains("eses"){power -= 2;}
match word.chars().nth(0).unwrap(){
  'c'=>{power -= 9;},
  'd'=>{power -= 7;},
  'e'=>{power -= 5;},
  'f'=>{power -= 4;},
  'g'=>{power -= 2;},
  'h'=>{power += 2;},
  _=>todo!()
}
if word == "a5"{Some(440.0)}else{
  if octave > 5{
    let distance = (octave as i16 - 5) * 12; 
    power += distance;
    let freq = 440.0 * f64::powf(a, power as f64);
    Some(freq)
  }
  else if octave < 5{
    let distance = (5 - octave as i16) * 12;
    power -= distance;
    let freq = 440.0 * f64::powf(a, power as f64);
    Some(freq)
  } else {None}
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
let tone_regex = Regex::new(r"([\d]+\.[\d]+|[\d]+)|((c|d|e|f|g|a|b)(\d)|(c|d|e|f|g|a|b)(is|isis|es|eses)(\d))").unwrap();
let gain_regex = Regex::new(r"((0.(\d+)|1|0)").unwrap();
let pan_regex = Regex::new(r"\[(((0.(\d+)|1|0))|(l)|(c)|(r))\]").unwrap();
let ra_regex = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
let rgb_regex = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)|[a-z]+").unwrap();
let uv_regex = Regex::new(r"\[(((0.(\d+)|1|0))|\-(0.(\d+)|1|0)),(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\]|[a-z]+").unwrap();
let rot_regex = Regex::new(r"ro\((((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\)").unwrap();
let mul_regex = Regex::new(r"([0-9]+)").unwrap();
let domain_regex = Regex::new(r"(amp)|(amp*(0.(\d+)|1|0))").unwrap();

let document = web_sys::window().unwrap().document().unwrap();
let error_p = document.get_element_by_id("error").unwrap().dyn_into::<HtmlParagraphElement>().unwrap();
let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
let code = document.get_element_by_id("input").unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value();
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

error_p.set_inner_html("");

if !code.is_empty(){
  let mut time_domains:Vec<(Vec<Domain>,i16)> = vec![];
  let mut models:Vec<(&dyn Object,i16)> = vec![];
  let mut rotations:Vec<(f32,i16)> = vec![];
  let comms: Vec<&str> = code.split(';').collect();
  let mut comm_index = -1;
  for comm in comms{
    comm_index += 1;
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
          match medium(expr[0]){
              Medium::Background=>{ 
                if rgb_regex.is_match(expr[1]){ 
                  if Regex::new(r"[a-z]+").unwrap().find(expr[1]) != None && Regex::new(r"(0.(\d+)|1|0))").unwrap().find(expr[1]) == None{
                    if color(expr[1]) != None {clear(&gl, color(expr[1]).unwrap()); } else {send_err(&error_p, "Invalid color name");}
                  } else {
                    let arr = floats_from(expr[1]); 
                    if arr != None {clear(&gl, color(expr[1]).unwrap()); } else {send_err(&error_p, "Invalid numbers for rgb values");}
                  }
                } else if "amp".find(expr[1]) != None {
                  let results = domains(expr[1], Medium::Background);
                  if results != None{
                    time_domains.push((results.unwrap(), -1));
                  } else {
                   send_err(&error_p, "Not a correct definition for reacting to sound");
                  }
                } else {
                  send_err(&error_p, "Invalid parameter, color allowed only");
                }
               },
              Medium::Visuals=>{
                let mut ra:Option<f32> = None;
                let mut rgb:Option<[f32; 3]> = None;
                let mut uv:Option<[f32; 2]> = None;
                let mut rot:Option<f32> = None;
                let mut model = CpuMesh::default();
                let mut material = ColorMaterial::default();
                for i in 1..expr.len()- 1{
                  if rgb_regex.is_match(expr[i]){ 
                    if Regex::new(r"[a-z]+").unwrap().find(expr[i]) != None && Regex::new(r"(0.(\d+)|1|0))").unwrap().find(expr[i]) == None{
                      if color(expr[i]) != None {
                        if rgb == None {rgb = color(expr[i]);} else {send_err(&error_p, "It is free to give whatever parameters, but not to repeat them");}
                      } else {
                        send_err(&error_p, "Could not parse color from text");
                      }
                    } else {
                      let arr = floats_from(expr[i]); 
                      if arr != None {
                        if rgb == None {rgb = Some([arr.clone().unwrap()[0], arr.clone().unwrap()[1], arr.clone().unwrap()[2]]);} else {send_err(&error_p, "It is free to give whatever parameters, but not to repeat them");}
                      } else {
                        send_err(&error_p, "Could not parse color from function");
                      }
                    }
                  }
                  else if ra_regex.is_match(expr[i]){ 
                    match String::from(expr[i]).parse::<f32>(){ 
                      Ok(r)=>{ra = Some(r);}, 
                      Err(e)=>send_err(&error_p, "Could not parse radius")
                    } 
                  }
                  else if uv_regex.is_match(expr[i]){ 
                    let arr = floats_from(expr[i]); 
                    if arr != None {
                      if uv == None {uv = Some([arr.clone().unwrap()[0], arr.clone().unwrap()[1]]);} else {send_err(&error_p, "It is free to give whatever parameters, but not to repeat them");}
                    } else {
                      send_err(&error_p, "Could not parse UV coords");
                    } 
                  } else if "amp".find(expr[i]) != None{
                    let results = domains(expr[1], Medium::Visuals);
                    if results != None{
                      time_domains.push((results.unwrap(), comm_index));
                    } else {
                     send_err(&error_p, "Not a correct definition for reacting to sound");
                    }
                  }
                  else {
                    send_err(&error_p, "Invalid parameter");
                  }
                } 
                match variant(expr[0]){
                  Variant::Cube=>{model = CpuMesh::cube();},
                  Variant::Sphere=>{model = CpuMesh::sphere(40);},
                  _=>todo!()
                }
                let mut object = &Gm::new(Mesh::new(&context, &model), material);
                if ra != None {object.set_transformation(Mat4::from_scale(ra.unwrap()));} else {object.set_transformation(Mat4::from_scale(1.0));}
                if rgb != None {material.color = Color::from_rgb_slice(&[rgb.unwrap()[0],rgb.unwrap()[1],rgb.unwrap()[2]]);}
                if uv != None {object.set_transformation(Mat4::from_translation(Camera::position_at_uv_coordinates(&camera, (uv.unwrap()[0],uv.unwrap()[1]))));} 
                if rot != None {rotations.push((rot.unwrap(), comm_index));}
                models.push((object, comm_index));
              },
              Medium::Audio=>{
                let mut freq:Option<f32> = None;
                let mut gain:Option<f32> = None;
                let mut pan:Option<f32> = None;
                if variant(expr[0]) != Variant::NoiseOsc{
                  if tone_regex.is_match(expr[1]){
                
                  } else {
                    send_err(&error_p, "Frequency must always be given at first");
                  }
                  for i in 2..expr.len()- 1{
                    
                  }
                } else {

                }
              },
              Medium::Multiplication=>{
                match expr.len(){
                  2=>{
                  if mul_regex.is_match(expr[1]){
                    let division = String::from(expr[1]).parse::<u16>();
                    let mut d:u16 = 0; 
                    match division{
                      Ok(v)=>{d = v;},
                      Err(e)=>send_err(&error_p, "Could not parse rows and columns")
                    }
                    if d != 0 {mul(&gl, d, d);}
                  }
                  },
                  3=>{
                    if mul_regex.is_match(expr[1]) && mul_regex.is_match(expr[2]){
                      let rows = String::from(expr[1]).parse::<u16>();
                      let columns = String::from(expr[1]).parse::<u16>();
                      let mut r:u16 = 0;
                      let mut c:u16 = 0;
                      match rows{
                        Ok(v)=>{r = v;},
                        Err(e)=>todo!()
                      }
                      match columns{
                        Ok(v)=>{c = v;},
                        Err(e)=>todo!()
                      }
                      if r != 0 && c != 0 {mul(&gl, r, c);}else{send_err(&error_p, "Could not parse rows and columns");}
                    }
                  },
                  _=>todo!()
                }
              },
              _=>todo!()
          }
         
      }
  }
  if !models.len() == 0 || !time_domains.len() == 0 || !rotations.len() == 0{
    let light = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0,0.0,0.0));
    window.render_loop(|frame_input|{
      for order in 0..comm_index{
        if models[order as usize].1 == order{       
          models[order as usize].0.render(&camera, &[&light]);
          if time_domains[order as usize].1 == order{}
          if rotations[order as usize].1 == order{
            /* FIXING THIS */
            //models[order as usize].set_transformation(Mat4::from_angle_x(frame_input.elapsed_time * rotations[order as usize].0));
          }
        } else {
          if time_domains[order as usize].1 == order{}
          if rotations[order as usize].1 == order{}
        }
      }
      FrameOutput::default()
    });
  }
} else { 
  clear(&gl, [0.0,0.0,0.0]); 
}
}
/*
OPERATIONS WITH WEBGL
 */
fn clear(gl: &WebGl2RenderingContext, color: [f32; 3]){
gl.clear_color(color[0], color[1], color[2], 1.0);
gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

fn mul(gl: &WebGl2RenderingContext, rows: u16, columns: u16){
  let window = web_sys::window().unwrap();
  let width = window.inner_width().unwrap().as_f64();
  let height = window.inner_height().unwrap().as_f64();
  gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
  if width != None || height != None{
      let scissor_width = width.unwrap() as i32 / rows as i32;
      let scissor_height = height.unwrap() as i32 / columns as i32;
      for c in 0..columns-1{
          for r in 0..rows-1{
          gl.scissor(scissor_width * r as i32, scissor_height * c as i32, scissor_width, scissor_height);
          }
      }
  }
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
  
