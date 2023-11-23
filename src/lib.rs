/*
This is the legacy code of the original
rendering context and interpreter of Pulchra.
Rather do not try to run it :)
*/
use js_sys::{JsString};
use noise::*;
use regex::Regex;
use std::ops::Mul;
use std::sync::Arc;
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
    Light,
    Camera,
    Unknown
  }
  #[derive(PartialEq)]
  enum Variant{
    Screen,
    Cube,
    Sphere,
    Quad,
    Circle,
    Square,
    Line,
    Point,
    Arc,
    Cylinder,
    Cone,
    Particles,
    SinOsc,
    SawOsc,
    SqrOsc,
    TriOsc,
    NoiseOsc,
    Multiplication,
    ProcMat,
    Custom,
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
fn str(self)->JsString{
match self{
  Domain::RedMat(v)=>JsString::from("surfaceRed"),
  Domain::GreenMat(v)=>JsString::from("surfaceGreen"),
  Domain::BlueMat(v)=>JsString::from("surfaceBlue"),
  Domain::RedScr(v)=>JsString::from("screenRed"),
  Domain::GreenScr(v)=>JsString::from("screenGreen"),
  Domain::BlueScr(v)=>JsString::from("screenBlue"),
  Domain::Scale(v)=>JsString::from("scale"),
  Domain::Rotation(v)=>JsString::from("rotationSpeed"),
  Domain::Transform(x,y,z)=>JsString::from("transformCoords"),
  _=>todo!()
}
}
}

fn medium(word: &str)->Medium{
    if Regex::new("screen").unwrap().is_match(word){Medium::Background}
    else if Regex::new("cube").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("sphere").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("circle").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("square").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("cone").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("cylinder").unwrap().is_match(word){Medium::Visuals}
    else if Regex::new("sin").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("saw").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("sqr").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("tri").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("rnd").unwrap().is_match(word){Medium::Audio}
    else if Regex::new("mul").unwrap().is_match(word){Medium::Multiplication}
    else if Regex::new("(lo|mi|hi)/g").unwrap().is_match(word){Medium::Mixed}
    else if word == "particles" {Medium::Effect}
    else {Medium::Unknown}
}
fn variant(word: &str)->Variant{
  match word{
  "screen"=>Variant::Screen,
  "cube"=>Variant::Cube,
  "sphere"=>Variant::Sphere,
  "circle"=>Variant::Circle,
  "square"=>Variant::Square,
  "line"=>Variant::Line,
  "particles"=>Variant::Particles,
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
    for float in Regex::new(r"[+-]?([0-9]*[.])?[0-9]+").unwrap().find_iter(word){
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
let param_regex = Regex::new(r"((0.(\d+)|1|0))|\-(0.(\d+)|1|\-1|1.0|\-1.0|0))").unwrap();
let float_regex = Regex::new(r"(0.(\d+)|1|0)").unwrap();
let amp_regex = Regex::new(r"amp*(0.(\d+)|1|0)|amp").unwrap();
let finder = param_regex.find_iter(word);
if param_regex.find_iter(word).count() > 1{
  for param in finder.into_iter(){
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
    let value = float_regex.find(param_regex.find_iter(word).nth(0).unwrap().as_str()).unwrap().as_str().parse::<f32>().unwrap();
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
        "blue"=>Some([0.0,0.0,1.0]),
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

fn send_err(p: &HtmlParagraphElement, error: &str){
    let content = format!("{} {}", p.inner_html(), error);
    p.set_inner_html(&content);
}

pub struct Model {
  pub geometry: Box<dyn Geometry>,
  pub material: Box<dyn Material>,
  pub rotation: Option<(f32,char)>,
}
impl Model {
  fn new(context: &Context) -> Self {
      Model { geometry: Box::new(Mesh::new(context, &CpuMesh::default())), material: Box::new(ColorMaterial {color: Color::WHITE, ..Default::default()}),
    rotation: None}
  }
}
pub struct ParticleSettings {
pub pos: Vec<Vector3<f32>>,
pub colors: Vec<Color>
}
pub struct Environment {
  pub background: ClearState,
  pub back_texture: Option<Vec<Color>>,
  pub models: Vec<Model>,
  pub particles: Vec<ParticleSettings>,
  pub multiplication: Option<(u32,u32)>
}
impl Environment {
  fn empty() -> Self{
Environment { background: ClearState::color(0.0, 0.0, 0.0, 1.0), back_texture: None, models: vec![], particles: vec![], multiplication: None }
  }
}

#[wasm_bindgen]
pub fn interpret(){
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
  let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();;
  let code = document.get_element_by_id("input").unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value();
  let context = Context::from_gl_context(Arc::new(three_d::context::Context::from_webgl2_context(gl))).unwrap();
  let error_p = document.get_element_by_id("error").unwrap().dyn_into::<HtmlParagraphElement>().unwrap();
  let mut background = ClearState::color(0.0,0.0,0.0,1.0);
  let mut back_texture:Option<Texture2DRef> = None;
  let mut models:Vec<Box<Model>> = vec![];
  let mut multiplication:Option<(u32,u32)> = None;
  /*let window = three_d::Window::new(WindowSettings {
    canvas: Some(canvas.clone()),
  ..Default:: default()
  }).unwrap();
  let context = window.gl();
  let mut camera:Camera = Camera::new_perspective(
   window.viewport(),
   vec3(0.0, 0.0, 4.0),
   vec3(0.0, 0.0, 0.0),
   vec3(0.0, 3.0, 0.0),
   degrees(45.0),
   0.1,
   1000.0);*/
  error_p.set_inner_html("");
  if code != ""{
    let comms: Vec<&str> = code.split(';').collect();
    for comm in comms{
      let expr: Vec<&str> = comm.split_whitespace().collect();
      if medium(expr[0]) == Medium::Unknown{ send_err(&error_p, "Unknown media. Allowed: screen, cube... More in help"); }
        else if medium(expr[0]) == Medium::Background && expr.len() == 1{ send_err(&error_p, "Missing color, either rgb() or color name"); } 
        else if medium(expr[0]) == Medium::Background && expr.len() > 2{ send_err(&error_p, "Too many parameters, color is enough"); } 
        else if medium(expr[0]) == Medium::Visuals && expr.len() > 5{ send_err(&error_p, "More than 4 parameters are not provided for models"); } 
        else if medium(expr[0]) == Medium::Audio && expr.len() == 1{ send_err(&error_p, "Missing frequency for the oscillator."); } 
        else if medium(expr[0]) == Medium::Audio && expr.len() > 3{ send_err(&error_p, "More than 3 parameters are not provided for oscillators"); } 
        else {
          match medium(expr[0]){
            Medium::Background=>{
              if color(expr[1]) != None {
                let values = color(expr[1]).unwrap();
                background = ClearState::color(values[0],values[1],values[2],1.0);            
              } else if String::from(expr[1]).parse::<f32>().is_ok(){
                let scale = String::from(expr[1]).parse::<f32>().unwrap();
                 background = ClearState::color(scale,scale,scale,1.0);
              } else if expr[1].starts_with("rgb(") && expr[1].ends_with(")") {
                let array = floats_from(expr[1]);
                if array != None {
                  let a = array.unwrap();
                  if a.len() == 3 {
                    if a[0] >= 0.0 && a[0] <= 1.0 &&
                    a[1] >= 0.0 && a[1] <= 1.0 &&
                    a[2] >= 0.0 && a[2] <= 1.0 
                    {  background = ClearState::color(a[0],a[1],a[2],1.0); 
                    }
                    else { send_err(&error_p, "Allowed range is 0 - 1"); }
                  } else {
                    send_err(&error_p, "RGB must always be given with three parameters");
                  } 
                } else {
                  send_err(&error_p, "An error occurred within the RGB parameter");
                }       
              } else if expr[1].starts_with("noise(") && expr[1].ends_with(")"){
                let colors = &expr[1][5..expr[1].find(')').unwrap()];
                let mut array:Vec<Color> = vec![];
                if colors.contains(',') && colors.split(',').count() == 2{
                  for name in colors.split(','){
                    if color(name) != None {
                      array.push(Color::from_rgb_slice(&color(name).unwrap()));
                    }
                  }
                }
                /*let width = window.size().0;
                let height = window.size().1;
                let mut texture = Texture2D::new_empty::<Color>(&context, width, height, Interpolation::Linear, Interpolation::Linear, 
                  Some(Interpolation::Linear), Wrapping::Repeat, Wrapping::Repeat);
                let noise = Simplex::new(100);
                let mut data:Vec<Color> = vec![];
                let first = array[0];
                let second = array[1];
                for h in 0..height {
                 for w in 0..width{
                 let value = (noise.get([w as f64, h as f64]) + 1.0 / 2.0) as f32;
                 let mut color = Color::default();
                 color.r = (((first.r - second.r) as f32).abs() * value) as u8;
                 color.g = (((first.g - second.g) as f32).abs() * value) as u8;
                 color.b = (((first.b - second.b) as f32).abs() * value) as u8;
                 data.push(color);
                 }
                 }
                texture.fill(&data);
                back_texture = Some(Texture2DRef::from(Arc::new(texture)));*/
              }
            },
            Medium::Visuals=>{
              let mut ra:Option<f32> = None;
              let mut rgb:Option<[f32; 3]> = None;
              let mut uv:Option<(f32,f32)> = None;
              let mut rot:Option<f32> = None;
              let mut model = Model::new(&context); 
              if variant(expr[0]) != Variant::Unknown {
                if variant(expr[0]) != Variant::Line{
                  let mut cpuMesh = CpuMesh::default();
                  let mut mesh = Mesh::new(&context, &cpuMesh);
                  match variant(expr[0]){
                    Variant::Cube=>cpuMesh = CpuMesh::cube(),
                    Variant::Sphere=>cpuMesh = CpuMesh::sphere(40),
                    Variant::Circle=>cpuMesh = CpuMesh::circle(40),
                    Variant::Square=>cpuMesh = CpuMesh::square(),
                    Variant::Cylinder=>cpuMesh = CpuMesh::cylinder(40),
                    _=>todo!()  
                  }
                  for i in 1..expr.len(){
                    if String::from(expr[i]).parse::<f32>().is_ok(){
                      if ra == None {
                        let radius = String::from(expr[i]).parse::<f32>().unwrap();
                        if radius > 1.0 || radius < 0.0 { send_err(&error_p, "Allowed range is 0 - 1"); } else {
                          cpuMesh.transform(&Mat4::from_scale(radius));
                        }
                      } else {
                        send_err(&error_p, "You are free to give whatever parameters, but not to repeat them");
                      }
                    } else if color(expr[i]) != None {
                      let a = color(expr[i]).unwrap();
                      model.material = Box::new(ColorMaterial {color: Color::from_rgb_slice(&[a[0],a[1],a[2]]), ..Default::default()});
                    } else if expr[i].starts_with("rgb(") && expr[i].ends_with(")") {
                      if rgb == None {
                        let array = floats_from(expr[i]);
                        if array != None {
                          let a = array.unwrap();
                          if a.len() == 3 {
                            if a[0] >= 0.0 && a[0] <= 1.0 &&
                            a[1] >= 0.0 && a[1] <= 1.0 &&
                            a[2] >= 0.0 && a[2] <= 1.0 { 
                              model.material = Box::new( ColorMaterial {color: Color::from_rgb_slice(&[a[0],a[1],a[2]]), ..Default::default()} );
                             }
                            else { 
                              send_err(&error_p, "Allowed range is 0 - 1");
                             }
                          } else {
                            send_err(&error_p, "RGB must always be given with three parameters");
                          }                         
                        } else {
                          send_err(&error_p, "An error occurred within the RGB parameter");
                        }
                      } else {
                        send_err(&error_p, "You are free to give whatever parameters, but not to repeat them");
                      }
                    } else if expr[i].starts_with("rot") && expr[i].find('(').unwrap() == 4 && expr[i].ends_with(")") {
                      if rot == None {
                        let value = floats_from(expr[i]);
                        if value != None {
                          let speed = value.clone().unwrap()[0];
                          if expr[i].chars().nth(3) == Some('X'){
                            model.rotation = Some((value.clone().unwrap()[0],'X'));
                            mesh.set_animation(move |time|Mat4::from_angle_x(radians(time * 360.0 / speed)));
                          } else if expr[i].chars().nth(3) == Some('Y'){
                            model.rotation = Some((value.clone().unwrap()[0],'Y'));
                            mesh.set_animation(move |time|Mat4::from_angle_y(radians(time * 360.0 / speed)));
                          } else if expr[i].chars().nth(3) == Some('Z'){
                            model.rotation = Some((value.clone().unwrap()[0],'Z'));
                            mesh.set_animation(move |time|Mat4::from_angle_z(radians(time * 360.0 / speed)));
                          } else {
                            send_err(&error_p, "Please specify the axis to rotate around");
                          }
                        }
                      } else {
                        send_err(&error_p, "You are free to give whatever parameters, but not to repeat them");
                      }
                    } else if expr[i].starts_with("mat(") && expr[i].ends_with(")") {
                      let mut colors_group:Vec<Color> = vec![];
                      let mut colors:Vec<Color> = vec![];
                      let slice = &expr[i][4..expr[i].find(')').unwrap()];
                      if slice.len() != 0{
                        if slice.contains(','){
                          for name in slice.split(','){
                            if color(name) != None {
                              colors_group.push(Color::from_rgb_slice(&color(name).unwrap()));
                            }
                          }
                        } else {
                          colors_group.push(Color::from_rgb_slice(&color(slice).unwrap()));
                        }
                        let mut index = 0;
                        for i in 0..cpuMesh.positions.len(){
                          colors.push(colors_group[index]);
                          index += 1;
                          if index == colors_group.len(){index = 0;}
                        }                      
                        cpuMesh.colors = Some(colors);
                        model.material = Box::new(ColorMaterial::default());
                      } else {
                        send_err(&error_p, "Give at least one color (not RGB!), or better, three colors.");
                      }
                    } else if expr[i].starts_with("[") && expr[i].ends_with("]"){
                      if uv == None {
                        let coords = floats_from(expr[i]);
                        if coords != None {
                          let c = coords.unwrap();
                          if c.len() == 2 {
                            if c[0] >= -1.0 && c[0] <= 1.0 &&
                            c[1] >= -1.0 && c[1] <= 1.0{
                              cpuMesh.transform(&Mat4::from_translation(vec3(c[0] * 4.0,c[1] * 4.0, 0.0)));
                            } else {
                              send_err(&error_p, "Allowed range is (-1) - 1");
                            }
                          } else {
                            send_err(&error_p, "UV coordinates are given only with two numbers");
                          }
                        } else {
                          send_err(&error_p, "An error occurred within the UV parameter");
                        }
                      } else {
                        send_err(&error_p, "You are free to give whatever parameters, but not to repeat them");
                      }
                    } else if expr[i].starts_with("noise(") && expr[1].ends_with(")"){
                      let colors = &expr[i][5..expr[1].find(')').unwrap()];
                      let mut array:Vec<Color> = vec![];
                      if colors.contains(',') && colors.split(',').count() == 2{
                        for name in colors.split(','){
                          if color(name) != None {
                            array.push(Color::from_rgb_slice(&color(name).unwrap()));
                          }
                        }
                      }
                      /*let width = window.size().0;
                      let height = window.size().1;
                      let mut texture = Texture2D::new_empty::<Color>(&context, width, height, Interpolation::Linear, Interpolation::Linear, 
                        Some(Interpolation::Linear), Wrapping::Repeat, Wrapping::Repeat);
                      let noise = Simplex::new(100);
                      let mut data:Vec<Color> = vec![];
                      let first = array[0];
                      let second = array[1];
                      for h in 0..height {
                       for w in 0..width{
                       let value = (noise.get([w as f64, h as f64]) + 1.0 / 2.0) as f32;
                       let mut color = Color::default();
                       color.r = (((first.r - second.r) as f32).abs() * value) as u8;
                       color.g = (((first.g - second.g) as f32).abs() * value) as u8;
                       color.b = (((first.b - second.b) as f32).abs() * value) as u8;
                       data.push(color);
                       }
                       }
                      texture.fill(&data);
                      model.material = Box::new(ColorMaterial {texture: Some(Texture2DRef::from(Arc::new(texture))), ..Default::default()});*/
                    } else {
                      send_err(&error_p, "Unknown parameter. Allowed: range ((-1) - 1), color (either names or rgb(0-1,0-1,0-1)) etc.");
                    }
                  }
                  model.geometry = Box::new(mesh);
                } else if variant(expr[0]) == Variant::Line {
                  let mut line = Line::new(&context, vec2(0.0,window.size().1 as f32 / 2.0),vec2(window.size().0 as f32, window.size().1 as f32 / 2.0), 10.0);
                } else {
                  for i in 1..expr.len(){
                  }
                }
              } else {
                send_err(&error_p, "Unknown shape");
              } 
              models.push(Box::new(model));   
            },
            Medium::Effect=>{
              if variant(expr[0]) == Variant::Particles{

              }
            },
            Medium::Multiplication=>{
                match expr.len(){
                  1=>send_err(&error_p, "Give at least a number"),
                  2=>{
                    let division = String::from(expr[1]).parse::<u32>();
                    if division.is_ok(){
                      multiplication = Some((division.clone().unwrap(),division.clone().unwrap()));
                    } else {
                      send_err(&error_p, "Could not parse rows/columns. Try positive integer like 10");
                    }
                  },
                  3=>{
                    let rows = String::from(expr[1]).parse::<u32>();
                    let columns = String::from(expr[2]).parse::<u32>();
                    if rows.is_ok() && columns.is_ok(){
                      multiplication = Some((columns.clone().unwrap(),rows.clone().unwrap()));
                    } else {
                      send_err(&error_p, "Could not parse rows/columns. Try positive integers like 10, 13");
                    }
                  },
                  _=>send_err(&error_p, "It is enough to give only two numbers")
                }
            },
            Medium::Light=>{},
            Medium::Camera=>{},
            _=>todo!()
          }
        }
    }
  }
}

fn render(){
  /*let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
  let error_p = document.get_element_by_id("error").unwrap().dyn_into::<HtmlParagraphElement>().unwrap();
  let code = document.get_element_by_id("input").unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value();
  let window = three_d::Window::new(WindowSettings {
     canvas: Some(canvas.clone()),
   ..Default:: default()
   }).unwrap();
   let context = window.gl();
   let mut camera:Camera = Camera::new_perspective(
    window.viewport(),
    vec3(0.0, 0.0, 4.0),
    vec3(0.0, 0.0, 0.0),
    vec3(0.0, 3.0, 0.0),
    degrees(45.0),
    0.1,
    1000.0);
  let light = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0,0.0,1.0));
  let e = interpret(&code, &context);
  window.render_loop( move |frame_input|{  
  models.len() != 0 {
      if multiplication != None {
        let width = frame_input.window_width as f64 * frame_input.device_pixel_ratio;
        let height = frame_input.window_height as f64 * frame_input.device_pixel_ratio;
        let columns = multiplication.unwrap().0;
        let rows = multiplication.unwrap().1;
        for c in 0..columns{
          for r in 0..rows{
            let viewport = Viewport {
              x: ((width as u32 / columns) * c) as i32,
              y: ((height as u32 / rows) * r) as i32,
              width: width as u32 / columns,
              height: height as u32 / rows,
             };
             frame_input.context.set_viewport(viewport);
             camera.set_viewport(viewport);
             for model in models.iter(){
              let mut object = Gm::new(Mesh::new(&context, &*model.mesh), &*model.material);
                let mut matrix = object.transformation();
                matrix = matrix.mul(Mat4::from_scale(model.radius));
                if model.rotation != None {
                  match model.rotation.unwrap().1{
                    'X'=>matrix = matrix.mul(Mat4::from_angle_x(radians(frame_input.accumulated_time as f32 / (360.0 / -model.rotation.unwrap().0)))),
                    'Y'=>matrix = matrix.mul(Mat4::from_angle_y(radians(frame_input.accumulated_time as f32 / (360.0 / -model.rotation.unwrap().0)))),
                    'Z'=>matrix = matrix.mul(Mat4::from_angle_z(radians(frame_input.accumulated_time as f32 / (360.0 / -model.rotation.unwrap().0)))),
                    _=>todo!()
                  }
                }
                if model.coords != None {
                  matrix = matrix.mul(Mat4::from_translation(vec3(model.coords.unwrap().0 * 4.0,model.coords.unwrap().1 * 4.0,0.0)));
                }
                object.geometry.set_transformation(matrix);
                object.render(&camera, &[&light]);
            }
          }
        }
      } else {
        for model in models.iter(){
          let mut object = Gm::new(Mesh::new(&context, &CpuMesh::default()), &*model.material);
          let mut matrix = object.transformation();
          matrix = matrix.mul(Mat4::from_scale(model.radius));
          if model.rotation != None {
            match model.rotation.unwrap().1{
              'X'=>matrix = matrix.mul(Mat4::from_angle_x(radians(frame_input.accumulated_time as f32 / (360.0 / -model.rotation.unwrap().0)))),
              'Y'=>matrix = matrix.mul(Mat4::from_angle_y(radians(frame_input.accumulated_time as f32 / (360.0 / -model.rotation.unwrap().0)))),
              'Z'=>matrix = matrix.mul(Mat4::from_angle_z(radians(frame_input.accumulated_time as f32 / (360.0 / -model.rotation.unwrap().0)))),
              _=>todo!()
            }
          }
          if model.coords != None {
            matrix = matrix.mul(Mat4::from_translation(vec3(model.coords.unwrap().0 * 4.0,model.coords.unwrap().1 * 4.0,0.0)));
          }
          object.geometry.set_transformation(matrix);
          object.render(&camera, &[&light]);
        }
      }
    }

    FrameOutput::default()
  });*/
}

fn draw(e: &Environment){

}
fn texture_data(first: Color, second: Color, width: u32, height: u32) -> Vec<Color>{
  let noise = Simplex::new(100);
  let mut data:Vec<Color> = vec![];
  for h in 0..height {
    for w in 0..width{
      let value = (noise.get([w as f64, h as f64]) + 1.0 / 2.0) as f32;
      let mut color = Color::default();
      color.r = (((first.r - second.r) as f32).abs() * value) as u8;
      color.g = (((first.g - second.g) as f32).abs() * value) as u8;
      color.g = (((first.b - second.b) as f32).abs() * value) as u8;
      data.push(color);
    }
  }
  data
}