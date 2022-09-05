use regex::Regex;
use three_d::renderer::*;
use three_d::window::*;
use three_d::core::prelude::Color;
use wasm_bindgen::JsValue;
use web_sys::*;
use wasm_bindgen::{JsCast,prelude::wasm_bindgen,JsObject};

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
      for param in Regex::new(r"((0\.[\d]+)|1)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let int = float.iter().collect::<String>().parse::<u8>();
        let value = float.iter().collect::<String>().parse::<f32>();
        match value{
          Ok(v)=>floats.push(v),
          Err(err)=>{
            match int{
            Ok(v)=>floats.push(v as f32),
            Err(err)=>{return Err("Invalid values.")}
            }
          },
        }
      };
      Ok(floats)
    }

    fn send_err(error: &str){
      let document = web_sys::window().unwrap().document().unwrap();
      let error_p = document.get_element_by_id("error").unwrap();
      error_p.set_inner_html(error);
    }

    fn create_osc(ctx: &AudioContext, wave: Variant, freq: u32, gain: f32) -> Result<(Option<OscillatorNode>,Option<AudioBufferSourceNode>,GainNode),&'static str>{
    let gain_node = GainNode::new(ctx).unwrap();
    gain_node.gain().set_value(gain);
    if wave != Variant::NoiseOsc{
    let osc = OscillatorNode::new(ctx).unwrap();
    osc.frequency().set_value(freq as f32);
    match wave{
    Variant::SinOsc=>osc.set_type(OscillatorType::Sine),
    Variant::SawOsc=>osc.set_type(OscillatorType::Sawtooth),
    Variant::SqrOsc=>osc.set_type(OscillatorType::Square),
    _=>todo!(),
    }
    Ok((Some(osc),None,gain_node))
    } else {
      use rand::prelude::*;
      let noise_buf = AudioBuffer::new(&AudioBufferOptions::new(2 * ctx.sample_rate() as u32, ctx.sample_rate())).unwrap();
      let mut output:Vec<f32> = noise_buf.get_channel_data(0).unwrap();
      for mut val in output{
       val = rand::thread_rng().gen::<f32>() - 1.0 * 2.0;
      }
      let mut options = AudioBufferSourceOptions::new();
      options.buffer(Some(&noise_buf));
      options.loop_(true);
      let buf_player = AudioBufferSourceNode::new_with_options(ctx, &options);
      match buf_player{
        Ok(val) =>{return Ok((None,Some(val),gain_node))},
        Err(err)=>{return Err("Invalid buffer player.")},
      }
    }
    }

    fn create_model(ctx: &Context, shape: Variant, range: f32, color: Color) -> Result<Box<dyn Object>,&'static str>{
     match shape{
      Variant::Cube=>{
        let cube_pos:Vec<Vector3<f32>>=vec![
        vec3(range*(-1.0),range*(-1.0),0.0),vec3(range,range*(-1.0),0.0),
        vec3(range,range,0.0),vec3(range*(-1.0),range,0.0),
        vec3(range*(-1.0),range*(-1.0),range),vec3(range,range*(-1.0),range),
        vec3(range,range,range),vec3(range*(-1.0),range,range)];
        Ok(Box::new(Gm::new(Mesh::new(ctx, &CpuMesh{positions: Positions::F32(cube_pos), colors: Some(vec![color]),
          ..Default::default()}), ColorMaterial::default())))
      },
      Variant::Sphere=>{
        let angle = Rad::<f32>::full_turn();
        let longitude = angle / 20.0;
        let rotation = angle / 30.0;
        let mut x:f32 = 0.0;
        let mut y:f32 = 0.0;
        let mut z:f32 = 0.0;
        let mut sphere_pos:Vec<Vector3<f32>>=vec![];
        //rotations of longitudes
        for i in 1..30{
        z = Rad::sin((rotation * i as f32) * range);
          //positions of vertices creating the longitude
          for i in 1..20{
          y = Rad::sin((longitude * i as f32) * range);
          x = Rad::cos((longitude * i as f32) * range);
          sphere_pos.push(vec3(x,y,z));
          }
        }
        Ok(Box::new(Gm::new(Mesh::new(ctx, &CpuMesh{positions: Positions::F32(sphere_pos), colors: Some(vec![color]),
          ..Default::default()}), ColorMaterial::default())))
      },
      _=>Err("Not suitable for 3d modelling."),
     }
    }
      
    #[derive(PartialEq, Copy, Clone)]
      pub struct Multiplication{
        rows: u32,
        columns: u32
      }
      pub struct Input{
        shapes: Vec<Box<dyn Object>>,
        colors: Vec<Color>,
        oscs: Vec<OscillatorNode>,
        noises: Vec<AudioBufferSourceNode>,
        osc_amps: Vec<GainNode>,
        noise_amps: Vec<GainNode>,
        screen_color: Color,
        muls: Vec<Multiplication>
      }
      impl Input{
        pub fn new(shapes:Vec<Box<dyn Object>>, colors:Vec<Color>, 
          oscs:Vec<OscillatorNode>, noises:Vec<AudioBufferSourceNode>, osc_amps:Vec<GainNode>, 
          noise_amps:Vec<GainNode>,screen_color:Color, muls:Vec<Multiplication>)->Self{
          Input{shapes: shapes, colors: colors, oscs: oscs, noises: noises, osc_amps: osc_amps,
          noise_amps: noise_amps, screen_color: screen_color, muls: muls}
        }
      }
        
      fn interpret(input: &str, audio_ctx: &AudioContext, gl_ctx: &Context)->Input{
      let mut Shapes:Vec<Box<dyn Object>> = vec![];
      let mut Colors:Vec<Color> = vec![];
      let mut Oscillators:Vec<OscillatorNode> = vec![];
      let mut Noises:Vec<AudioBufferSourceNode> = vec![];
      let mut Oscamps:Vec<GainNode>=vec![];
      let mut Noiseamps:Vec<GainNode>=vec![];
      let mut screen_color:Color = Color::new_opaque(0,0,0); 
      let mut multiplications:Vec<Multiplication> = vec![];

      //an expression as a string
      let mut expr:&str = "";
      //individual words of the expression
      let mut words:Vec<&str> = expr.split_whitespace().collect();
      let medium = || -> Medium {get_medium(words[0])};
      let variant = || -> Variant {get_variant(words[0])};
      let param = |w: &str| -> Param {get_param(w)};

      let mut create_effect = |effect: Effect| {

      };

      let mut create_audio = |freq: u32, gain: f32| {
        if variant() != Variant::Unknown {
          match variant(){
            Variant::SinOsc=>{
              let osc = create_osc(audio_ctx, Variant::SinOsc, freq, gain);
              match osc{
                Ok(val) => {
                  if val.0 != None && val.1 == None{
                    Oscillators.push(val.0.unwrap()); Oscamps.push(val.2)
                  }
                },
                Err(err) => send_err(err),
              }
            },
            Variant::SawOsc=>{
              let osc = create_osc(audio_ctx, Variant::SawOsc, freq, gain);
              match osc{
                Ok(val)=>{     
                  if val.0 != None && val.1 == None{
                  Oscillators.push(val.0.unwrap()); Oscamps.push(val.2)
                }
               },
                Err(err)=>send_err(err),
              }
            },
            Variant::SqrOsc=>{
              let osc = create_osc(audio_ctx, Variant::SqrOsc, freq, gain);
              match osc{
                Ok(val)=>{
                  if val.0 != None && val.1 == None{
                    Oscillators.push(val.0.unwrap()); Oscamps.push(val.2)
                  } 
                },
                Err(err)=>send_err(err),
              }
            },
            Variant::NoiseOsc=>{
              let osc = create_osc(audio_ctx, Variant::NoiseOsc, freq, gain);
              match osc{
                Ok(val)=>{
                  if val.1 != None && val.0 == None{
                    Noises.push(val.1.unwrap()); Noiseamps.push(val.2)
                  }
                },
                Err(err)=>send_err(err),
              }
            },
            _=>send_err("Not suitable for audio."),
          }
        } else {
         send_err("Unknown audio type to play.")
        }
      };

      let mut create_visual = |range: f32, color: Vec<f32>| {
        if variant() != Variant::Unknown {
        match variant(){
          Variant::Screen=>screen_color = Color::from_rgb_slice(&[color[0],color[1],color[2]]),
          Variant::Cube=>{
            let model = create_model(gl_ctx, Variant::Cube, range, 
              Color::from_rgb_slice(&[color[0],color[1],color[2]]));
              match model{
                Ok(val)=>Shapes.push(val),
                Err(err)=>send_err(err),
              }
            },
          Variant::Sphere=>{
            let model = create_model(gl_ctx, Variant::Sphere, range, 
              Color::from_rgb_slice(&[color[0],color[1],color[2]]));
              match model{
                Ok(val)=>Shapes.push(val),
                Err(err)=>send_err(err),
              }
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
            Ok(val)=>multiplications.push(Multiplication{rows: val, columns: val}),
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
              multiplications.push(Multiplication{rows: result_r, columns: result_c});
            }
          }
          else {send_err("Too many or little parameters for mul.")}
        }
        
      };

      let mut prepare_audio = |w: &str| {
        if Regex::new(r"(sin|saw|sqr) ([\d]+) \* (0.(\d+)|1|0)").unwrap().is_match(w) ||
        Regex::new(r"(sin|saw|sqr) ([\d]+)\*(0.(\d+)|1|0)").unwrap().is_match(w){
        let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
        let gain_result = Regex::new(r"(0\.[\d]+)|1").unwrap().find(w);
        let mut freq:u32=0;
        let mut gain:f32=0.0;
        if freq_result != None && gain_result != None{
          match String::from(freq_result.unwrap().as_str()).parse::<u32>(){
            Ok(val)=>{freq = val},
            Err(err)=>send_err("Invalid frequency."),
          }
          match String::from(gain_result.unwrap().as_str()).parse::<f32>(){
            Ok(val)=>{gain = val},
            Err(err)=>send_err("Invalid gain."),
          }
          if freq != 0 && gain != 0.0{
            create_audio(freq, gain);
          }
        }
        } else if Regex::new(r"(sin|sqr|saw) ([\d]+)").unwrap().is_match(w)  {
          let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
          let mut freq:u32=0;
          match String::from(freq_result.unwrap().as_str()).parse::<u32>(){
            Ok(val)=>{freq = val; create_audio(freq, 1.0)},
            Err(err)=>send_err("Invalid frequency."),
          }
        } else if Regex::new(r"rnd (0.(\d+)|1)").unwrap().is_match(w) {
          let gain_result = Regex::new(r"(0\.[\d]+)|1").unwrap().find(w);
          let mut gain:f32=0.0;
          match String::from(gain_result.unwrap().as_str()).parse::<f32>(){
            Ok(val)=>{gain = val; create_audio(0,gain)},
            Err(err)=>send_err("Invalid gain."),
          }
        }
         else {
          send_err("Unknown type of audio.")
        }
      };

      let mut prepare_visual = |w: &[&str]| {
        if w.len() == 1{
          if param(w[0]) == Param::Range{
            if variant() != Variant::Screen{
            match String::from(w[0]).parse::<f32>(){
              Ok(val)=>create_visual(val, vec![1.0,1.0,1.0]),
              Err(err)=>send_err("Invalid radius for the object."),
            }
          } else {
            match String::from(w[0]).parse::<f32>(){
              Ok(val)=>create_visual(val, vec![val,val,val]),
              Err(err)=>send_err("Invalid grayscale for the screen."),
            }
          }
          }
          else if analyze_func(w[0]) == FnType::Rgb{
            if variant() != Variant::Screen{
            match floats_from(w[0]){
              Ok(val)=>create_visual(1.0, val),
              Err(err)=>send_err("Invalid rgb values for the object."),
            }
          } else {
            match floats_from(w[0]){
              Ok(val)=>create_visual(0.0, val),
              Err(err)=>send_err("Invalid rgb values for the screen."),
            }
          }
          }
          else {
            send_err("Unknown parameter for the object.");
          }
        }
        else if w.len() == 2{
           if variant() != Variant::Screen{
            if param(w[0]) == Param::Range && analyze_func(w[1]) == FnType::Rgb{
              let mut range:f32 = 0.0;
              let mut color:Vec<f32> = vec![];
              match String::from(w[0]).parse::<f32>(){
              Ok(val)=>{range = val},
              Err(err)=>send_err("Invalid value for the object radius."),
              }
              match floats_from(w[1]){
              Ok(val)=>{for v in val{color.push(v)}},
              Err(err)=>send_err("Invalid values for the rgb."),
              }
              if range != 0.0 && color.len() != 0{
                create_visual(range, color);
              }
            }
            else{
              send_err("Invalid parameters for the object.");
            }
           } else {send_err("Too many parameters for the screen.")}
        }
        else if w.len() < 1 || w.len() > 2{
          send_err("Too many or little parameters for the media.");
        } 
        else {
        todo!();
        }
        };

      let mut control_media = || {
        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>prepare_visual(&words[1.. ]),
            Medium::Audio=>prepare_audio(&expr),
            Medium::Effect=>prepare_effect(&expr),
            _=>send_err("Not suitable."),
          }
        } else {
          send_err("Unknown media.");
        }
      };

      if input.contains(";"){
        let exprs = input.split(';');
        for mut expression in exprs{
          expression = &expr;
          control_media();
        }
        Input::new(Shapes,Colors,Oscillators,Noises,Oscamps,Noiseamps,screen_color,multiplications)
      } else {
        let mut expr_ref = &*expr;
        expr_ref = input;
        control_media();
        Input::new(Shapes,Colors,Oscillators,Noises,Oscamps,Noiseamps,screen_color,multiplications)
      }
      }

        //the closure for JS which will change it
        fn execute(code: String, audio_context: &AudioContext, gl: &Context) {
          let input:Input = interpret(&*code, &audio_context, &gl);
          let mut oscs_len = input.oscs.len();
          let mut rnds_len = input.noises.len();
          let mut objs_len = input.shapes.len();
          let mut muls_len = input.muls.len();
         
            for i in 0..objs_len{
    
            }
        }

    pub fn start(){
      let document = web_sys::window().unwrap().document().unwrap();
      let canvas = document.get_element_by_id("graphics").unwrap()
      .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
      let textarea = document.get_element_by_id("input").unwrap()
      .dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
      let window = three_d::Window::new(WindowSettings{
        title: String::from("Pulchra"),
        canvas: Some(canvas),
        borderless: true,
        ..Default::default()
     }).unwrap();
      let gl = window.gl();
      let mut camera:Camera = Camera::new_perspective(
       window.viewport(),
       vec3(-3.0, 1.0, 2.5),
       vec3(0.0, 0.0, 0.0),
       vec3(0.0, 1.0, 0.0),
       degrees(45.0),
       0.1,
       1000.0
   );
   let mut channels:[f32; 4] = [0.0,0.0,0.0,1.0];
   let audio_context = AudioContext::new().unwrap();
   let input = interpret(&*textarea.value(), &audio_context, &gl);
    
   //mutable variables for changing the environment
    let mut channels:[f32; 4] = [0.0,0.0,0.0,1.0];
    let audio_context = AudioContext::new().unwrap();
    //the Input struct and its variables
    let input:Input = interpret(&*textarea.value(), &audio_context, &gl);
    let mut oscs = &input.oscs;
    let mut rnds = &input.noises;
    let mut objs = input.shapes;
    let mut muls = input.muls;
    let mut oscs_len = oscs.len();
    let mut rnds_len = rnds.len();
    let mut objs_len = objs.len();
    let mut muls_len = muls.len();
   
    //starting audio oscillators
   for i in 0..oscs_len{
      input.oscs[i].connect_with_audio_node(&input.osc_amps[i]);
      let mut gain_origin_val = input.osc_amps[i].gain().value();
      let new_amp = gain_origin_val / oscs_len as f32;
      gain_origin_val = new_amp;
      input.osc_amps[i].connect_with_audio_node(&audio_context.destination());
      input.oscs[i].stop();
      input.oscs[i].start();
      }
      for i in 0..rnds_len{
      input.noises[i].connect_with_audio_node(&input.noise_amps[i]);
      let mut gain_origin_val = input.noise_amps[i].gain().value();
      let new_amp = gain_origin_val / oscs_len as f32;
      gain_origin_val = new_amp;
      input.noise_amps[i].connect_with_audio_node(&audio_context.destination());
      input.noises[i].stop();
      input.noises[i].start();
      }
     
  //finally the window rendering 
     window.render_loop(move |frame_input| { 
      let screen = frame_input.screen();
      camera.set_viewport(frame_input.viewport);
        screen.clear(ClearState::color(channels[0], channels[1], channels[2], channels[3]));
        if muls_len != 0{
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
  
      
      FrameOutput::default()
    }); 
    }