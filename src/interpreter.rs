use regex::Regex;
use three_d::*;
use web_sys::{AudioContext,AudioDestinationNode,AudioNode,AudioParam,
GainNode,OscillatorNode,OscillatorType};

  #[derive(PartialEq)]
  enum Medium{
    Visuals,
    Audio,
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
    Unknown
  }
  #[derive(PartialEq)]
  enum Shape{
    Cube,
    Sphere
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
    enum Operator{
      Mul,
      Division,
      Unknown
    }
    #[derive(PartialEq)]
    enum Effect{
      Multiplication
    }

    /* These functions find meaning of individual words.*/

    fn get_medium(word: &str)->Medium{
    match word{
      "screen"=>Medium::Visuals,
      "cube"=>Medium::Visuals,
      "sphere"=>Medium::Visuals,
      "sin"=>Medium::Audio,
      "saw"=>Medium::Audio,
      "sqr"=>Medium::Audio,
      "noise"=>Medium::Audio,
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
      "noise"=>Variant::NoiseOsc,
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

    fn get_operator(symbol: char)->Operator{
      match symbol{
      '*'=>Operator::Mul,
      '/'=>Operator::Division,
      _=>Operator::Unknown
      }
    }
      
    //parameter: slovo ako funkcia rgb()...
    fn analyze_func(word: &str) -> FnType{
    if Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgb;}
    if Regex::new(r"rgba\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgba;}
    if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
    if Regex::new(r"(?!.*(rgb|rgba|fft))").unwrap().is_match(word){return FnType::Unknown;}
    else {return FnType::NotFunction;}
    }

    fn floats_from(word: &str)->Result<Vec<f32>,&'static str>{
      let mut floats:Vec<f32> = vec![];
      for param in Regex::new(r"(0.(\d+)|1|0)").unwrap().find_iter(word){
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
  
    fn create_model(shape: Shape, radius: Option<f32>, color: Option<Color>)->Result<CpuMesh, &'static str>{
      let cube_pos = vec![];
      let sphere_pos = vec![];
      match shape{
        Shape::Cube=>{if radius != None{return Ok(CpuMesh{positions: Positions::F32(cube_pos), colors: Some(vec![]), ..Default::default()})}
        else{return Ok(CpuMesh::cube())}},
        Shape::Sphere=>{if radius != None{return Ok(CpuMesh{positions: Positions::F32(sphere_pos), colors: Some(vec![]), ..Default::default()})}
        else{return Ok(CpuMesh::sphere(20))}},
      }
      }
      
      #[derive(PartialEq)]
      struct Multiplication{
        rows: u32,
        columns: u32
      }
      #[derive(PartialEq)]
      struct Oscillator{
        freq: u32,
        gain: Option<f32>,
        wave: OscillatorType
      }
    
     enum Input{
      Visual{
        object: Variant,
        radius: Option<f32>,
        color: Option<Color>
      },
      Audio{
        wave: OscillatorType,
        gain: Option<f32>
      }
     }
      
      fn interpret(expr: String)->Result<(),&'static str>{
      let words:Vec<&str> = expr.split_whitespace().collect();
      let medium = || -> Medium {get_medium(words[0])};
      let variant = || -> Variant {get_variant(words[0])};
      let param = |w: &str| -> Param {get_param(w)};

      let create_color = |w : &str| -> Result<(), &'static str>{
        match floats_from(w){
          Ok(vec)=>{
            match vec.len(){
                   3=>{return Ok(())},
                   4=>{return Ok(())},
                   _=>{return Err("Too many or little parameters.")},
                     }
          },
          Err(err)=>{return Err("Invalid values.")},
        }
        Ok(())
      };
   
      let control_functions = |w: &str| -> Result<(), &'static str> {
        if medium() == Medium::Visuals{
          match analyze_func(w){
            FnType::Rgb=>create_color(w),
            FnType::Rgba=>create_color(w),
            FnType::Fft=>Ok(()),
            FnType::Unknown=>Err("This function does not exist."),
            _=>Err("This function does not suit."),
          }
        }
        else{
          match analyze_func(w){
            FnType::Unknown=>Err("This function does not exist."),
            _=>Err("This function does not suit here."),
          }
        }
      };

      let video_params = |w: &str| -> Result<(), &'static str> {
        if param(w) != Param::Unknown && param(w) != Param::Quantity{
         match param(w){
         Param::Range=>Ok(()),
         Param::Function=>control_functions(w),
         Param::Quantity=>Err("Unknown parameter."),
         Param::Unknown=>todo!(),
         }
        } else {
          Err("Unknown parameter.")
        }
      };

      let audio_params = |w: &str| -> Result<(), &'static str> {
        if param(w) != Param::Unknown && param(w) != Param::Quantity{
          match param(w){
          Param::Range=>Ok(()),
          Param::Function=>control_functions(w),
          Param::Quantity=>Err("Unknown parameter."),
          Param::Unknown=>todo!(),
          }
         } else {
           Err("Unknown parameter.")
         }
      };

      let control_media = || -> Result<(), &'static str> {
        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>video_params(words[1]),
            Medium::Audio=>audio_params(words[1]),
            _=>todo!(),
          }
        } else {
          Err("Unknown type of media.")
        }
      };
      control_media()
      }

    /*fn render(code: &Input, context: &Context, camera: &Camera, f_input: &mut FrameInput, gui: &mut three_d::GUI){
      let render_target:RenderTarget = f_input.screen();
      if code.clear_state == None{
        render_target.clear(ClearState::color(0.0, 0.0, 0.0, 1.0)).unwrap();
      } else{
        render_target.clear(code.clear_state.unwrap());
      }
      let mut gm_array = Vec::new();
      for model in &code.models{
        let object = Gm::new(Mesh::new(&context, &model).unwrap(), ColorMaterial::default());
        gm_array.push(object);
      }
      if code.muls == None{
        render_target.render(&camera, &[], &[]).unwrap();  
      }else{
         for muls in code.muls.as_ref(){
          let mut i:u32 = 0;
            for screen in muls{
              let rows:u32 = screen.rows;
              let columns:u32 = screen.columns;
              let scissor_box = ScissorBox{
                height: f_input.viewport.height/columns,
                width: f_input.viewport.width/rows,
                x: ((f_input.viewport.width/rows)*i) as i32,
                y: ((f_input.viewport.height/columns)*i) as i32
              };
              render_target.render_partially(scissor_box, &camera, &[], &[]).unwrap();
              i += 1;
            }
         }
      }
      render_target.write(|| gui.render());
    }

    fn sound(code: &Input){
     let ctx = AudioContext::new().unwrap();
     for arr in &code.sounds{
       for osc in arr {
       let synth = ctx.create_oscillator().unwrap();
       let gain = ctx.create_gain().unwrap();
       synth.set_type(osc.wave);
       gain.gain().set_value(osc.gain.unwrap());
       synth.connect_with_audio_node(&gain);
       gain.connect_with_audio_node(&ctx.destination());
       }
     }
    }*/
  
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(module = "/index.js")]
    extern "C"{
      fn get_input() -> String;
      fn send_err(error: &str);
    }
    
  pub fn start(){
      let window = Window::new(WindowSettings{title: String::from("Pulchra"),
      min_size: (100, 100),
      ..Default::default()
  }).unwrap();
      let context = window.gl().unwrap();
      let mut camera = Camera::new_perspective(
          &context,
          window.viewport().unwrap(),
          vec3(-3.0, 1.0, 2.5),
          vec3(0.0, 0.0, 0.0),
          vec3(0.0, 1.0, 0.0),
          degrees(45.0),
          0.1,
          1000.0
      ).unwrap();
      let mut gui = three_d::GUI::new(&context).unwrap();
      window.render_loop(move |mut frame_input| {
          let viewport = Viewport::new_at_origo(frame_input.viewport.width, frame_input.viewport.height);
          camera.set_viewport(viewport);
          /*frame_input
          .screen()
          .clear(ClearState::color(0.0, 0.0, 0.0, 1.0))
          .unwrap()
          .render(&camera, &[], &[])
          .unwrap()
          .write(|| gui.render());*/ 
          FrameOutput::default()
      });
  }