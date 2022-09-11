use std::sync::Arc;
use regex::Regex;
use three_d::renderer::*;
use three_d::window::*;
use three_d::core::Context as core_context;
use three_d::context::Context as cont_context;
use three_d::core::prelude::Color;
use web_sys::*;
use wasm_bindgen::JsCast;

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

    fn create_osc(audio_context: &AudioContext, wave: Variant, freq: f32, gain: f32) -> Result<(OscillatorNode,GainNode),&'static str>{
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
    osc.stop();
    osc.start();
    Ok((osc,gain_node))
    }

    fn create_noise(gain: f32) ->Result<(AudioBufferSourceNode, GainNode),&'static str>{
      use rand::prelude::*;
      let ctx = AudioContext::new().unwrap();
      let gain_node = GainNode::new(&ctx).unwrap();
      gain_node.gain().set_value(gain);
      let noise_buf = AudioBuffer::new(&AudioBufferOptions::new(2 * ctx.sample_rate() as u32, ctx.sample_rate())).unwrap();
      let mut output:Vec<f32> = noise_buf.get_channel_data(0).unwrap();
      for mut val in output{
       val = rand::thread_rng().gen::<f32>() - 1.0 * 2.0;
      }
      let mut options = AudioBufferSourceOptions::new();
      options.buffer(Some(&noise_buf));
      options.loop_(true);
      let buf_player = AudioBufferSourceNode::new_with_options(&ctx, &options).unwrap();
      buf_player.connect_with_audio_node(&gain_node);
      gain_node.connect_with_audio_node(&ctx.destination());
      buf_player.stop();
      buf_player.start();
      Ok((buf_player,gain_node))
    }

    fn produce_cube_face_anticlockwise(pos:[Vector3<f32>; 4])->Vec<Vector3<f32>>{
      vec![pos[0], pos[1], pos[2], pos[0], pos[2], pos[3]]
    }
    fn produce_longitude()->Vec<Vector3<f32>>{
      vec![]
    }

    fn create_model(ctx: &Context, shape: Variant, range: f32, color: Color) -> Result<Box<dyn Object>,&'static str>{
     match shape{
      Variant::Cube=>{
        /*let cube_pos:Vec<Vector3<f32>>=vec![
        ];*/
        Ok(Box::new(Gm::new(Mesh::new(ctx, &CpuMesh::cube()), ColorMaterial{color: color, ..Default::default()})))
      },
      Variant::Sphere=>{
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
        Ok(Box::new(Gm::new(Mesh::new(ctx, &CpuMesh::sphere(40)), ColorMaterial{color: color, ..Default::default()})))

      },
      _=>Err("Not suitable for 3d modelling."),
     }
    }
      
    #[derive(PartialEq, Copy, Clone)]
      pub struct Multiplication{
        rows: u32,
        columns: u32
      }
        
      fn interpret(input: &str, gl_ctx: &Context, audio: &AudioContext){
      let mut screen_color:Color = Color::new_opaque(0,0,0); 
      //individual words of the expression
      let mut words:Vec<&str> = input.split_whitespace().collect();
      let medium = || -> Medium {get_medium(words[0])};
      let variant = || -> Variant {get_variant(words[0])};
      let param = |w: &str| -> Param {get_param(w)};

      let mut create_effect = |effect: Effect| {

      };

      let mut create_audio = |freq: f32, gain: f32| {
        if variant() != Variant::Unknown {
          match variant(){
            Variant::SinOsc=>{
              let osc = create_osc(&audio, Variant::SinOsc, freq, gain);
            },
            Variant::SawOsc=>{
              let osc = create_osc(&audio, Variant::SawOsc, freq, gain);
            },
            Variant::SqrOsc=>{
              let osc = create_osc(&audio, Variant::SqrOsc, freq, gain);
            },
            Variant::TriOsc=>{
              let osc = create_osc(&audio, Variant::TriOsc, freq, gain);
                },
          _=>todo!(),
          }
        } else {
        }
      };

      let mut create_visual = |range: f32, color: Vec<f32>| {
        if variant() != Variant::Unknown {
        match variant(){
          Variant::Cube=>{
            let model = create_model(gl_ctx, Variant::Cube, range, 
              Color::from_rgb_slice(&[color[0],color[1],color[2]])).unwrap();
            },
          Variant::Sphere=>{
            let model = create_model(gl_ctx, Variant::Sphere, range, 
              Color::from_rgb_slice(&[color[0],color[1],color[2]])).unwrap();
            },
          _=>send_err("Not suitable for creating models."),
        }
      } else {
       send_err("Unknown shape type.")
      }
      };

      let mut prepare_effect = |w: &str|{
        let stat:Vec<&str> = w.split_whitespace().collect();
        if stat[0] == "mul"{
          if stat.len() == 2{
          let uint = String::from(stat[1]).parse::<u32>();
          match uint{
            Ok(val)=>{},//multiplications.push(Multiplication{rows: val, columns: val}),
            Err(err)=>send_err("Invalid mul parameter."),
          }
          }
          else if stat.len() == 3{
            let mut result_r:u32 = 0;
            let mut result_c:u32 = 0;
            let r = String::from(stat[1]).parse::<u32>();
            let c = String::from(stat[2]).parse::<u32>();
            match r{
              Ok(val)=>{result_r = val},
              Err(err)=>send_err("Invalid number of rows to repeat.")
            }
            match c{
              Ok(val)=>{result_c = val},
              Err(err)=>send_err("Invalid number of columns to repeat.")
            }
            if result_r != 0 && result_c != 0{
             // multiplications.push(Multiplication{rows: result_r, columns: result_c});
            }
          }
          else {send_err("Too many or little parameters for mul.")}
        }
        
      };

      let mut prepare_audio = |w: &str| {
        if Regex::new(r"(sin|saw|sqr|tri) ([\d]+|[\d]+\.[\d]+) \* (0.(\d+)|1|0)").unwrap().is_match(w) ||
        Regex::new(r"/(sin|saw|sqr|tri) ([\d]+|[\d]+\.[\d]+)\*(0.(\d+)|1|0)/").unwrap().is_match(w){
        let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
        let gain_result = Regex::new(r"(0\.[\d]+)|1|0").unwrap().find(w);
        let mut freq:f32=0.0;
        let mut gain:f32=0.0;
        if freq_result != None && gain_result != None{
          match String::from(freq_result.unwrap().as_str()).parse::<f32>(){
            Ok(val)=>{freq = val},
            Err(err)=>send_err("Invalid frequency."),
          }
          match String::from(gain_result.unwrap().as_str()).parse::<f32>(){
            Ok(val)=>{gain = val},
            Err(err)=>send_err("Invalid gain."),
          }
          if freq != 0.0 && gain != 0.0{
             if gain > 1.0{send_err("Gain must not be greater than 1.")} 
             else {create_audio(freq, gain);}
          } else {
            send_err("Invalid oscillator.")
          }
        }
        } else if Regex::new(r"(sin|sqr|saw|tri) ([\d]+|[\d]+\.[\d]+)").unwrap().is_match(w)  {
          let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
          let mut freq:f32=0.0;
          match String::from(freq_result.unwrap().as_str()).parse::<f32>(){
            Ok(val)=>{freq = val},
            Err(err)=>send_err("Invalid frequency."),
          }
          if freq != 0.0{create_audio(freq, 1.0);}else{
            send_err("Invalid oscillator. Frequency must be always greater than zero.");}
        } else if Regex::new(r"rnd (0.(\d+)|1)").unwrap().is_match(w) {
          let gain_result = Regex::new(r"(0\.[\d]+)|1").unwrap().find(w);
          let mut gain:f32=0.0;
          match String::from(gain_result.unwrap().as_str()).parse::<f32>(){
            Ok(val)=>{gain = val},
            Err(err)=>send_err("Invalid gain."),
          }
          if gain > 1.0{send_err("Gain must not be greater than 1.")}else{
            create_noise(gain);
          }
        }
         else {
          send_err("Unknown type of audio.")
        }
      };

      let mut prepare_visual = |w: &str| {
        let expr:Vec<&str> = w.split_whitespace().collect();
        let matches_float = Regex::new(r"^(0.(\d+)|1|0)").unwrap();
        let matches_rgb = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap();
        //let matches_uv = Regex::new(r"^\[(0.(\d+)|1|0),(0.(\d+)|1|0)\]").unwrap();
        match variant(){
          Variant::Screen=>{
            if expr.len() == 1{
              send_err("Missing parameters for the screen.")
            }
            else if expr.len() == 2{
              if matches_float.is_match(expr[1]){
                match String::from(expr[1]).parse::<f32>(){
                    Ok(val)=>{screen_color = Color::from_rgb_slice(&[val,val,val])},
                    Err(err)=>send_err("Invalid grayscale for the screen."),
                  }
                } 
                else if matches_rgb.is_match(expr[1]){
                  match floats_from(expr[1]){
                    Ok(val)=>{screen_color = Color::from_rgb_slice(&[val[0], val[1], val[2]])},
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
              create_visual(1.0, vec![1.0,1.0,1.0]);
            }
            else if expr.len() == 2{
              if matches_float.is_match(expr[1]){
                match String::from(expr[1]).parse::<f32>(){
                    Ok(val)=>create_visual(val, vec![1.0,1.0,1.0]),
                    Err(err)=>send_err("Invalid grayscale for the object."),
                  }
                } 
                else if matches_rgb.is_match(expr[1]){
                  match floats_from(expr[1]){
                    Ok(val)=>create_visual(1.0, val),
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
                create_visual(range, color);
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
        };

      let mut control_media = || {
        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>prepare_visual(&input),
            Medium::Audio=>prepare_audio(&input),
            Medium::Effect=>prepare_effect(&input),
            _=>send_err("Not suitable."),
          }
        } else {
          send_err("Unknown media.");
        }
      };
        control_media();
      }
    
    pub fn set(code: String, context: WebGl2RenderingContext, audio: AudioContext){
      if &*code != "" {
        send_err("");
        let orig_context = core_context::from_gl_context(Arc::new(cont_context::from_webgl2_context(context))).unwrap();
      if code.contains(';'){
        let exprs = code.split(';');
        for expr in exprs{
          interpret(expr, &orig_context, &audio);
        }
      } else {
        interpret(&*code, &orig_context, &audio);
      }
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
      /*if muls_len != 0{
        for m in 0..muls_len{
          let rows:u32 = muls[m].rows;
                  let columns:u32 = muls[m].columns;
                  let w = frame_input.viewport.width/rows;
                  let h = frame_input.viewport.height/columns;
                  for i in 0..(rows-1){
                    let x = (w*i) as i32;
                    for i in 1..columns{
                    let y = (h*i) as i32;
                    let scissor_box = ScissorBox{
                      height: h,
                      width: w,
                      x: x,
                      y: y
                    };
              if objs_len != 0{
                for o in 0..objs_len{
                  screen.render_partially(scissor_box, &camera, &[&*objs[o]], &[]);
                }
              }
        }
      }
      }
    } else {
      if objs_len != 0{
        for o in 0..objs_len{
          screen.render(&camera, &[&*objs[o]], &[]);
        }
      }
      }
   */
      FrameOutput::default()
    });
    }

    //just some models for testing
    /*let model = Gm::new(Mesh::new(&gl, 
    &CpuMesh::cube()), 
      PositionMaterial{..Default::default()});
      let dist:f32=1.8;
  let custom = Gm::new(Mesh::new(&gl,
     &CpuMesh{
      positions: Positions::F32(vec![
      vec3(0.0,0.0,0.0),
      vec3(dist,0.0,-dist),
      vec3(-dist,0.0,-dist),
      vec3(dist,0.0,-dist),
      vec3(-dist,0.0,-dist),
      vec3(0.0,dist/2.0,-dist)
      ]), ..Default::default()}),
      PositionMaterial{..Default::default()});*/