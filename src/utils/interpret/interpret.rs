use regex::Regex;
use web_sys::*;
use three_d::core::*;
use three_d::renderer::*;
use crate::utils::primitives::error::error::send_err;
use crate::utils::primitives::enums::enums::{FnType,Variant,Param,Medium,Channel,Sound};
use crate::utils::primitives::enums::enums::{get_variant,get_medium,get_param,get_sound};
use crate::utils::webgl::program::create_program;
use crate::utils::webgl::models::{model_with_pos, model_with_pos_indices, model_with_pos_normals, model_with_pos_indices_normals};
use crate::utils::primitives::generate::sphere::{sphere_vertices,sphere_indices,sphere_normals};
use crate::utils::webgl::screen_fft::{screen_fft,screen_fft_all};
use crate::utils::render::scissor::divide_canvas;
use crate::utils::primitives::generate::matrices::view_projection;
use crate::utils::render::viewport::set_viewport;
use crate::utils::webgl::uniforms::animate_uniform;
fn set_screen_color(context: &WebGl2RenderingContext, channels: [f32; 3]){
  context.clear_color(channels[0], channels[1], channels[2], 1.0);
  context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}
fn floats_from(word: &str)->Result<Vec<f32>,&'static str>{
  let mut floats:Vec<f32> = vec![];
  for float in Regex::new(r"((0\.[\d]+)|1|0)").unwrap().find_iter(word){
    match String::from(float.as_str()).parse::<f32>(){
      Ok(val)=>floats.push(val),
      Err(err)=>send_err("Invalid values, could not collect them."),
    }
  };
  if floats.len() == 0{
    Err("There are no floats in this parameter.")
  } else {Ok(floats) }
}
fn analyze_func(word: &str) -> FnType{
  if Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgb;}
  else if Regex::new(r"rgba\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgba;}
  else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
  else if Regex::new(r"(?!.*(rgb|rgba|fft))").unwrap().is_match(word){return FnType::Unknown;}
  else {return FnType::NotFunction;}
  }
fn create_visual(gl: &WebGl2RenderingContext, shape: Variant, range: f32, color: [f32; 3]){
    let program = create_program(gl,
    "
    attribute vec3 position;
    attribute vec3 normal;
    uniform mat4 view;
    uniform mat4 projection;
    varying vec3 normalForDot;
    void main(){
      gl_Position = view * projection * vec4(position, 1.0);
      normalForDot = normal;
    }
    ",
    "
    precision mediump float;
    uniform vec4 color;
    uniform vec3 lightDirection;
    varying vec3 normalForDot;
    void main(){
      vec3 normal = normalize(normalForDot);     
      float light = dot(normal, lightDirection);
      gl_FragColor = color;
      gl_FragColor.rgb *= light;
    }
    ");
    gl.link_program(&program);
    if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS) == false{
      send_err(gl.get_program_info_log(&program).unwrap().as_str());
    } else {
      gl.use_program(Some(&program));
    }
    let web_window = web_sys::window().unwrap();          
    let viewport = Viewport{
      width: web_window.inner_width().unwrap().as_f64().unwrap() as u32,
      height: web_window.inner_height().unwrap().as_f64().unwrap() as u32,
      x: 0,
      y: 0
      };
    let mut camera:Camera = Camera::new_perspective(
      viewport,
      vec3(1.0, 1.0, 3.0),
      vec3(0.0, 0.0, 0.0),
      vec3(0.0, 3.0, 0.0),
      degrees(45.0),
      0.1,
      1000.0
  );
  view_projection(&camera, &program, "view", "projection", &gl);
  gl.uniform4fv_with_f32_array(gl.get_uniform_location(&program, "color").as_ref(), &[color[0], color[1], color[2], 1.0]);
  //gl.uniform3fv_with_f32_array(gl.get_uniform_location(&program, "lightDirection").as_ref(), &[0.0,0.0,0.0]);
  match shape{
            Variant::Cube=>{
                let positions = vec![
                  //front face
                  -range, -range, range,
                  range, -range, range,
                  range, range, range,
                  -range, -range, range,
                  -range, range, range, 
                  range, range, range,
                  //back face
                  -range, -range, -range,
                  range, -range, -range,
                  range, range, -range,
                  -range, -range, -range,
                  -range, range, -range,
                  range, range, -range, 
                  //upper face
                  range, range, range,
                  -range, range, range, 
                  -range, -range, -range,
                  range, range, range,
                  -range, -range, -range,
                  range, -range, -range,
                  //bottom face
                  -range, -range, range,
                  range, -range, range,
                  -range, -range, -range,
                  -range, -range, range,
                  range, -range, -range,
                  -range, -range, -range,
                  //right face
                  range, -range, range,
                  range, range, range,
                  range, -range, -range,
                  range, range, range,
                  range, range, -range,
                  range, -range, -range,
                  //left face
                  -range, -range, range,
                  -range, range, range, 
                  -range, -range, -range,
                  -range, -range, range,
                  -range, -range, -range,
                  -range, range, -range   
                ];
                /*let cube_indices:Vec<u16> = vec![
                //front face
                0,1,2,0,3,2, 
                //back face  
                4,5,6,4,7,6, 
                //upper face
                2,3,4,2,4,5,
                //bottom face
                0,1,4,0,5,4, 
                //right face
                1,2,5,2,6,5,
                //left face 
                0,3,4,0,4,7
                ];*/
                let normals:Vec<f32> = vec![
                //front face
                0.0,0.0,1.0,
                0.0,0.0,1.0,
                0.0,0.0,1.0,
                0.0,0.0,1.0,
                0.0,0.0,1.0,
                0.0,0.0,1.0,
                //back face
                0.0,0.0,-1.0,
                0.0,0.0,-1.0,
                0.0,0.0,-1.0,
                0.0,0.0,-1.0,
                0.0,0.0,-1.0,
                0.0,0.0,-1.0,
                //upper face
                0.0,1.0,0.0,
                0.0,1.0,0.0,
                0.0,1.0,0.0,
                0.0,1.0,0.0,
                0.0,1.0,0.0,
                0.0,1.0,0.0,
                //bottom face
                0.0,-1.0,0.0,
                0.0,-1.0,0.0,
                0.0,-1.0,0.0,
                0.0,-1.0,0.0,
                0.0,-1.0,0.0,
                0.0,-1.0,0.0,
                //right face
                1.0,0.0,0.0,
                1.0,0.0,0.0,
                1.0,0.0,0.0,
                1.0,0.0,0.0,
                1.0,0.0,0.0,
                1.0,0.0,0.0,
                //left face
                -1.0,0.0,0.0,
                -1.0,0.0,0.0,
                -1.0,0.0,0.0,
                -1.0,0.0,0.0,
                -1.0,0.0,0.0,
                -1.0,0.0,0.0
                ];
                model_with_pos_normals(gl, &program, "position", "normal", &positions, &normals);
              },
            Variant::Sphere=>{
            let positions = sphere_vertices(30,range);
            let indices = sphere_indices(30,30);
            let normals = sphere_normals(30);
          /model_with_pos_indices_normals(gl, &program, "position", "normal", &positions, &normals, &indices);
            },
            _=>todo!(),
           }  
      }
      fn prepare_effect(w: &str, gl: &WebGl2RenderingContext){
      let expr:Vec<&str> = w.split_whitespace().collect();
      match get_variant(expr[0]){
      Variant::Multiplication=>{
      if expr.len() == 2{
      match String::from(expr[1]).parse::<u16>(){
      Ok(val)=>divide_canvas(gl, val, val),
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
        divide_canvas(gl, rows, columns);
      }
      }
      },
      _=>todo!(),
      }
      }
      fn prepare_mixed(w: &str, gl: &WebGl2RenderingContext, audio: &AudioContext){
      let expr:Vec<&str> = w.split_whitespace().collect();
        if Regex::new(r"screen rgb\(((lo|mi|hi)|(0.(\d+)|1|0)),((lo|mi|hi)|(0.(\d+)|1|0)),((lo|mi|hi)|(0.(\d+)|1|0))\)").unwrap().is_match(w){
          let param_regex = Regex::new(r"((lo|mi|hi)|(0.(\d+)|1|0))/g").unwrap();
          let float_regex = Regex::new(r"(0.(\d+)|1|0)").unwrap();
          let freq_regex = Regex::new(r"(lo|mi|hi)").unwrap();
          let mut param_index = -1;
          let mut channels:[f32; 3] = [0.0,0.0,0.0];
          for param in param_regex.find_iter(w){
           param_index += 1;
           if float_regex.is_match(param.as_str()){
           let channel = String::from(param.as_str()).parse::<f32>();
           match param_index{
             0=>{channels[0] = channel.unwrap()},
             1=>{channels[1] = channel.unwrap()},
             2=>{channels[2] = channel.unwrap()},
             _=>todo!(),
           }
           } else if freq_regex.is_match(param.as_str()){
           match param_index{
             0=>screen_fft(gl, audio, Channel::Red, get_sound(param.as_str()), 1.0),
             1=>screen_fft(gl, audio, Channel::Green, get_sound(param.as_str()), 1.0),
             2=>screen_fft(gl, audio, Channel::Blue, get_sound(param.as_str()), 1.0),
             _=>todo!(),
           }
           } else {
             send_err("Unknown parameters for the screen.");
           }
          }
      } else if Regex::new(r"screen (lo|mi|hi)").unwrap().is_match(w){
        screen_fft_all(gl, audio, get_sound(expr[1]), 1.0);
      }
    }
      fn create_osc(audio_context: &AudioContext, wave: Variant, freq: f32, gain: f32){
        let gain_node = GainNode::new(&audio_context).unwrap();
        gain_node.gain().set_value(gain);
        let osc = OscillatorNode::new(&audio_context).unwrap();
        osc.frequency().set_value(freq);
        match wave{
        Variant::SinOsc=>osc.set_type(OscillatorType::Sine),
        Variant::SawOsc=>osc.set_type(OscillatorType::Sawtooth),
        Variant::SqrOsc=>osc.set_type(OscillatorType::Square),
        Variant::TriOsc=>osc.set_type(OscillatorType::Triangle),
        _=>todo!(),
        }
        osc.connect_with_audio_node(&gain_node);
        gain_node.connect_with_audio_node(&audio_context.destination());
        osc.start();
        }
    
        fn create_noise(ctx: &AudioContext, gain: f32){
          /*use rand::prelude::*;
          let ctx = AudioContext::new().unwrap();
          let gain_node = GainNode::new(&ctx).unwrap();
          gain_node.gain().set_value(gain);
          let noise_buf = AudioBuffer::new(&AudioBufferOptions::new(2 * ctx.sample_rate() as u32, ctx.sample_rate())).unwrap();
          let mut output:Vec<f32> = noise_buf.get_channel_data(0).unwrap();
          for mut val in output{
           val = rand::thread_rng().gen::<f32>() - gain * 2.0;
          }
          let mut options = AudioBufferSourceOptions::new();
          options.buffer(Some(&noise_buf));
          options.loop_(true);
          let buf_player = AudioBufferSourceNode::new_with_options(&ctx, &options).unwrap();
          buf_player.start();*/
        }
    
      fn create_audio(audio_context: &AudioContext, first_word: &str, freq: f32, gain: f32){
        let variant = get_variant(first_word);
        if variant != Variant::Unknown {
          match variant{
            Variant::SinOsc=>create_osc(audio_context, Variant::SinOsc, freq, gain),
            Variant::SawOsc=>create_osc(audio_context, Variant::SawOsc, freq, gain),
            Variant::SqrOsc=>create_osc(audio_context, Variant::SqrOsc, freq, gain),
            Variant::TriOsc=>create_osc(audio_context, Variant::TriOsc, freq, gain),
          _=>todo!(),
          }
        } else {
          send_err("Unknown oscillator.")
        }
      }
      fn prepare_audio(w: &str, audio: &AudioContext){
        let words:Vec<&str> = w.split_whitespace().collect();
        if words.len() == 1{
          if get_variant(words[0]) != Variant::NoiseOsc{
            send_err("Missing frequency for oscillator.");
          } else {
            send_err("Missing gain for noise.");
          }
        }
        else if words.len() == 2{
          if get_variant(words[0]) != Variant::NoiseOsc{
            if Regex::new(r"([\d]+|[\d]+\.[\d]+)\*(0.(\d+)|1|0)").unwrap().is_match(words[1]){
              let freq_result = Regex::new(r"[\d]+\.[\d]+|[\d]+").unwrap().find(w);
              let gain_result = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap().find(w);
              let mut freq:f32=0.0;
              let mut gain:f32=0.0;
              if freq_result != None && gain_result != None{
                match String::from(freq_result.unwrap().as_str()).parse::<f32>(){
                  Ok(val)=>{freq = val;},
                  Err(err)=>send_err("Invalid frequency."),
                }
                match String::from(gain_result.unwrap().as_str()).parse::<f32>(){
                  Ok(val)=>{
                    if val <= 1.0{
                      gain = val;
                    } else {
                      send_err("Invalid gain.");
                    }
                  },
                  Err(err)=>send_err("Invalid gain."),
                }
                if freq != 0.0 && gain <= 1.0{
                  create_audio(audio, words[0], freq, gain);
                } else {
                  send_err("Invalid oscillator.")
                }
              } else if freq_result != None && gain_result == None{
                send_err("Gain must not be greater than 1.");
              }
            }
            else if Regex::new(r"([\d]+|[\d]+\.[\d]+)").unwrap().is_match(words[1]){
              let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
              let mut freq:f32=0.0;
              match String::from(freq_result.unwrap().as_str()).parse::<f32>(){
                Ok(val)=>{freq = val},
                Err(err)=>send_err("Invalid frequency."),
              }
              if freq != 0.0{
                create_audio(audio, words[0], freq, 1.0);
              }
              else{
                send_err("Invalid oscillator. Frequency must be always greater than zero.");
              }
            }
          } else {
            if Regex::new(r"(0.(\d+)|1|0)").unwrap().is_match(words[1]){
              let mut gain:f32=0.0;
             match String::from(words[1]).parse::<f32>(){
              Ok(val)=>{
                if val <= 1.0{
                gain = val;
              } else {
                send_err("Unknown parameters for the noise.");
              }},
              Err(err)=>send_err("Invalid gain."),
             }
             if gain != 0.0{
              create_noise(audio, gain);
             }
            } else {
             send_err("Unknown parameters for the noise.")
            }
          }
          } else {
            send_err("Too many parameters for audio.")
          }
      }
      fn prepare_visual(w: &str, gl: &WebGl2RenderingContext){
        let expr:Vec<&str> = w.split_whitespace().collect();
        let range = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
        let rgb = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap();
        let uv = Regex::new(r"\[(0.(\d+)|1|0),(0.(\d+)|1|0)\]").unwrap();
         
        if expr.len() == 1{
          match get_variant(expr[0]){
            Variant::Screen=>send_err("Missing grayscale or rgb for the screen."),
            Variant::Cube=>create_visual(gl, Variant::Cube, 1.0, [1.0,1.0,1.0]),
            Variant::Sphere=>create_visual(gl, Variant::Sphere, 1.0, [1.0,1.0,1.0]),
            _=>todo!(),
          }
        }
        else if expr.len() == 2{
            match get_variant(expr[0]){
              Variant::Screen=>{
                if Regex::new(r"screen (0.(\d+)|1|0)").unwrap().is_match(w){
                    if range.is_match(expr[1]){
                      match String::from(expr[1]).parse::<f32>(){
                        Ok(val)=>{
                          if val <= 1.0{
                            set_screen_color(gl, [val,val,val]);
                          }
                        },
                        Err(err)=>send_err("Invalid grayscale for the screen."),
                      }
                    }
                } else if Regex::new(r"screen rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(w){
                  if rgb.is_match(expr[1]){
                    let channels = floats_from(expr[1]);
                    match channels{
                      Ok(val)=>set_screen_color(gl, [val[0], val[1], val[2]]),
                      Err(err)=>send_err(err),
                    }
                  } else {
                    send_err("Invalid color function for the screen.");
                  }
                } else {
                  send_err("Unknown parameters for the screen.");
                }
              },
              Variant::Cube=>{
                if Regex::new(r"cube (0.(\d+)|1|0)").unwrap().is_match(w){
                  if range.is_match(expr[1]) {
                    match String::from(expr[1]).parse::<f32>(){
                      Ok(val)=>{
                        if val <= 1.0{
                          create_visual(gl, Variant::Cube, val, [1.0,1.0,1.0]);
                        } else {
                          send_err("Radius must not be greater than 1.")
                        }
                      },
                      Err(err)=>send_err("Invalid radius for the object."),
                    }
                  } else {
                    send_err("Radius must not be greater than 1.");
                  }
                } else if Regex::new(r"cube rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(w){
                  let color = rgb.find(w);
                  if color != None {
                   let channels = floats_from(expr[1]).unwrap();
                   create_visual(gl, Variant::Cube, 1.0, [channels[0],channels[1],channels[2]]);
                  } else {
                  send_err("Invalid rgb for the object.");
                  }
                } else {
                  send_err("Unknown parameters for the object.");
                }
              },
              Variant::Sphere=>{ 
                if Regex::new(r"sphere (0.(\d+)|1|0)").unwrap().is_match(w){
                  if range.is_match(expr[1]) {
                    match String::from(expr[1]).parse::<f32>(){
                    Ok(val)=>{
                      if val <= 1.0{
                        create_visual(gl, Variant::Sphere, val, [1.0,1.0,1.0]);
                      } else {
                        send_err("Radius must not be greater than 1.")
                      }
                    },
                    Err(err)=>send_err("Invalid radius for the object."),
                  }
                } else {
                  send_err("Radius must not be greater than 1.");
                }
              } else if Regex::new(r"sphere rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(w){
                if rgb.is_match(expr[1]) {
                  let channels = floats_from(expr[1]).unwrap();
                  create_visual(gl, Variant::Sphere, 1.0, [channels[0],channels[1],channels[2]]);
                } else {
                send_err("Invalid rgb for the object.");
                }
              } else {
                send_err("Unknown parameters for the object.");
              }},
              _=>todo!(),
            }
        }
        else if expr.len() == 3{
          match get_variant(expr[0]){
           Variant::Cube=>{},
           Variant::Sphere=>{},
           _=>todo!(),
          }
        } else if expr.len() > 2 || get_variant(expr[0]) == Variant::Screen{
         send_err("Too many parameters for the screen.");
        } else if expr.len() > 3 || get_variant(expr[0]) == Variant::Cube ||
        get_variant(expr[0]) == Variant::Sphere{
          send_err("Too many parameters for the object.");
        } else {
          todo!();
        }
      }

      pub fn interpret(input: &str, gl_ctx: &WebGl2RenderingContext, audio: &AudioContext){
      //at first, the error log must be cleaned
      send_err("");
      //individual words of the expression
      let words:Vec<&str> = input.split_whitespace().collect();
      let medium = || -> Medium {get_medium(input)};

        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>prepare_visual(&input, &gl_ctx),
            Medium::Audio=>prepare_audio(&input, &audio),
            Medium::Mixed=>prepare_mixed(&input, &gl_ctx, &audio),
            Medium::Effect=>prepare_effect(&input, &gl_ctx),
            _=>todo!(),
          }
        } else {
          send_err("Unknown media.");
        }
     }
     pub fn set(code: String, gl: WebGl2RenderingContext, audio: AudioContext){
     set_viewport();
    if code.contains(';'){
      let exprs = code.split(';');
      for expr in exprs{
       interpret(expr, &gl, &audio);
      }
    } else {
      interpret(&*code, &gl, &audio);
    }
    }