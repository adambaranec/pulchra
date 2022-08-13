use regex::Regex;
use three_d::*;

  #[derive(PartialEq)]
  enum Type{
    Object,
    Radius,
    ShapeColor,
    FxName
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
      
    //parameter: slovo ako funkcia rgb()...
    fn analyze_func(word: &str) -> FnType{
    if Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgb;}
    else if Regex::new(r"rgba\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgba;}
    else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
    else if Regex::new(r"(?!.*(rgb|rgba|fft))").unwrap().is_match(word){return FnType::Unknown;}
    else {return FnType::NotFunction;}
    }
  
    fn get_color(word: &str) -> Result<Vec<f32>,&'static str>{
    match analyze_func(word){
      FnType::Rgb=>{
        let mut channels:Vec<f32> = vec![];
        for param in Regex::new(r"(0.(\d+)|1|0)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let value:f32 = float.iter().collect::<String>().parse::<f32>().unwrap();
        channels.push(value);
      };
      return Ok(channels);
      }, 
      FnType::Rgba=>{
        let mut channels:Vec<f32> = vec![];
        for param in Regex::new(r"(0.(\d+)|1|0)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let value:f32 = float.iter().collect::<String>().parse::<f32>().unwrap();
        channels.push(value);
      }; 
      return Ok(channels);
      },
      _=>return Err("Cannot get parameters from a non-function"),
    }
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
      struct Input {
        models: Vec<CpuMesh>,
        clear_state: Option<ClearState>,
        muls: Option<Vec<Multiplication>>
      }

      fn interpret(input: String)->Result<Input,&'static str>{
        if input == ""{
          return Err("No input")
        }
        else{

        let statements:Vec<&str> = input.split(";").collect();
        let mut shapes:Vec<CpuMesh> = vec![];
        let mut clear_color:Option<ClearState> = None;
        let mut multiplications:Option<Vec<Multiplication>> = None;
        let mut input_values:Input = Input{
          models: shapes,
          clear_state: clear_color,
          muls: multiplications
        };
        for statement in statements{
          let words:Vec<&str> = statement.split_whitespace().collect();
          match words[0]{
          "screen"=>{
            if Regex::new(r"(rgb)+\([^\)]*\)?").unwrap().is_match(words[1]){
              input_values.clear_state = Some(ClearState::color(get_color(words[1]).unwrap()[0], get_color(words[1]).unwrap()[1], get_color(words[1]).unwrap()[2],1.0));
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
              Err(err) => {return Err("Unknown parameter for shape radius.")},
            }
          },
          "sphere"=>{
            let radius = words[1].chars().collect::<String>().parse::<f32>();
            match radius{
              Ok(val)=> input_values.models.push(create_model(Shape::Sphere,Some(val),None).unwrap()),
              Err(err) => {return Err("Unknown parameter for shape radius.")},
            }
          },
          _=>{return Err("Unknown type.")}
          } 
        }
        return Ok(input_values);
      }
    }

    fn render(code: &mut Input, context: &Context, camera: &Camera, f_input: &mut FrameInput){
      let render_target:RenderTarget = f_input.screen();
      if code.clear_state == None{
        render_target.clear(ClearState::color(0.0, 0.0, 0.0, 1.0));
      } else{
        render_target.clear(code.clear_state.unwrap());
      }
      let mut gm_array = Vec::new();
      for model in &code.models{
        let object = Gm::new(Mesh::new(&context, &model).unwrap(), ColorMaterial::default());
        gm_array.push(object);
      }
      if code.muls == None{
        render_target.render(&camera, &[], &[]);  
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
              render_target.render_partially(scissor_box, &camera, &[], &[]);
              i += 1;
            }
         }
      }
    }

  
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(module = "/index.js")]
    extern "C"{
      fn get_input() -> String;
    }

    #[wasm_bindgen]
    pub fn get_code() -> String{
      get_input()
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

          render(&mut interpret(String::from("")).unwrap(), &context, &camera, &mut frame_input);
  
          FrameOutput::default()
      });
  }