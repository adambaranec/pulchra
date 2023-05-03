
  /*
  CASE 1: PROBLEMS WITH THREE_D::WINDOW, THEREFORE CAMERA AND OTHERS TIED TO IT CANNOT BE BUILT - SOLVED (W/ SOME EXCEPTIONS)
  CASE 2: CTRL P, CTRL H, CRTL R DON'T OPEN WINDOWS OR DIALOGS - SOLVED!
  CASE 3: PROBLEMS WITH REGEXES - SOLVED!
  CASE 4: RADIUS FOR OBJECTS - DOES NOT DISTINGUISH 0 - 1, ACCEPTS HIGHER VALUES - SOLVED!
  CASE 5: INTERPRETER RECOGNIZES WHITESPACE TO ANOTHER LINE WHICH SHOULDN'T
  CASE 6: RENDERER WORKS ONLY FOR THE FIRST TIME, AFTER THAT IT SAYS "UNREACHABLE"
   */

use js_sys::{JsString};
use regex::Regex;
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
    ProcMat,
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

struct Model {
  mesh: Box<CpuMesh>,
  material: Box<dyn Material>,
  radius: f32,
  coords: Option<(f32,f32)>,
  rotation: Option<f32>
}
impl Model {
  fn new()->Model{
    Model {mesh: Box::new(CpuMesh::default()), material: Box::new(ColorMaterial { color: Color::WHITE, ..Default::default() }), radius: 1.0, coords: None, rotation: None}
  }
}
#[wasm_bindgen]
pub fn interpret(){
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
  let mut models:Vec<Model> = vec![];
  let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
  let error_p = document.get_element_by_id("error").unwrap().dyn_into::<HtmlParagraphElement>().unwrap();
  error_p.set_inner_html("");
  let code = document.get_element_by_id("input").unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value();
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
                clear(&gl, color(expr[1]).unwrap());                
              } else if String::from(expr[1]).parse::<f32>().is_ok(){
                let scale = String::from(expr[1]).parse::<f32>().unwrap();
                 clear(&gl, [scale,scale,scale]);
              } else if expr[1].starts_with("rgb(") && expr[1].ends_with(")") {
                let array = floats_from(expr[1]);
                if array != None {
                  let a = array.unwrap();
                  if a.len() == 3 {
                    if a[0] >= 0.0 && a[0] <= 1.0 &&
                    a[1] >= 0.0 && a[1] <= 1.0 &&
                    a[2] >= 0.0 && a[2] <= 1.0 
                    { clear(&gl, [a[0],a[1],a[2]]); }
                    else { send_err(&error_p, "Allowed range is 0 - 1"); }
                  } else {
                    send_err(&error_p, "RGB must always be given with three parameters");
                  } 
                } else {
                  send_err(&error_p, "An error occurred within the RGB parameter");
                }       
              }
            },
            Medium::Visuals=>{
              let mut ra:Option<f32> = None;
              let mut rgb:Option<[f32; 3]> = None;
              let mut uv:Option<(f32,f32)> = None;
              let mut rot:Option<f32> = None;
              let mut model = Model::new(); 
              if variant(expr[0]) != Variant::Unknown {
                match variant(expr[0]){
                  Variant::Cube=>model.mesh = Box::new(CpuMesh::cube()),
                  Variant::Sphere=>model.mesh = Box::new(CpuMesh::sphere(40)),
                  _=>todo!()
                }   
                for i in 1..expr.len(){
                  if String::from(expr[i]).parse::<f32>().is_ok(){
                    if ra == None {
                      let radius = String::from(expr[i]).parse::<f32>().unwrap();
                      if radius > 1.0 || radius < 0.0 { send_err(&error_p, "Allowed range is 0 - 1"); }
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
                  } else if expr[i].starts_with("rot(") && expr[i].ends_with(")") {
                    if rot == None {
                      let value = floats_from(expr[i]);
                      if value != None {
                        if value.clone().unwrap()[0] > 1.0 || value.clone().unwrap()[0] < -1.0{
                          send_err(&error_p, "Allowed range is (-1) - 1");
                        }
                        model.rotation = Some(value.clone().unwrap()[0]);
                      }
                    } else {
                      send_err(&error_p, "You are free to give whatever parameters, but not to repeat them");
                    }
                  } else if expr[i].starts_with("[") && expr[i].ends_with("]"){
                    if uv == None {
                      let coords = floats_from(expr[i]);
                      if coords != None {
                        let c = coords.unwrap();
                        if c.len() == 2 {
                          if c[0] >= -1.0 && c[0] <= 1.0 &&
                          c[1] >= -1.0 && c[1] <= 1.0{
                            model.coords = Some((c[0]/2.0+0.5,c[1]/2.0+0.5));
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
                  } else {
                    send_err(&error_p, "Unknown parameter. Allowed: range ((-1) - 1), color (either names or rgb(0-1,0-1,0-1)) etc.");
                  }
                }
              } else {
                send_err(&error_p, "Unknown shape");
              } 
              models.push(model);   
            },
            Medium::Multiplication=>{
                match expr.len(){
                  1=>send_err(&error_p, "Give at least a number"),
                  2=>{
                    let division = String::from(expr[1]).parse::<u16>();
                    if division.is_ok(){
                      mul(&gl, division.clone().unwrap(), division.clone().unwrap());
                    } else {
                      send_err(&error_p, "Could not parse rows/columns. Try positive integer like 10");
                    }
                  },
                  3=>{
                    let rows = String::from(expr[1]).parse::<u16>();
                    let columns = String::from(expr[2]).parse::<u16>();
                    if rows.is_ok() && columns.is_ok(){
                       mul(&gl, rows.clone().unwrap(), columns.clone().unwrap());
                    } else {
                      send_err(&error_p, "Could not parse rows/columns. Try positive integers like 10, 13");
                    }
                  },
                  _=>send_err(&error_p, "It is enough to give only two numbers")
                }
            },
            _=>todo!()
          }
        }
    }
    render(Box::new(models), &canvas);
  }
}

fn render(models: Box<Vec<Model>>, canvas: &HtmlCanvasElement){
  let window = three_d::Window::new(WindowSettings {
    canvas: Some(canvas.clone()),
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
  let light = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0,0.0,1.0));
  window.render_loop(move |frame_input|{
    frame_input.screen().clear(ClearState::color(0.0,0.0,0.0,1.0));
    for model in models.iter(){
      let mut geom = Mesh::new(&context, &model.mesh);
      if model.radius != 1.0 {
        geom.set_transformation(Mat4::from_scale(model.radius));
      }
      if model.coords != None {
        geom.set_transformation(Mat4::from_translation(Camera::position_at_uv_coordinates(&camera, model.coords.unwrap())));
      }
      if model.rotation != None {
        geom.set_transformation(Mat4::from_angle_y(radians((360.0/60.0) * model.rotation.unwrap())));
      }
      geom.render_with_material(&*model.material, &camera, &[&light]);
    }
    FrameOutput::default()
  });
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
  
