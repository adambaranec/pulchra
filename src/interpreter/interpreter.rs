use regex::Regex;
use wasm_bindgen::JsCast;
use web_sys::*;
use crate::canvas::scissor::divide_canvas;
use crate::drawing::renderer::render;
use crate::error::error::send_err;
use crate::enums::enums::{Variant,Medium};
use crate::enums::enums::{get_variant,get_medium};
use crate::fft::fft_options::FftOptions;
fn floats_from(word: &str)->Result<Vec<f32>,&'static str>{
  let mut floats:Vec<f32> = vec![];
  for float in Regex::new(r"(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))").unwrap().find_iter(word){
    match String::from(float.as_str()).parse::<f32>(){
      Ok(val)=>floats.push(val),
      Err(err)=>send_err("Invalid values, could not collect them."),
    }
  };
  if floats.len() == 0{
    Err("There are no floats in this parameter.")
  } else {Ok(floats) }
}
/* ----- just a legacy function ----------
fn analyze_func(word: &str) -> FnType{
  if Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgb;}
  else if Regex::new(r"rgba\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgba;}
  else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
  else if Regex::new(r"(?!.*(rgb|rgba|fft))").unwrap().is_match(word){return FnType::Unknown;}
  else {return FnType::NotFunction;}
  }
*/
      fn prepare_effect(w: &str){
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();
         let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
      let expr:Vec<&str> = w.split_whitespace().collect();
      match get_variant(expr[0]){
      Variant::Multiplication=>{
      if expr.len() == 2{
      match String::from(expr[1]).parse::<u16>(){
      Ok(val)=>divide_canvas(&gl, val, val),
      Err(err)=>send_err("Invalid parameter for multiplication.")
      }
      } else if expr.len() == 3{
      let mut rows:u16=0;
      let mut columns:u16=0;
      match String::from(expr[1]).parse::<u16>(){
      Ok(val)=>rows=val,
      Err(err)=>send_err("Invalid parameter.")
      }
      match String::from(expr[2]).parse::<u16>(){
      Ok(val)=>columns=val,
      Err(err)=>send_err("Invalid parameter.")
      }
      if rows != 0 && columns != 0{
        divide_canvas(&gl, rows, columns);
      }
      }
      },
      _=>todo!(),
      }
      }
      fn prepare_mixed(w: &str, audio: &AudioContext){
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();
         let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
      let expr:Vec<&str> = w.split_whitespace().collect();
      let analyser = AnalyserNode::new(audio).unwrap();
    }
      fn prepare_audio(w: &str, audio: &AudioContext){
        let expr:Vec<&str> = w.split_whitespace().collect();
        let freq_regex = Regex::new(r"[\d]+\.[\d]+|[\d]+").unwrap();
        let note_regex = Regex::new(r"((c|d|e|f|g|a|b)(\d)|(c|d|e|f|g|a|b)(#|b)(\d))").unwrap();
        let gain_regex = Regex::new(r"((0.(\d+)|1|0)").unwrap();
        let pan_regex = Regex::new(r"\[(((0.(\d+)|1|0))|(l)|(c)|(r))\]").unwrap();
        let variant = get_variant(expr[0]);

        let gain = GainNode::new(audio).unwrap();
        let panner = StereoPannerNode::new(audio).unwrap();

        let inspect_freq = |word: &str| -> Result<f32, &'static str>{
          if freq_regex.is_match(word){
            match String::from(word).parse::<f32>(){
             Ok(val)=>return Ok(val),
             Err(err)=>match String::from(word).parse::<u16>(){
                  Ok(freq)=>return Ok(freq as f32),
                  Err(err)=>return Err("Invalid frequency argument.")
                   }
            }
          } else {
            Err("Unknown parameter for the oscillator.")
          }
        };

        let inspect_gain = |word: &str| -> Result<f32, &'static str>{
          let mut gainval:f32 = 0.0;
          if gain_regex.is_match(word){
          match String::from(word).parse::<f32>(){
          Ok(val)=>{
            gainval = val;
            if gainval > 1.0{
              Err("Gain must not be higher than 1.")
            } else {
              Ok(gainval)
            }
          },
          Err(err)=>Err("Not a valid gain"),
          }
        } else {
          Err("error")
        }
        };

        let inspect_pan = |word: &str| -> Result<f32, &'static str>{
          if pan_regex.is_match(word){
             if word.contains("l"){Ok(0.0)}
             else if word.contains("c"){Ok(0.5)}
             else if word.contains("r"){Ok(1.0)}
             else{
              match floats_from(word){
                Ok(val)=>{if val.len() == 0 {Err("Pan range is 0 - 1.")} else {
                  Ok(val[0])
                }
                }
                Err(err)=>Err("Invalid pan argument.")
              }
             }
          } else {
            Err("Invalid panorama.")
          }
        };

        if variant != Variant::Unknown{
          if variant != Variant::NoiseOsc{
            let oscillator = OscillatorNode::new(audio).unwrap();
            match variant{
              Variant::SinOsc=>oscillator.set_type(OscillatorType::Sine),
              Variant::SqrOsc=>oscillator.set_type(OscillatorType::Square),
              Variant::TriOsc=>oscillator.set_type(OscillatorType::Triangle),
              Variant::SawOsc=>oscillator.set_type(OscillatorType::Sawtooth),
              _=>todo!()
             }
             if expr.len() == 2{
              gain.gain().set_value(1.0);
              panner.pan().set_value(0.0);
              match inspect_freq(expr[1]){
                Ok(val)=>oscillator.frequency().set_value(val),
                Err(err)=>send_err("Invalid frequency value."),
              }
            } else if expr.len() == 3{
              match inspect_freq(expr[1]){
                Ok(val)=>oscillator.frequency().set_value(val),
                Err(err)=>send_err("Invalid frequency value."),
              }
              match inspect_gain(expr[2]){
                Ok(val)=>gain.gain().set_value(val),
                Err(err)=>match inspect_pan(expr[2]){
                      Ok(val)=>panner.pan().set_value(val * 2.0 - 1.0),
                      Err(err)=>send_err("Invalid second parameter. Only gain or pan is allowed"),
                         }
              }
            } else if expr.len() == 4{
              match inspect_freq(expr[1]){
                Ok(val)=>oscillator.frequency().set_value(val),
                Err(err)=>send_err("Invalid frequency value."),
              }
              match inspect_gain(expr[2]){
                Ok(val)=>gain.gain().set_value(val),
                Err(err)=>send_err("Invalid gain."),
              }
              match inspect_pan(expr[3]){
                Ok(val)=>panner.pan().set_value(val * 2.0 - 1.0),
                Err(err)=>send_err("Invalid pan."),
              }
            }
            oscillator.connect_with_audio_node(&gain);
            oscillator.connect_with_audio_node(&panner);
            gain.connect_with_audio_node(&audio.destination());
            panner.connect_with_audio_node(&audio.destination());
            oscillator.start();
          } else {
            /*----------NOISE--------------*/
            let mut painval:Option<f32> = None;
            let mut gainval:Option<f32> = None;
            if expr.len() == 2{
              match inspect_gain(expr[1]){
                Ok(g)=>{gain.gain().set_value(g); gainval = Some(g)},
                Err(err)=>match inspect_pan(expr[1]){
                      Ok(p)=>{panner.pan().set_value(p * 2.0 - 1.0); painval = Some(p)},
                      Err(err)=>send_err("Invalid second parameter. Only gain or pan is allowed"),
                         }
              }
            } else if expr.len() == 3{
              match inspect_gain(expr[1]){
                Ok(val)=>{gain.gain().set_value(val); gainval = Some(val)},
                Err(err)=>send_err("Invalid gain.")
              }
              match inspect_pan(expr[2]){
                Ok(val)=>{panner.pan().set_value(val * 2.0 - 1.0); painval = Some(val)},
                Err(err)=>send_err("Invalid pan.")
              }
            }
            if painval != None && gainval != None{
              use rand::prelude::*;
              let noise_buf = AudioBuffer::new(&AudioBufferOptions::new(2 * audio.sample_rate() as u32, audio.sample_rate())).unwrap();
              let output:Vec<f32> = noise_buf.get_channel_data(0).unwrap();
              for mut val in output{
              val = rand::thread_rng().gen::<f32>() * gainval.unwrap() * 2.0 - 1.0;
              }
              let mut options = AudioBufferSourceOptions::new();
              options.buffer(Some(&noise_buf));
              options.loop_(true);
              let buf_player = AudioBufferSourceNode::new_with_options(audio, &options).unwrap();
              buf_player.start();
            }
          }
        } else {
          send_err("Unknown audio oscillator.");
        }
      }
      fn prepare_visual(w: &str){
        let expr:Vec<&str> = w.split_whitespace().collect();
        let range = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
        let rgb = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap();
        let uv = Regex::new(r"\[(((0.(\d+)|1|0))|\-(0.(\d+)|1|0)),(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\]").unwrap();
        let rotate = Regex::new(r"rotate\((((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\)").unwrap();

        //let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
      // .dyn_into::<HtmlCanvasElement>().unwrap();
        //let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
         
        let mut visual:Variant = Variant::Unknown;
        let mut r:f32 = 0.0;
        let mut color:[f32; 3] = [0.0,0.0,0.0];
        let mut coords:Option<[f32; 2]> = None;
        let mut rotation:Option<f64> = None;
        let variant = get_variant(expr[0]);

        let mut inspect_color = |word: &str| -> Result<[f32; 3], &'static str>{
          if rgb.is_match(word){
            let channels = floats_from(word).unwrap();
            Ok([channels[0],channels[1],channels[2]])
          } 
          else if word == "red"{Ok([1.0,0.0,0.0])}
          else if word == "green"{Ok([0.0,1.0,0.0])}
          else if word == "blue"{Ok([0.0,0.0,1.0])}
          else if word == "yellow"{Ok([1.0,1.0,0.0])}
          else if word == "magenta"{Ok([1.0,0.0,1.0])}
          else if word == "cyan"{Ok([0.0,1.0,1.0])}
          else if word == "orange"{Ok([1.0,0.45,0.0])}
          else if word == "pink"{Ok([1.0,0.6,0.8])}
          else if word == "purple"{Ok([0.2,0.0,0.5])}
          else if word == "brown"{Ok([0.3,0.2,0.1])}
          else if word == "beige"{Ok([0.5,0.4,0.3])}
          else if word == "black"{Ok([0.0,0.0,0.0])}
          else if word == "white"{Ok([1.0,1.0,1.0])}
          else if word == "gray" || word == "grey"{Ok([0.3,0.3,0.3])}
          else {
            if variant != Variant::Screen{
              send_err("Invalid radius for the object.");
            } else {
              send_err("Invalid grayscale for the screen.");
            }
            Err("error")
          }
        };

        let mut inspect_range = |word: &str| -> Result<f32, &'static str>{
          if range.is_match(word){
            match String::from(word).parse::<f32>(){
              Ok(val)=>Ok(val),
              Err(err)=>Err("Error in parsing range.")
            }
            } else {
              Err("Error")
            }
        };
        let mut inspect_uv = |word: &str| -> Result<[f32; 2], &'static str>{
          if uv.is_match(word){
           let floats = floats_from(word).unwrap();
           Ok([floats[0],floats[1]])
          } else {
            Err("error")
          }
        };
        let mut inspect_rotate = |word: &str| -> Result<f64, &'static str>{
        if rotate.is_match(word){
          let speed = floats_from(word).unwrap();
          Ok(speed[0] as f64)
        }
        else {Err("error")}
        };

        match variant{
          Variant::Cube=>visual = Variant::Cube,
          Variant::Sphere=>visual = Variant::Sphere,
          Variant::Screen=>visual = Variant::Screen,
          _=>send_err("Unknown media."),
        }

          if variant != Variant::Unknown{
            if variant != Variant::Screen{
             /*---------- VARIANT - WHATEVER SHAPE ------------*/
             if expr.len() == 1 { render(visual, 1.0, [1.0,1.0,1.0], None, None, None); }
             else if expr.len() == 2 {
              match inspect_range(expr[1]){
                Ok(val)=>{r = val; render(visual, r, color, None, None, None);},
                Err(err)=>{
                  match inspect_color(expr[1]){
                    Ok(val)=>{color = val; render(visual, 1.0, color, None, None, None);},
                    Err(err)=>send_err("Invalid second parameter. Only radius or color are allowed."),
                  }
                },
              }
             }
             else if expr.len() == 3 {
              match inspect_range(expr[1]){
                Ok(val)=>{r = val;
                  match inspect_uv(expr[2]){
                    Ok(c)=>{coords = Some(c); render(visual, r, [1.0,1.0,1.0], coords, None, None);},
                    Err(err)=>{
                      match inspect_rotate(expr[2]){
                        Ok(rot)=>{rotation = Some(rot); render(visual, r, [1.0,1.0,1.0], None, rotation, None);},
                        Err(err)=>send_err("Invalid second parameter. Only UV or rotation allowed."),
                      }
                    },
                  }
                },
                Err(err)=>{
                  match inspect_color(expr[1]){
                    Ok(val)=>{color = val;
                      match inspect_uv(expr[2]){
                        Ok(c)=>{coords = Some(c); render(visual, 1.0, color, coords, None, None);},
                        Err(err)=>{
                          match inspect_rotate(expr[2]){
                            Ok(rot)=>{rotation = Some(rot); render(visual, r, color, None, rotation, None);},
                            Err(err)=>send_err("Invalid second parameter. Only UV or rotation allowed."),
                          }
                        },
                      }
                    },
                    Err(err)=>send_err("Invalid first parameter. Cannot proceed further."),
                  }
                },
              }
             }
             else if expr.len() == 4 {
              let mut first:Option<f32> = None;
              let mut second:Option<[f32; 3]> = None; 
              match inspect_range(expr[1]){
                Ok(val)=>first = Some(val),
                Err(err)=>send_err("To proceed, the first parameter must be a radius."),
              }
              match inspect_color(expr[2]){
                Ok(val)=>second = Some(val),
                Err(err)=>send_err("To proceed, the second parameter must be a color."),
              }
              if first != None && second != None{
                match inspect_uv(expr[3]){
                  Ok(val)=>{coords = Some(val); render(visual, first.unwrap(), second.unwrap(), coords, None, None);},
                  Err(err)=>{
                    match inspect_rotate(expr[3]){
                      Ok(val)=>{rotation = Some(val); render(visual, first.unwrap(), second.unwrap(), None, rotation, None);},
                      Err(err)=>send_err("Invalid third parameter. Only UV or rotation allowed."),
                    }
                  },
                }
              }
             }
             else if expr.len() == 5 {
              let mut first:Option<f32> = None;
              let mut second:Option<[f32; 3]> = None;
              let mut third:Option<[f32; 2]> = None;
              let mut fourth:Option<f64> = None;
              match inspect_range(expr[1]){
               Ok(val)=>first = Some(val),
               Err(err)=>send_err("To proceed, the first parameter must be a radius."),
              }
              match inspect_color(expr[2]){
               Ok(val)=>second = Some(val),
               Err(err)=>send_err("To proceed, the second parameter must be a color."),
              }
              match inspect_uv(expr[3]){
               Ok(val)=>third = Some(val),
               Err(err)=>send_err("To proceed, the third parameter must be UV coordinates."),
              }
              match inspect_rotate(expr[4]){
               Ok(val)=>fourth = Some(val),
               Err(err)=>send_err("To proceed, the fourth parameter must be rotation speed."),
              }
              if first != None && second != None && third != None && fourth != None{
                render(visual, first.unwrap(), second.unwrap(), third, fourth, None);
              }
             } else if expr.len() > 5{ send_err("Too many parameters for the shape."); }
            } else {
             /*---------- VARIANT - SCREEN ------------*/
             if expr.len() == 2 {
              match inspect_range(expr[1]){
                Ok(val)=>render(visual, 0.0, [r,r,r], None, None, None),
                Err(err)=>{
                  match inspect_color(expr[2]){
                   Ok(col)=>render(visual, 0.0, color, None, None, None),
                   Err(err)=>send_err("Invalid second parameter. Only grayscale or RGB allowed.")
                  }
                },
              }
             }
             else if expr.len() > 2 {send_err("Too many parameters for the screen.")}
             else if expr.len() == 1 {send_err("Describe grayscale or RGB.");}
            }
          }
      }

      pub fn interpret(input: &str, audio: &AudioContext){
      //at first, the error log must be cleaned
      send_err("");
      let medium = || -> Medium {get_medium(input)};
        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>prepare_visual(&input),
            Medium::Audio=>prepare_audio(&input, &audio),
            Medium::Mixed=>prepare_mixed(&input, &audio),
            Medium::Effect=>prepare_effect(&input),
            _=>todo!(),
          }
        } else {
          send_err("Unknown media.");
        }
     }