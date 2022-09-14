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

    //functions which will produce shapes and parts of them
    fn square_face_anticlockwise(pos:[Vector3<f32>; 4])->Vec<Vector3<f32>>{
      vec![pos[0], pos[1], pos[2], pos[0], pos[3], pos[2]]
    }

    fn create_model(ctx: &Context, shape: Variant, range: f32, color: Color){
      let mut vertices = VertexBuffer::new(ctx);
     match shape{
      Variant::Cube=>{
        vertices.fill::<f32>(&[]);
        /*let cube_pos:Vec<Vector3<f32>>=vec![
        ];*/     
       },
      Variant::Sphere=>{
        vertices.fill::<f32>(&[]);
       /*let angle = Rad::<f32>::full_turn();
        let longitude = angle / 40.0;
        let rotation = angle / 30.0;
        let mut x:f32 = 0.0;
        let mut y:f32 = 0.0;
        let mut z:f32 = 0.0;
        let mut sphere_pos:Vec<Vector3<f32>>=vec![];
        //rotations of longitudes
        for i in 1..30{
        z = Rad::sin((rotation * i as f32) * range);
          //positions of vertices creating the longitude
          for i in 1..40{
          y = Rad::sin((longitude * i as f32) * range);
          x = Rad::cos((longitude * i as f32) * range);
          sphere_pos.push(vec3(x,y,z));
          }
        }*/
      },
      _=>{}   
    }
    }
      
    #[derive(PartialEq, Copy, Clone)]
      pub struct Multiplication{
        rows: u32,
        columns: u32
      }
      #[wasm_bindgen]
      #[derive(PartialEq, Clone)]
      pub struct Audio{
       osc: Vec<OscillatorNode>,
       gain: Vec<GainNode>
      }
      #[wasm_bindgen]
      impl Audio{
        #[wasm_bindgen(constructor)]
        pub fn new(oscs: Vec<OscillatorNode>, gains: Vec<GainNode>) -> Audio{
          Audio{osc: oscs, gain: gains}
        }
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
      fn create_visual(gl_ctx: &Context, first_word: &str, range: f32, color: Vec<f32>){
        let variant = get_variant(first_word);
        if variant != Variant::Unknown {
           match variant{
            Variant::Cube=>create_model(gl_ctx, Variant::Cube, range, 
                Color::from_rgb_slice(&[color[0],color[1],color[2]])),
            Variant::Sphere=>create_model(gl_ctx, Variant::Sphere, range, 
                  Color::from_rgb_slice(&[color[0],color[1],color[2]])),
            _=>send_err("Not suitable for creating models."),
              }          
            }
        else {
         send_err("Unknown shape type.");
        }
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
      fn prepare_visual(w: &str, gl: &Context){
        let expr:Vec<&str> = w.split_whitespace().collect();
        let matches_float = Regex::new(r"(0.(\d+)|^1|0)").unwrap();
        let matches_rgb = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap();
        //let matches_uv = Regex::new(r"^\[(0.(\d+)|1|0),(0.(\d+)|1|0)\]").unwrap();
        match get_variant(expr[0]){
          Variant::Screen=>{
            if expr.len() == 1{
              send_err("Missing parameters for the screen.")
            }
            else if expr.len() == 2{
              if matches_float.is_match(expr[1]){
                match String::from(expr[1]).parse::<f32>(){
                    Ok(val)=>{
                      if val <= 1.0{
                    /*screen_color = Color::from_rgb_slice(&[val,val,val])*/
                      } else {
                        send_err("Unknown parameters for the screen.");                 
                       }
                      },
                    Err(err)=>send_err("Invalid grayscale for the screen."),
                  }
                } 
                else if matches_rgb.is_match(expr[1]){
                  match floats_from(expr[1]){
                    Ok(val)=>{/*screen_color = Color::from_rgb_slice(&[val[0], val[1], val[2]])*/},
                    Err(err)=>send_err("Invalid rgb for the screen."),
                  }
                } else {
                  send_err("Unknown parameters for the screen.");
                }
            }
            else if expr.len() > 2{
            send_err("Too many parameters for the screen.");
            }
            else {
             todo!();
            }
          },
          Variant::Cube | Variant::Sphere=>{
            if expr.len() == 1{
              create_visual(gl, expr[0], 1.0, vec![1.0,1.0,1.0]);
            }
            else if expr.len() == 2{
              if matches_float.is_match(expr[1]){
                match String::from(expr[1]).parse::<f32>(){
                    Ok(val)=>{
                      if val <= 1.0{
                        create_visual(gl, expr[0], val, vec![1.0,1.0,1.0]);
                      } else {
                        send_err("Unknown parameters for the object.");                 
                       }
                    },
                    Err(err)=>send_err("Invalid radius for the object."),
                  }
                } 
                else if matches_rgb.is_match(expr[1]){
                  match floats_from(expr[1]){
                    Ok(val)=>create_visual(gl, expr[0], 1.0, val),
                    Err(err)=>send_err("Invalid rgb for the object."),
                  }
                } else {
                  send_err("Unknown parameters for the object.");
                }
            } else if expr.len() == 3{
              if matches_float.is_match(expr[1]) && matches_rgb.is_match(expr[2]){
                let mut range:f32=-1.0;
                let mut color:Vec<f32> = vec![];
                match String::from(expr[1]).parse::<f32>(){
                  Ok(val)=>{range = val},
                  Err(err)=>send_err("One of parameters is not valid."),
                }
                match floats_from(expr[2]){
                  Ok(val)=>{color = val},
                  Err(err)=>send_err("One of parameters is not valid."),
              }
              if range != -1.0 && color.len() != 0{
                create_visual(gl, expr[0], range, color);
              } else {
                todo!();
              }
            }
          } if expr.len() > 3{
            send_err("Too many parameters for the object.")
            } else {
              todo!();
            }
          },
          _=>todo!(),
         }
      }
      fn interpret(input: &str, gl_ctx: &Context, audio: &AudioContext){
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
            _=>send_err("Not suitable."),
          }
        } else {
          send_err("Unknown media.");
        }
     }
    pub fn set(code: String, context: WebGl2RenderingContext, audio: AudioContext){
      let orig_context = core_context::from_gl_context(Arc::new(cont_context::from_webgl2_context(context))).unwrap();     
      if code.contains(';'){
        let exprs = code.split(';');
        for expr in exprs{
          interpret(expr, &orig_context, &audio);
        }
      } else {
        interpret(&*code, &orig_context, &audio);
      }
      let window = web_sys::window().unwrap();
      let viewport = Viewport{
        width: window.inner_width().unwrap().as_f64().unwrap() as u32,
        height: window.inner_height().unwrap().as_f64().unwrap() as u32,
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
    //mutable variables for changing the environment
    let mut channels:[f32; 4] = [0.0,0.0,0.0,1.0];
  //finally the window rendering
     window.render_loop(move |frame_input| { 
      let screen = frame_input.screen();
      camera.set_viewport(frame_input.viewport);
      screen.clear(ClearState::color(channels[0], channels[1], channels[2], channels[3]));
      FrameOutput::default()
    });
    }