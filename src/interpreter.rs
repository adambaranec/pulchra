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
    if Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgb;}
    else if Regex::new(r"rgba\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1),(0.(\d+)|1|0)\)").unwrap().is_match(word){return FnType::Rgba;}
    else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
    else if Regex::new(r"(?!.*(rgb|rgba|fft))").unwrap().is_match(word){return FnType::Unknown;}
    else {return FnType::NotFunction;}
    }
  
    fn get_color(word: &str) -> Vec<f32>{
    match analyze_func(word){
      FnType::Rgb=>{
        let mut channels:Vec<f32> = vec![];
        for param in Regex::new(r"(0.(\d+)|1|0)").unwrap().find_iter(word){
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
        for param in Regex::new(r"(0.(\d+)|1|0)").unwrap().find_iter(word){
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

    fn create_model(context: &Context, shape: Shape, radius: Option<f32>, color: Option<Color>)->Result<Gm, &'static str>{
      let cube_pos = vec![];
      let sphere_pos = vec![];
      match shape{
        Shape::Cube=>{if radius != None{return Ok(Gm::new(Mesh::new(&context, CpuMesh{positions: Postions::F32(cube_pos), colors: vec![color]}), ColorMaterial::default()))}
        else{return Ok(Gm::new(Mesh::new(&context, CpuMesh::cube(), ColorMaterial::default())))}},
        Shape::Sphere=>{if radius != None{return Ok(Gm::new(Mesh::new(&context, CpuMesh{positions: Postions::F32(sphere_pos), colors: vec![color]}), ColorMaterial::default()))}
        else{return Ok(Gm::new(Mesh::new(&context, CpuMesh::sphere(20), ColorMaterial::default())))}},
        _=>return Err("Unknown shape"),
      }
      }
      
      struct Input {
        models: [&dyn Object],
        clear_state: Option<ClearState>,
        effects: Option<Vec<Effect>>
      }

      fn interpret(input: &str)->Result<Input,()>{
        let statements:Vec<&str> = input.split(";").collect();
        let models = [&dyn Object];
        let clear_state:Option<ClearState> = None;
        let effects:Option<Vec<Effect>> = None;
        for statement in statements{
          let words:Vec<&str> = statement.split_whitespace().collect();
          match words[0]{
          "screen"=>{
            let float = words[1].iter().collect::<String>().parse::<f32>().unwrap();
            if float{
            let clear_state = ClearState::color(float,float,float);
            }
            else if Regex::new(r"(rgb)+\([^\)]*\)?").is_match(words[1]){
            let clear_state = ClearState::color(get_color[0], get_color[1], get_color[2]);
            }
           },
          "mul"=>{
            let num = words[1].iter().collect::<String>().parse::<u8>().unwrap();
          },
          "cube"=>{},
          "sphere"=>{},
          } 
        }
    }