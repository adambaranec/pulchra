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
        let value:f32 = float.iter().collect::<String>().parse::<f32>().unwrap();
        floats.push(value);
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
      struct Input {
        models: Vec<CpuMesh>,
        clear_state: Option<ClearState>,
        muls: Option<Vec<Multiplication>>,
        sounds: Option<Vec<Oscillator>>
      }

      enum State{
        Success,
        Wrong
      }
      fn inspect(input: String){
         let object = Medium::Visuals;
         let object_radius = (Medium::Visuals, Param::Range);
         let object_radius_color = (Medium::Visuals, Param::Range, FnType::Rgb);
         let object_color = (Medium::Visuals, FnType::Rgb);
         let osc_freq = (Medium::Audio, Param::Quantity);
         let osc_freq_amp = (Medium::Audio, Param::Quantity, Operator::Mul, Param::Range);
         let multiplication = (Effect::Multiplication, Param::Quantity);
      }

      fn execute(input: String)->Result<Input,&'static str>{
        if input == ""{
          Err("No input")
        }
        else{
        let mut shapes:Vec<CpuMesh> = vec![];
        let mut clear_color:Option<ClearState> = None;
        let mut multiplications:Option<Vec<Multiplication>> = None;
        let mut oscs:Option<Vec<Oscillator>> = None;
        let mut input_values:Input = Input{
          models: shapes,
          clear_state: clear_color,
          muls: multiplications,
          sounds: oscs
        };
        let mut words:Vec<&str> = vec![];
        Ok(input_values)
      }
      }
        /*for statement in statements{
          match words[0]{
          "screen"=>{
            if Regex::new(r"(rgb)+\([^\)]*\)?").unwrap().is_match(words[1]){
              input_values.clear_state = Some(ClearState::color(floats_from(words[1]).unwrap()[0], floats_from(words[1]).unwrap()[1], floats_from(words[1]).unwrap()[2],1.0));
              } else{ 
                let float = words[1].chars().collect::<String>().parse::<f32>();
                match float{
                  Ok(val) => input_values.clear_state = Some(ClearState::color(val,val,val,1.0)),
                  Err(err) => {return Err("Unknown parameter for screen color.");},
                }
              }
            },
          "mul"=>{
            let num = words[1].chars().collect::<String>().parse::<u32>();
            match num{
              Ok(val) => input_values.muls = Some(vec![Multiplication{rows: val, columns: val}]),
              Err(err) => {return Err("Unknown parameter for multiplication.")},
            }
          },
          "cube"=>{
            let radius = words[1].chars().collect::<String>().parse::<f32>();
            match radius{
              Ok(val)=> input_values.models.push(create_model(Shape::Cube,Some(val),None).unwrap()),
              Err(err) => input_values.models.push(create_model(Shape::Cube,None,None).unwrap()),
            }
          },
          "sphere"=>{
            let radius = words[1].chars().collect::<String>().parse::<f32>();
            match radius{
              Ok(val)=> input_values.models.push(create_model(Shape::Sphere,Some(val),None).unwrap()),
              Err(err) =>input_values.models.push(create_model(Shape::Sphere,None,None).unwrap()),
            }
          },
          "sin"=>{
            let mut array = vec![];
            let freq =  words[1].chars().collect::<String>().parse::<u32>();
            if words[2] == "*"{
              let gain = words[3].chars().collect::<String>().parse::<f32>();
              match freq{
                Ok(f)=>{
                  match gain{
                    Ok(g) => {array.push(Oscillator{freq: f, gain: Some(g), wave: OscillatorType::Sine});
                    input_values.sounds = Some(array)
                  },
                    Err(err) =>{array.push(Oscillator{freq: f, gain: Some(1.0), wave: OscillatorType::Sine});
                    input_values.sounds = Some(array)}
                  }
                },
                Err(err)=>{return Err("You must always supply the frequency value.")}
              }
            }
          },
          "saw"=>{
            let mut array = vec![];
            let freq =  words[1].chars().collect::<String>().parse::<u32>();
            if words[2] == "*"{
              let gain = words[3].chars().collect::<String>().parse::<f32>();
              match freq{
                Ok(f)=>{
                  match gain{
                    Ok(g) => {array.push(Oscillator{freq: f, gain: Some(g), wave: OscillatorType::Sawtooth});
                    input_values.sounds = Some(array)
                  },
                    Err(err) =>{array.push(Oscillator{freq: f, gain: Some(1.0), wave: OscillatorType::Sawtooth});
                    input_values.sounds = Some(array)}
                  }
                },
                Err(err)=>{return Err("You must always supply the frequency value.")}
              }
            }
          },
          "sqr"=>{
            let mut array = vec![];
            let freq =  words[1].chars().collect::<String>().parse::<u32>();
            if words[2] == "*"{
              let gain = words[3].chars().collect::<String>().parse::<f32>();
              match freq{
                Ok(f)=>{
                  match gain{
                    Ok(g) => {array.push(Oscillator{freq: f, gain: Some(g), wave: OscillatorType::Square});
                    input_values.sounds = Some(array)
                  },
                    Err(err) =>{array.push(Oscillator{freq: f, gain: Some(1.0), wave: OscillatorType::Square});
                    input_values.sounds = Some(array)}
                  }
                },
                Err(err)=>{return Err("You must always supply the frequency value.")}
              }
            }
          },
          "noise"=>{},
          _=>{return Err("Unknown type.")}
          } 
        }
         Ok(input_values)
      }*/

    fn render(code: &Input, context: &Context, camera: &Camera, f_input: &mut FrameInput, gui: &mut three_d::GUI){
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
    }
  
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
          
          match execute(String::from(get_input())){
            Ok(values)=>render(&values, &context, &camera, &mut frame_input, &mut gui),
            Err(err)=>send_err(err),
          }  
          FrameOutput::default()
      });
  }