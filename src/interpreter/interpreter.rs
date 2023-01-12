use js_sys::*;
use regex::Regex;
use wasm_bindgen::JsCast;
use web_sys::*;
use three_d::*;
use crate::canvas::scissor::divide_canvas;
use crate::error::error::send_err;
use crate::enums::enums::{Variant,Medium,Fft};
use crate::enums::enums::{get_variant,get_medium};
use crate::fft::fft_options::FftOptions;
use crate::fft::fft::fft;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
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

      //will be used for later...
      /*fn prepare_mixed(w: &str, audio: &AudioContext) -> Result<(Oscillator,FftOptions),()>{
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();
         let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
      let expr:Vec<&str> = w.split_whitespace().collect();
      let analyser = AnalyserNode::new(audio).unwrap();
    }*/

      fn prepare_audio(w: &str, audio: &AudioContext) -> Result<js_sys::Object, ()>{
        let expr:Vec<&str> = w.split_whitespace().collect();
        let freq_regex = Regex::new(r"[\d]+\.[\d]+|[\d]+").unwrap();
        let note_regex = Regex::new(r"((c|d|e|f|g|a|b)(\d)|(c|d|e|f|g|a|b)(#|b)(\d))").unwrap();
        let gain_regex = Regex::new(r"((0.(\d+)|1|0)").unwrap();
        let pan_regex = Regex::new(r"\[(((0.(\d+)|1|0))|(l)|(c)|(r))\]").unwrap();
        let variant = get_variant(expr[0]);

        let gain = GainNode::new(audio).unwrap();
        let panner = StereoPannerNode::new(audio).unwrap();
        const osc:js_sys::Object = js_sys::Object::new();

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
              panner.pan().set_value(0.5);
              match inspect_freq(expr[1]){
                Ok(val)=>{
                  oscillator.frequency().set_value(val); 
                  js_sys::Object::define_property(&osc, &JsValue::from(oscillator), &js_sys::Object::from(JsValue::from("oscNode")));
                  js_sys::Object::define_property(&osc, &JsValue::from(gain),  &js_sys::Object::from(JsValue::from("gainNode")));
                  js_sys::Object::define_property(&osc, &JsValue::from(panner), &js_sys::Object::from(JsValue::from("panner")));
                  js_sys::Object::define_property(&osc, &JsValue::from(val), &js_sys::Object::from(JsValue::from("freq")));
                  js_sys::Object::define_property(&osc, &JsValue::from(1.0), &js_sys::Object::from(JsValue::from("gain")));
                  js_sys::Object::define_property(&osc, &JsValue::from(0.5), &js_sys::Object::from(JsValue::from("pan")));
                  Ok(osc)                     
                },
                Err(err)=>{send_err("Invalid frequency value."); return Err(());},
              }             
            } else if expr.len() == 3{
              let mut freq:Option<f32> = None;
              let mut amp:Option<f32> = None;
              let mut pan:Option<f32> = None;
              match inspect_freq(expr[1]){
                Ok(val)=>{oscillator.frequency().set_value(val); freq = Some(val);},
                Err(err)=>send_err("Invalid frequency value."),
              }
              match inspect_gain(expr[2]){
                Ok(val)=>{gain.gain().set_value(val); amp = Some(val)},
                Err(err)=>match inspect_pan(expr[2]){
                      Ok(val)=>{panner.pan().set_value(val * 2.0 - 1.0); pan = Some(val);},
                      Err(err)=>send_err("Invalid second parameter. Only gain or pan is allowed"),
                         }
              }
              if freq != None && amp != None {
                js_sys::Object::define_property(&osc, &JsValue::from(oscillator), &js_sys::Object::from(JsValue::from("oscNode")));
                js_sys::Object::define_property(&osc, &JsValue::from(gain), &js_sys::Object::from(JsValue::from("gainNode")));
                js_sys::Object::define_property(&osc, &JsValue::from(panner), &js_sys::Object::from(JsValue::from("panner")));
                js_sys::Object::define_property(&osc, &JsValue::from(freq), &js_sys::Object::from(JsValue::from("freq")));
                js_sys::Object::define_property(&osc, &JsValue::from(amp), &js_sys::Object::from(JsValue::from("gain")));
                js_sys::Object::define_property(&osc, &JsValue::from(pan), &js_sys::Object::from(JsValue::from("pan")));
                Ok(osc)
              } else {Err(())}
              }
              else if expr.len() == 4{
              let mut freq:Option<f32> = None;
              let mut amp:Option<f32> = None;
              let mut pan:Option<f32> = None;
              match inspect_freq(expr[1]){
                Ok(val)=>{oscillator.frequency().set_value(val); freq = Some(val);},
                Err(err)=>send_err("Invalid frequency value."),
              }
              match inspect_gain(expr[2]){
                Ok(val)=>{gain.gain().set_value(val); amp = Some(val);},
                Err(err)=>send_err("Invalid gain."),
              }
              match inspect_pan(expr[3]){
                Ok(val)=>{panner.pan().set_value(val * 2.0 - 1.0); pan = Some(val);},
                Err(err)=>send_err("Invalid pan."),
              }
              if freq != None && amp != None && pan != None {
                js_sys::Object::define_property(&osc, &JsValue::from(oscillator), &js_sys::Object::from(JsValue::from("oscNode")));
                js_sys::Object::define_property(&osc, &JsValue::from(gain), &js_sys::Object::from(JsValue::from("gainNode")));
                js_sys::Object::define_property(&osc, &JsValue::from(panner), &js_sys::Object::from(JsValue::from("panner")));
                js_sys::Object::define_property(&osc, &JsValue::from(freq), &js_sys::Object::from(JsValue::from("freq")));
                js_sys::Object::define_property(&osc, &JsValue::from(amp), &js_sys::Object::from(JsValue::from("gain")));
                js_sys::Object::define_property(&osc, &JsValue::from(pan), &js_sys::Object::from(JsValue::from("pan")));
                Ok(osc)
              } else
                 {
             send_err("Incorrect parameters for the oscillator.");
            return Err(());
            }
              /*} else {
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
            }*/
          } else {todo!()}
        } else {send_err("Unknown media."); Err(())}
        } else {Err(())}
      }
      
      fn prepare_visual(w: &str) -> Result<js_sys::Object, ()>{
        let expr:Vec<&str> = w.split_whitespace().collect();
        let range = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
        let rgb = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap();
        let uv = Regex::new(r"\[(((0.(\d+)|1|0))|\-(0.(\d+)|1|0)),(((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\]").unwrap();
        let rotate = Regex::new(r"rotate\((((0.(\d+)|1|0))|\-(0.(\d+)|1|0))\)").unwrap();
         
        let mut visual:Variant = Variant::Unknown;
        const shape:js_sys::Object = js_sys::Object::new();
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
              send_err("Invalid radius for the object.");
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
        let mut inspect_rotate = |word: &str| -> Result<f32, &'static str>{
        if rotate.is_match(word){
          let speed = floats_from(word).unwrap();
          Ok(speed[0])
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
              if expr.len() == 1 { 
                const color:Array = Array::new();
                color.push(&JsValue::from(1.0)); color.push(&JsValue::from(1.0)); color.push(&JsValue::from(1.0));
                const coords:Array = Array::new();
                coords.push(&JsValue::from(0.0)); coords.push(&JsValue::from(0.0));
                js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                js_sys::Object::define_property(&shape, &JsValue::from(1.0), &js_sys::Object::from(JsValue::from("radius")));
                js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("color")));
                js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                js_sys::Object::define_property(&shape, &JsValue::NULL, &js_sys::Object::from(JsValue::from("rotation")));
                Ok(shape)
                }
                else if expr.len() == 2 {
                const color:Array = Array::new();
                const coords:Array = Array::new();
                coords.push(&JsValue::from(0.0)); coords.push(&JsValue::from(0.0));
                match inspect_range(expr[1]){
                  Ok(val)=>{
                    color.push(&JsValue::from(1.0)); color.push(&JsValue::from(1.0)); color.push(&JsValue::from(1.0));
                    js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                    js_sys::Object::define_property(&shape, &JsValue::from(val), &js_sys::Object::from(JsValue::from("radius")));
                    js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("color")));
                    js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                    js_sys::Object::define_property(&shape, &JsValue::NULL, &js_sys::Object::from(JsValue::from("rotation")));
                    Ok(shape)
                  }
                  Err(err)=>{
                    match inspect_color(expr[1]){
                      Ok(val)=>{
                        color.push(&JsValue::from(val[0])); color.push(&JsValue::from(val[1])); color.push(&JsValue::from(val[2]));
                       js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                        js_sys::Object::define_property(&shape, &JsValue::from(1.0), &js_sys::Object::from(JsValue::from("radius")));
                        js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("color")));
                        js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                        js_sys::Object::define_property(&shape, &JsValue::NULL, &js_sys::Object::from(JsValue::from("rotation")));
                        Ok(shape)
                      }
                      Err(err)=>{send_err("Invalid second parameter. Only radius or color are allowed."); return Err(());},
                    }
                  },
                }
               }
               else if expr.len() == 3 {  
                const color:Array = Array::new();
                const coords:Array = Array::new();      
                match inspect_range(expr[1]){
                  Ok(val)=>{ color.push(&JsValue::from(1.0)); color.push(&JsValue::from(1.0)); color.push(&JsValue::from(1.0));
                    match inspect_uv(expr[2]){
                      Ok(c)=>{
                        coords.push(&JsValue::from(c[0])); coords.push(&JsValue::from(c[1]));
                        js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                        js_sys::Object::define_property(&shape, &JsValue::from(val), &js_sys::Object::from(JsValue::from("radius")));
                        js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("color")));
                        js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                        js_sys::Object::define_property(&shape, &JsValue::NULL, &js_sys::Object::from(JsValue::from("rotation")));  
                        Ok(shape)
                      }
                      Err(err)=>{
                        match inspect_rotate(expr[2]){
                          Ok(rot)=>{
                            coords.push(&JsValue::from(0.0)); coords.push(&JsValue::from(0.0));
                            js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                            js_sys::Object::define_property(&shape, &JsValue::from(val), &js_sys::Object::from(JsValue::from("radius")));
                            js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("color")));
                            js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                            js_sys::Object::define_property(&shape, &JsValue::from(rot), &js_sys::Object::from(JsValue::from("rotation")));
                            Ok(shape)
                           },           
                          Err(err)=>{send_err("Invalid second parameter. Only UV or rotation allowed."); return Err(());},
                        }
                      }
                    }
                  },
                  Err(err)=>{match inspect_color(expr[1]){
                      Ok(val)=>{
                        color.push(&JsValue::from(val[0])); color.push(&JsValue::from(val[1])); color.push(&JsValue::from(val[2]));
                        match inspect_uv(expr[2]){
                          Ok(c)=>{
                            coords.push(&JsValue::from(c[0])); coords.push(&JsValue::from(c[1]));
                            js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                            js_sys::Object::define_property(&shape, &JsValue::from(1.0), &js_sys::Object::from(JsValue::from("radius")));
                            js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("color")));
                            js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                            js_sys::Object::define_property(&shape, &JsValue::NULL, &js_sys::Object::from(JsValue::from("rotation"))); 
                            Ok(shape)      
                          }         
                          Err(err)=>{
                            match inspect_rotate(expr[2]){
                              Ok(rot)=>{
                                coords.push(&JsValue::from(0.0)); coords.push(&JsValue::from(0.0));
                                js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                                js_sys::Object::define_property(&shape, &JsValue::from(1.0), &js_sys::Object::from(JsValue::from("radius")));
                                js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("color")));
                                js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                                js_sys::Object::define_property(&shape, &JsValue::from(rot), &js_sys::Object::from(JsValue::from("rotation")));
                                Ok(shape)                             
                              }           
                              Err(err)=>{send_err("Invalid second parameter. Only UV or rotation allowed."); return Err(());},
                            } 
                          },
                        } 
                      },
                      Err(err)=>{send_err("Invalid first parameter. Cannot proceed further."); return Err(());},
                      }
                    }
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
                const color:Array = Array::new();
                const coords:Array = Array::new();
                if first != None && second != None{
                  color.push(&JsValue::from(second.unwrap()[0])); color.push(&JsValue::from(second.unwrap()[1])); color.push(&JsValue::from(second.unwrap()[2]));
                  match inspect_uv(expr[3]){
                    Ok(val)=>{
                      coords.push(&JsValue::from(val[0])); coords.push(&JsValue::from(val[1])); 
                      js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                      js_sys::Object::define_property(&shape, &JsValue::from(first.unwrap()), &js_sys::Object::from(JsValue::from("radius")));
                      js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("radius")));
                      js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                      js_sys::Object::define_property(&shape, &JsValue::NULL, &js_sys::Object::from(JsValue::from("rotation"))); 
                      Ok(shape)                    
                    },
                    Err(err)=>{
                      match inspect_rotate(expr[3]){
                        Ok(val)=>{
                          coords.push(&JsValue::from(0.0)); coords.push(&JsValue::from(0.0));
                          js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                          js_sys::Object::define_property(&shape, &JsValue::from(first.unwrap()), &js_sys::Object::from(JsValue::from("radius")));
                          js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("radius")));
                          js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                          js_sys::Object::define_property(&shape, &JsValue::from(val), &js_sys::Object::from(JsValue::from("rotation")));
                          Ok(shape)  
                        },
                        Err(err)=>{send_err("Invalid third parameter. Only UV or rotation allowed."); return Err(());},                     
                         }
                        }
                      }
                    } else {return Err(());}
                  } 
               else if expr.len() == 5 {
                let mut first:Option<f32> = None;
                let mut second:Option<[f32; 3]> = None;
                let mut third:Option<[f32; 2]> = None;
                let mut fourth:Option<f32> = None;
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
                  const color:Array = Array::new();
                  const coords:Array = Array::new();
                  color.push(&JsValue::from(second.unwrap()[0])); color.push(&JsValue::from(second.unwrap()[1])); color.push(&JsValue::from(second.unwrap()[2]));
                  coords.push(&JsValue::from(third.unwrap()[0])); coords.push(&JsValue::from(third.unwrap()[1]));
                  js_sys::Object::define_property(&shape, &JsValue::from(visual.str()), &js_sys::Object::from(JsValue::from("type")));
                  js_sys::Object::define_property(&shape, &JsValue::from(first.unwrap()), &js_sys::Object::from(JsValue::from("radius")));
                  js_sys::Object::define_property(&shape, &JsValue::from(color), &js_sys::Object::from(JsValue::from("radius")));
                  js_sys::Object::define_property(&shape, &JsValue::from(coords), &js_sys::Object::from(JsValue::from("coords")));
                  js_sys::Object::define_property(&shape, &JsValue::from(fourth.unwrap()), &js_sys::Object::from(JsValue::from("rotation")));
                  Ok(shape)
                } else {return Err(());}
               } else if expr.len() > 5{ send_err("Too many parameters for the shape."); return Err(());}
               else {return Err(());}
            } else {Err(())}
            } else {return Err(());}
          }
  
      
      fn prepare_screen(input: &str) -> Result<Array, ()>{
        let expr:Vec<&str> = input.split_whitespace().collect();
        let range = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
        let rgb = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap();
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
              send_err("Invalid radius for the object.");
            Err("error")
          }
        };
         if expr.len() == 2 {
              match inspect_range(expr[1]){
                Ok(val)=>{
                let rgb:Array = Array::new();
                rgb.push(&JsValue::from(val));
                rgb.push(&&JsValue::from(val));
                rgb.push(&JsValue::from(val));
                Ok(rgb)
                },
                Err(err)=>{
                  match inspect_color(expr[2]){
                   Ok(col)=>{
                    let rgb:Array = Array::new();
                    rgb.push(&JsValue::from(col[0]));
                    rgb.push(&JsValue::from(col[1]));
                    rgb.push(&JsValue::from(col[2]));
                    Ok(rgb)
                   },
                   Err(err)=>{send_err("Invalid second parameter. Only grayscale or RGB allowed."); Err(())}
                  }
                },
              }
             }
             else if expr.len() > 2 {Err(())}
             else if expr.len() == 1 {Err(())}
             else {todo!()}
            }
      
       pub fn interpret(input: &str, audio: &AudioContext) -> js_sys::Object{
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();
        let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
        let window = three_d::Window::new(WindowSettings {
          canvas: Some(canvas),
         ..Default:: default()
         }).unwrap();
        /*pub struct Shape {
          variant: Variant,
          radius: f32,
          color: [f32; 3],
          coords: Option<[f32; 2]>,
          rotation: Option<f32>
         }
         #[derive(PartialEq)]
         pub struct Oscillator {
         osc: OscillatorNode,
         gain: GainNode,
         pan: StereoPannerNode,
         freq: f32,
         gain_val: f32,
         pan_val: f32
         }
         pub struct Engine {
         pub shapes: Vec<Shape>,
         pub oscillators: Vec<Oscillator>,
         pub fft_options: Option<Vec<FftOptions>>,
         pub screen_color: ClearState
         }*/
        const oscillatorsArray:Array = Array::new();
        const shapesArray:Array = Array::new();

        //at first, the error log must be cleaned
        send_err("");
        if input.contains(';'){
           if input.chars().last().unwrap() == ';'{
             String::from(input).pop();
          }
        for expr in input.split(';'){
         if get_medium(expr) != Medium::Unknown {
          match get_medium(expr){
            Medium::Visuals=>shapesArray.push(&prepare_visual(&expr).unwrap()),
            Medium::Background=>{},
            Medium::Audio=>oscillatorsArray.push(&prepare_audio(&expr, &audio).unwrap()),
            Medium::Effect=>prepare_effect(&expr),
            _=>todo!(),
          }
        } else {
          send_err("Unknown media.");
        }
          }
        }
        else {
              if get_medium(input) != Medium::Unknown {
          match get_medium(input){
            Medium::Visuals=>shapesArray.push(&prepare_visual(&input).unwrap()),
            Medium::Background=>{},
            Medium::Audio=>oscillatorsArray.push(&prepare_audio(&input, &audio).unwrap()),
            Medium::Effect=>prepare_effect(&input),
            _=>todo!(),
          }
        } else {
          send_err("Unknown media.");
        }
        }
         let engine = js_sys::Object::new();
         js_sys::Object::define_property(&engine, &JsValue::from(shapesArray), &js_sys::Object::from(JsValue::from("shapes")));
         js_sys::Object::define_property(&engine, &JsValue::from(oscillatorsArray), &js_sys::Object::from(JsValue::from("oscs")));
         js_sys::Object::define_property(&engine, &JsValue::from(Array::new_with_length(3)), &js_sys::Object::from(JsValue::from("clear")));
         engine
     }

     fn render_to_target(object: &dyn three_d::Object, camera: &Camera, light: &dyn Light, target: &RenderTarget){
      target.render(&camera, &[object], &[light]);
     }

     pub fn render(shapes: Vec<js_sys::Object>, clear_color: Option<ClearState>, opt: Option<FftOptions>){
       let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
       .dyn_into::<HtmlCanvasElement>().unwrap();
       let window = three_d::Window::new(WindowSettings {
        canvas: Some(canvas),
       ..Default::default()
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
       let light = DirectionalLight::new(&context, 0.5, Color::WHITE, &vec3(0.0,0.0,0.0));
       let mut clear_state = ClearState{..Default::default()};
       let mut cpu_mesh = CpuMesh{..Default::default()};
       if clear_color != None {clear_state = ClearState::color(1.0,1.0,1.0,1.0);} else {clear_state = clear_color.unwrap();}
           window.render_loop(move |frame_input: FrameInput|{
            /*if shapes.len() != 0{
              for shape in &shapes{
                match shape.variant{
                Variant::Cube=>cpu_mesh = CpuMesh::cube(),
                Variant::Sphere=>cpu_mesh = CpuMesh::sphere(30),
                _=>todo!(),
                }
                let mut object = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial{color: Color::from_rgb_slice(&[shape.color[0],shape.color[1],shape.color[2]]), ..Default::default()});
                if shape.coords != None {object.set_transformation(Mat4::from_translation(Camera::position_at_uv_coordinates(&camera, (shape.coords.unwrap()[0],shape.coords.unwrap()[1]))));}
                if shape.rotation != None {object.set_transformation(Mat4::from_angle_x(radians(frame_input.accumulated_time as f32 * shape.rotation.unwrap())));}
                render_to_target(&object, &camera, &light, &frame_input.screen().clear(clear_state));
              }
            } else {
              if opt != None {
                let options = opt.as_ref().unwrap();
                match options.param{
                  Fft::Red=>clear_state.red = Some(fft(&options.analyser, options.sound)),
                  Fft::Green=>clear_state.green = Some(fft(&options.analyser, options.sound)),
                  Fft::Blue=>clear_state.blue = Some(fft(&options.analyser, options.sound)),
                  _=>todo!(),
                }
              }
              frame_input.screen().clear(clear_state);
            }*/
           FrameOutput::default()
           });
 
     }

     pub fn play_audios(context: &AudioContext, oscs: &Vec<js_sys::Object>){
      /*for osc in oscs{
        let o = &osc.osc;
        let g = &osc.gain;
        let p = &osc.pan;
        o.connect_with_audio_node(&g);
        o.connect_with_audio_node(&p);
        g.connect_with_audio_node(&context.destination());
        p.connect_with_audio_node(&context.destination());
        o.start();
      }*/
     }

     pub fn compare_audios(first: &Vec<js_sys::Object>, second: &Vec<js_sys::Object>){
     /*if first.len() != second.len(){
      for o in 0..second.len(){
       if second[o] != first[o]{
        if second.len() > first.len(){
          let osc = &first[o].osc;
          osc.stop();
          osc.disconnect();
        } else if second.len() < first.len(){
          let osc = &second[o].osc;
          osc.start();
        }
       }
      }
     }*/
     }
