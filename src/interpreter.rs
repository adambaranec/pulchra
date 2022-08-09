use regex::Regex;
use three_d::*;

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

    fn create_model(context: &Context, shape: Shape, radius: Option<f32>, color: Option<Color>)->Result<CpuMesh, &'static str>{
      let cube_pos = vec![];
      let sphere_pos = vec![];
      match shape{
        Shape::Cube=>{if radius != None{return Ok(CpuMesh{positions: Positions::F32(cube_pos), colors: Some(vec![]), ..Default::default()})}
        else{return Ok(CpuMesh::cube())}},
        Shape::Sphere=>{if radius != None{return Ok(CpuMesh{positions: Positions::F32(sphere_pos), colors: Some(vec![]), ..Default::default()})}
        else{return Ok(CpuMesh::sphere(20))}},
        _=>{return Err("Unknown shape")},
      }
      }
      
      pub struct Input {
        models: Vec<CpuMesh>,
        clear_state: Option<ClearState>,
        effects: Option<Vec<Effect>>
      }

      pub fn interpret(input: &str)->Result<Input,&'static str>{
        let statements:Vec<&str> = input.split(";").collect();
        let mut models:Vec<CpuMesh> = vec![];
        let mut clear_color:Option<ClearState> = None;
        let mut effects:Option<Vec<Effect>> = None;
        for statement in statements{
          let words:Vec<&str> = statement.split_whitespace().collect();
          match words[0]{
          "screen"=>{
            if Regex::new(r"(rgb)+\([^\)]*\)?").unwrap().is_match(words[1]){
              clear_color = Some(ClearState::color(get_color(words[1]).unwrap()[0], get_color(words[1]).unwrap()[1], get_color(words[1]).unwrap()[2],1.0));
              } else{ 
                let float = words[1].chars().collect::<String>().parse::<f32>();
                match float{
                  Ok(val) => clear_color = Some(ClearState::color(val,val,val,1.0)),
                  Err(err) => return Err("Unknown parameter for screen color."),
                }
              }
            },
          "mul"=>{
            let num = words[1].chars().collect::<String>().parse::<u8>().unwrap();
            effects = Some(vec![Effect::Multiplication(num,num)]);
          },
          "cube"=>{},
          "sphere"=>{},
          _=>{return Err("Unknown type.")}
          } 
        }
        return Ok(Input{clear_state: clear_color, effects: effects, models: models});
    }