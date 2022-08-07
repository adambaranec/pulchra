use three_d::core::prelude::*;
use three_d::core::render_target::ClearState;
use three_d::renderer::*;
use regex::Regex;

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
      Multiplication(u8,u8)
    }
      
    //parameter: slovo ako funkcia rgb()...
    fn analyze_func(word: &str) -> FnType{
    if Regex::new(r"rgb\((0.(\d+)|1),(0.(\d+)|1),(0.(\d+)|1)\)").unwrap().is_match(word){return FnType::Rgb;}
    else if Regex::new(r"rgba\((0.(\d+)|1),(0.(\d+)|1),(0.(\d+)|1),(0.(\d+)|1)\)").unwrap().is_match(word){return FnType::Rgba;}
    else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
    else if Regex::new(r"(?!.*(rgb|rgba|fft))").unwrap().is_match(word){return FnType::Unknown;}
    else {return FnType::NotFunction;}
    }
  
    fn get_color(word: &str) -> Vec<f32>{
    match analyze_func(word){
      FnType::Rgb=>{
        let mut channels:Vec<f32> = vec![];
        for param in Regex::new(r"(0.(\d+)|1)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let value = float.iter().collect::<String>().parse::<f32>().unwrap();
        channels.push(value);
      };
      return channels;
      }, 
      FnType::Rgba=>{
        let mut channels:Vec<f32> = vec![];
        for param in Regex::new(r"(0.(\d+)|1)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let value = float.iter().collect::<String>().parse::<f32>().unwrap();
        channels.push(value);
      }; 
      return channels;
      },
    }
    }

    fn screen_color(word: &str)->ClearState{
      match analyze_func(word){
        let mut channels:Vec<f32> = vec![];
        for param in Regex::new(r"(0.(\d+)|1)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let value = float.iter().collect::<String>().parse::<f32>().unwrap();
        channels.push(value);
      }; 
      return ClearState{red: channels[0], green: channels[1], blue: channels[2], alpha: channels[3]}
    }
    }

    fn create_model(context: Context, shape: Shape, radius: Option<f32>, color: Option<Color>)->Result<Gm, &'static str>{
      match shape{
        Shape::Cube=>return Ok(Gm::new(Mesh::new(&context, CpuMesh{positions: Postions::F32(vec![]), colors: vec![color]}), ColorMaterial::default(), ..Default: default())),
        Shape::Sphere=>return Ok(Gm::new(Mesh::new(&context, CpuMesh{positions: Postions::F32(vec![]), colors: vec![color]}), ColorMaterial::default(), ..Default: default())),
        _=>return Err("Unknown shape"),
      }
    }