use std::sync::Arc;
use regex::Regex;
use three_d::renderer::*;
use three_d::window::*;
use three_d::core::Context as core_context;
use three_d::context::Context as cont_context;
use three_d::core::{Program,buffer::*,prelude::Color};
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;

  #[derive(PartialEq)]
  enum Medium{
    Visuals,
    Audio,
    Effect,
    Unknown
  }
  #[derive(PartialEq)]
  enum Variant{
    Screen,
    Cube,
    Sphere,
    SinOsc,
    SawOsc,
    SqrOsc,
    TriOsc,
    NoiseOsc,
    Unknown
  }
  #[derive(PartialEq)]
  enum Param{
    Quantity,
    Range,
    Function,
    Realtime,
    Unknown
  }
  #[derive(PartialEq)]
    enum FnType{
      Rgb,
      Rgba,
      Fft,
      Unknown,
      NotFunction
    }
    #[derive(PartialEq)]
    enum Effect{
      Multiplication
    }

    fn get_medium(word: &str)->Medium{
    match word{
      "screen"=>Medium::Visuals,
      "cube"=>Medium::Visuals,
      "sphere"=>Medium::Visuals,
      "sin"=>Medium::Audio,
      "saw"=>Medium::Audio,
      "sqr"=>Medium::Audio,
      "tri"=>Medium::Audio,
      "rnd"=>Medium::Audio,
      "mul"=>Medium::Effect,
      _=>Medium::Unknown
    }
    }

    fn get_variant(word: &str)->Variant{
      match word{
      "screen"=>Variant::Screen,
      "cube"=>Variant::Cube,
      "sphere"=>Variant::Sphere,
      "sin"=>Variant::SinOsc,
      "saw"=>Variant::SawOsc,
      "sqr"=>Variant::SqrOsc,
      "tri"=>Variant::TriOsc,
      "rnd"=>Variant::NoiseOsc,
      _=>Variant::Unknown
      }
    }

    fn get_param(param: &str)->Param{
      let range:bool = Regex::new(r"(0.(\d+)|1|0)").unwrap().is_match(param);
      let qnt:bool = Regex::new(r"[0-9]+").unwrap().is_match(param);
      let func:bool = Regex::new(r"[a-zA-Z]+\([^\)]*\)?").unwrap().is_match(param);
      match range{
       true=>Param::Range,
       false=>
        match qnt{
          true=>Param::Quantity,
          false=>match func{
                true=>Param::Function,
                false=>Param::Unknown
               }
        },
      }
    }

    fn analyze_func(word: &str) -> FnType{
    if Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgb;}
    else if Regex::new(r"rgba\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgba;}
    else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
    else if Regex::new(r"(?!.*(rgb|rgba|fft))").unwrap().is_match(word){return FnType::Unknown;}
    else {return FnType::NotFunction;}
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

    fn send_err(error: &str){
      let document = web_sys::window().unwrap().document().unwrap();
      let error_p = document.get_element_by_id("error").unwrap();
      error_p.set_inner_html(error);
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

    fn create_noise(gain: f32){
      use rand::prelude::*;
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
      buf_player.start();
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

    //functions which will produce shapes and parts of them
    fn square_face_anticlockwise(pos:[Vector3<f32>; 4])->Vec<Vector3<f32>>{
      vec![pos[0], pos[1], pos[2], pos[0], pos[3], pos[2]]
    }
    fn set_screen_color(context: &WebGl2RenderingContext, channels: [f32; 3]){
      context.clear_color(channels[0], channels[1], channels[2], 1.0);
      context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
    fn create_visual(gl: &WebGl2RenderingContext, shape: Variant, range: f32, color: Vec<f32>){
           match shape{
            Variant::Cube=>{
            let cube_pos:Vec<Vector3<f32>> = vec![
            Vec3::new(range, range, -range),
            Vec3::new(-range, range, -range),
            Vec3::new(range, range, range),
            Vec3::new(-range, range, range),
            Vec3::new(range, range, range),
            Vec3::new(-range, range, -range),
            Vec3::new(-range, -range, -range),
            Vec3::new(range, -range, -range),
            Vec3::new(range, -range, range),
            Vec3::new(range, -range, range),
            Vec3::new(-range, -range, range),
            Vec3::new(-range, -range, -range),
            Vec3::new(range, -range, -range),
            Vec3::new(-range, -range, -range),
            Vec3::new(range, range, -range),
            Vec3::new(-range, range, -range),
            Vec3::new(range, range, -range),
            Vec3::new(-range, -range, -range),
            Vec3::new(-range, -range, range),
            Vec3::new(range, -range, range),
            Vec3::new(range, range, range),
            Vec3::new(range, range, range),
            Vec3::new(-range, range, range),
            Vec3::new(-range, -range, range),
            Vec3::new(range, -range, -range),
            Vec3::new(range, range, -range),
            Vec3::new(range, range, range),
            Vec3::new(range, range, range),
            Vec3::new(range, -range, range),
            Vec3::new(range, -range, -range),
            Vec3::new(-range, range, -range),
            Vec3::new(-range, -range, -range),
            Vec3::new(-range, range, range),
            Vec3::new(-range, -range, range),
            Vec3::new(-range, range, range),
            Vec3::new(-range, -range, -range),
            ];
            let cube_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&cube_buffer));
            gl.buffer_data_with_f64(WebGl2RenderingContext::ARRAY_BUFFER, cube_pos.len() as f64, WebGl2RenderingContext::DYNAMIC_DRAW);
            },
            Variant::Sphere=>{
            let rotation = Rad::<f32>::full_turn();
            let longitudes = 40; 
            let latitudes = 30;
            let mut sphere_pos:Vec<Vector3<f32>>=vec![];
            //triangles to the top
            for i in 1..longitudes{
              sphere_pos.push(vec3(0.0,range,0.0));
              sphere_pos.push(vec3(Rad::<f32>::sin((rotation / longitudes as f32) * ((i as f32)-1.0)) * range,
              Rad::<f32>::cos((rotation / latitudes as f32) * 1.0) * range,
              Rad::<f32>::sin((rotation / latitudes as f32) * 1.0) * range));
              sphere_pos.push(vec3(Rad::<f32>::sin((rotation / longitudes as f32) * i as f32) * range,
              Rad::<f32>::cos((rotation / latitudes as f32) * 1.0) * range,
              Rad::<f32>::sin((rotation / latitudes as f32) * 1.0) * range));
            }
            //latitudes and longitudes
            for i in 1..latitudes-1{
              let y_first = Rad::<f32>::cos((rotation / latitudes as f32) * i as f32) * range;
              let z_first = Rad::<f32>::sin((rotation / latitudes as f32) * i as f32) * range;
              let y_second = Rad::<f32>::cos((rotation / latitudes as f32) * (i+1) as f32) * range;
              let z_second = Rad::<f32>::sin((rotation / latitudes as f32) * (i+1) as f32) * range;
             
              for j in 0..longitudes{
                let x_first = Rad::<f32>::sin((rotation / longitudes as f32) * j as f32) * range;
                let x_second =Rad::<f32>::sin((rotation / longitudes as f32) * (j+1) as f32) * range;
                let array = [vec3(x_first,y_second,z_second),
                vec3(x_second,y_second,z_second),vec3(x_second,y_first,z_first),
                vec3(x_first,y_first,z_first)];
                sphere_pos.push(square_face_anticlockwise(array)[0]);
                sphere_pos.push(square_face_anticlockwise(array)[1]);
                sphere_pos.push(square_face_anticlockwise(array)[2]);
                sphere_pos.push(square_face_anticlockwise(array)[3]);
                sphere_pos.push(square_face_anticlockwise(array)[4]);
                sphere_pos.push(square_face_anticlockwise(array)[5]);
              }
            }
            //triangles to the bottom
            for i in 1..longitudes{
              sphere_pos.push(vec3(0.0,-range,0.0));
              sphere_pos.push(vec3(Rad::<f32>::sin((rotation / longitudes as f32) * ((i as f32)-1.0)) * range,
              Rad::<f32>::cos((rotation / latitudes as f32) * 1.0) * -range,
              Rad::<f32>::sin((rotation / latitudes as f32) * 1.0) * -range));
              sphere_pos.push(vec3(Rad::<f32>::sin((rotation / longitudes as f32) * i as f32) * range,
              Rad::<f32>::cos((rotation / latitudes as f32) * 1.0) * -range,
              Rad::<f32>::sin((rotation / latitudes as f32) * 1.0) * -range));
            }
            let sphere_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&sphere_buffer));
            gl.buffer_data_with_f64(WebGl2RenderingContext::ARRAY_BUFFER, sphere_pos.len() as f64, WebGl2RenderingContext::DYNAMIC_DRAW);   
            },
            _=>send_err("Not suitable for creating models."),
           }       
      }
      #[derive(PartialEq, Copy, Clone)]
      pub struct Multiplication{
        rows: u32,
        columns: u32
      }
      fn prepare_effect(w: &str){}
      fn prepare_audio(w: &str, audio: &AudioContext){
        let words:Vec<&str> = w.split_whitespace().collect();
        if words.len() == 1{
          if get_variant(words[0]) != Variant::NoiseOsc{
            send_err("Missing frequency for oscillator.")
          } else {
            send_err("Missing gain for noise.")
          }
        }
        else if words.len() == 2{
          if get_variant(words[0]) != Variant::NoiseOsc{
            if Regex::new(r"([\d]+|[\d]+\.[\d]+)\*(0.(\d+)|1|0)").unwrap().is_match(words[1]){
              let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
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
              create_noise(gain);
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
            Variant::Cube=>create_visual(gl, Variant::Cube, 1.0, vec![1.0,1.0,1.0]),
            Variant::Sphere=>create_visual(gl, Variant::Sphere, 1.0, vec![1.0,1.0,1.0]),
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
                          } else {
                            send_err("Grayscale must not be greater than 1.");
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
                          create_visual(gl, Variant::Cube, val, vec![1.0,1.0,1.0]);
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
                   let channels = floats_from(expr[1]);
                   create_visual(gl, Variant::Cube, 1.0, channels.unwrap());
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
                        create_visual(gl, Variant::Sphere, val, vec![1.0,1.0,1.0]);
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
                  let channels = floats_from(expr[1]);
                  create_visual(gl, Variant::Sphere, 1.0, channels.unwrap());
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
       /* let vertex_src = 
       r##"
        attribute vec4 aVertexPosition;
        uniform mat4 uProjectionMatrix;
        uniform mat4 uModelViewMatrix;

        void main(void){
         gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
        }
        "##
        ;
        let fragment_src = 
        r##"
        #ifdef GL_ES
        precision mediump float;
        #endif
        
        void main() {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
        "##
        ;
        let vertex_shader = gl.create_shader(WebGl2RenderingContext::VERTEX_SHADER).unwrap();
        let fragment_shader = gl.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&vertex_shader, vertex_src);
        gl.shader_source(&fragment_shader, fragment_src);
        gl.compile_shader(&vertex_shader);
        gl.compile_shader(&fragment_shader);
        let compile_status_v = gl.get_shader_parameter(&vertex_shader, WebGl2RenderingContext::COMPILE_STATUS).as_string();
        let compile_status_f = gl.get_shader_parameter(&fragment_shader, WebGl2RenderingContext::COMPILE_STATUS).as_string();
        let program = gl.create_program().unwrap();
        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);
        let link_status = gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_string();
    
          let program = gl.create_program().unwrap();
          gl.attach_shader(&program, &vertex_shader);
          gl.attach_shader(&program, &fragment_shader);
          gl.link_program(&program);
          let link_status = gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_string();
        
        gl.clear_depth(1.0);        
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);         
        gl.depth_func(WebGl2RenderingContext::LEQUAL); 
        */
      }

      fn interpret(input: &str, gl_ctx: &WebGl2RenderingContext, audio: &AudioContext){
      //at first, the error log must be cleaned
      send_err("");
      //individual words of the expression
      let mut words:Vec<&str> = input.split_whitespace().collect();
      let medium = || -> Medium {get_medium(words[0])};
      let variant = || -> Variant {get_variant(words[0])};
      let param = |w: &str| -> Param {get_param(w)};

        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>prepare_visual(&input, &gl_ctx),
            Medium::Audio=>prepare_audio(&input, &audio),
            Medium::Effect=>prepare_effect(&input),
            _=>todo!(),
          }
        } else {
          send_err("Unknown media.");
        }
     }
    pub fn set(code: String, context: WebGl2RenderingContext, audio: AudioContext){
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
    camera.set_viewport(viewport); 
      if code.contains(';'){
          let exprs = code.split(';');
          for expr in exprs{
           interpret(expr, &context, &audio);
          }
        } else {
          interpret(&*code, &context, &audio);
        }
    }
    pub fn start(){
      let document = web_sys::window().unwrap().document().unwrap();
      let canvas = document.get_element_by_id("graphics").unwrap()
      .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
      let textarea = document.get_element_by_id("input").unwrap()
      .dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
      let window:three_d::Window = three_d::Window::new(WindowSettings{
        title: String::from("Pulchra"),
        canvas: Some(canvas),
        borderless: true,
        ..Default::default()
     }).unwrap();
      let gl = window.gl();
      let mut camera:Camera = Camera::new_perspective(
       window.viewport(),
       vec3(1.0, 1.0, 3.0),
       vec3(0.0, 0.0, 0.0),
       vec3(0.0, 3.0, 0.0),
       degrees(45.0),
       0.1,
       1000.0
   );
  //finally the window rendering
     window.render_loop(move |frame_input| { 
      let screen = frame_input.screen();
      camera.set_viewport(frame_input.viewport);
      screen.clear(ClearState::color(0.0,0.0,0.0,1.0));
      FrameOutput::default()
    });
    }