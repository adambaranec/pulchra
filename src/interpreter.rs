use regex::Regex;
use three_d::*;
use web_sys::{AudioContext,AudioDestinationNode,AudioNode,AudioParam,
GainNode,OscillatorNode,OscillatorType,AudioBuffer,AudioBufferOptions};

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
    Realtime,
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
      
    //parameter: slovo ako funkcia rgb()...
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

    fn create_osc(ctx: &AudioContext, wave: Variant, freq: u32, gain: f32){
    if wave != Variant::NoiseOsc{
    let osc = OscillatorNode::new(ctx).unwrap();
    let gain_node = GainNode::new(ctx).unwrap();
    osc.frequency().set_value(freq as f32);
    gain_node.gain().set_value(gain);
    match wave{
    Variant::SinOsc=>osc.set_type(OscillatorType::Sine),
    Variant::SawOsc=>osc.set_type(OscillatorType::Sawtooth),
    Variant::SqrOsc=>osc.set_type(OscillatorType::Square),
    _=>todo!(),
    }
    } else {
      use rand::prelude::*;
      let noise_buf = AudioBuffer::new(&AudioBufferOptions::new(2 * ctx.sample_rate() as u32, ctx.sample_rate())).unwrap();
      let mut output:Vec<f32> = noise_buf.get_channel_data(0).unwrap();
      for mut val in output{
       val = rand::thread_rng().gen::<f32>() - 1.0 * 2.0;
      }
    }
    }
  
    fn create_model(ctx: &Context, shape: Variant, range: f32, color: Color){
     match shape{
      Variant::Cube=>{
        let cube_pos:Vec<Vector3<f32>>=vec![
        vec3(range*(-1.0),range*(-1.0),0.0),vec3(range,range*(-1.0),0.0),
        vec3(range,range,0.0),vec3(range*(-1.0),range,0.0),
        vec3(range*(-1.0),range*(-1.0),range),vec3(range,range*(-1.0),range),
        vec3(range,range,range),vec3(range*(-1.0),range,range)];
        let model = Gm::new(Mesh::new(ctx, 
        &CpuMesh{positions: Positions::F32(cube_pos), colors: Some(vec![color]),
         ..Default::default()}),
        ColorMaterial::default());
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
        let model = Gm::new(Mesh::new(ctx, 
          &CpuMesh{positions: Positions::F32(sphere_pos), colors: Some(vec![color]),
           ..Default::default()}),
          ColorMaterial::default());
      },
      _=>todo!(),
     }
    }
      
      #[derive(PartialEq)]
      struct Multiplication{
        rows: u32,
        columns: u32
      }
        
      fn interpret(expr: String, audio_ctx: &AudioContext, gl_ctx: &Context){
      let words:Vec<&str> = expr.split_whitespace().collect();
      let medium = || -> Medium {get_medium(words[0])};
      let variant = || -> Variant {get_variant(words[0])};
      let param = |w: &str| -> Param {get_param(w)};

      let create_audio = |freq: u32, gain: f32| {
        if variant() != Variant::Unknown {
          match variant(){
            Variant::SinOsc=>create_osc(audio_ctx, Variant::SinOsc, freq, gain),
            Variant::SawOsc=>create_osc(audio_ctx, Variant::SawOsc, freq, gain),
            Variant::SqrOsc=>create_osc(audio_ctx, Variant::SqrOsc, freq, gain),
            Variant::NoiseOsc=>{},
            _=>todo!(),
          }
        } else {
  
        }
      };

      let create_visual = |range: f32, color: Vec<f32>| {
        if variant() != Variant::Unknown {
        match variant(){
          Variant::Screen=>{},
          Variant::Cube=>create_model(gl_ctx, Variant::Cube, range, Color::from_rgb_slice(&[color[0],color[1],color[2]])),
          Variant::Sphere=>create_model(gl_ctx, Variant::Sphere, range, Color::from_rgb_slice(&[color[0],color[1],color[2]])),
          _=>todo!(),
        }
      } else {

      }
      };

      let prepare_visual = |w: &[&str]| {
      if w.len() == 1{
        if param(w[0]) == Param::Range{
          match String::from(w[0]).parse::<f32>(){
            Ok(val)=>create_visual(val, vec![1.0,1.0,1.0]),
            Err(err)=>todo!(),
          }
        }
        else if analyze_func(w[0]) == FnType::Rgb{
          match floats_from(w[0]){
            Ok(val)=>create_visual(1.0, val),
            Err(err)=>todo!(),
          }
        }
        else {
          todo!();
        }
      }
      else if w.len() == 2{
         if variant() != Variant::Screen{
          if param(w[0]) == Param::Range && analyze_func(w[1]) == FnType::Rgb{
            let mut range:f32 = 0.0;
            let mut color:Vec<f32> = vec![];
            match String::from(w[0]).parse::<f32>(){
            Ok(val)=>{range = val},
            Err(err)=>todo!(),
            }
            match floats_from(w[0]){
            Ok(val)=>{for v in val{color.push(v)}},
            Err(err)=>todo!(),
            }
            create_visual(range, color);
          }
          else{
            todo!();
          }
         } else {todo!();}
      }
      else if w.len() < 1 || w.len() > 2{
        todo!();
      } 
      else {
      todo!()
      }
      };

      let prepare_audio = |w: &str| {
        if Regex::new(r"([a-z]+) ([\d]+) \* (0.(\d+)|1|0)").unwrap().is_match(w) ||
        Regex::new(r"([a-z]+) ([\d]+)\*(0.(\d+)|1|0)").unwrap().is_match(w){
        let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
        let gain_result = Regex::new(r"(0\.[\d]+)|1").unwrap().find(w);
        let mut freq:u32=0;
        let mut gain:f32=0.0;
        if freq_result != None{
          match String::from(freq_result.unwrap().as_str()).parse::<u32>(){
            Ok(val)=>{freq = val},
            Err(err)=>todo!(),
          }
          if gain_result == None{
            gain = 1.0;
            create_audio(freq, gain);
          }
          else{
            match String::from(gain_result.unwrap().as_str()).parse::<f32>(){
              Ok(val)=>{gain = val; create_audio(freq, gain)},
              Err(err)=>todo!(),
            }
          }
        }
        } else {
          todo!();
        }
      };

      let control_media = || {
        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>prepare_visual(&words[1.. ]),
            Medium::Audio=>prepare_audio(&expr),
            _=>todo!(),
          }
        }
      };
      control_media();
      }


  
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(module = "/index.js")]
    extern "C"{
      fn get_input() -> String;
      fn send_err(error: &str);
    }

    fn execute(input: String){
      if input.contains(";"){
        let exprs = input.split(';');
          for expr in exprs{

          }
      }
      else{
        let exprs = input.split_whitespace();
          for expr in exprs{
            
          }
    }
  }
    
  pub fn start(){
      let window = Window::new(WindowSettings{title: String::from("Pulchra"),
      min_size: (100, 100),
      ..Default::default()
  }).unwrap();
      let context = window.gl();
      let mut camera = Camera::new_perspective(
          window.viewport(),
          vec3(-3.0, 1.0, 2.5),
          vec3(0.0, 0.0, 0.0),
          vec3(0.0, 1.0, 0.0),
          degrees(45.0),
          0.1,
          1000.0
      );
      let ctx = AudioContext::new().unwrap();
  }

  fn render(window: Window){
    window.render_loop(move |frame_input| {
      FrameOutput::default()
  }); 
  }

  fn play(context: &AudioContext){

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
    }*/