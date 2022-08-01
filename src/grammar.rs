use three_d::core::prelude::*;
use regex::Regex;

enum Token {
    Character,
    String,
    Figure,
    Whitespace,
    LeftBracket,
    RightBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Dot
    }

// What the individual tokens mean //
    enum Type{
        Screen,
        Cube,
        Cuboid,
        Sphere,
        MulEffect,
        RangeParam,
        QuantityParam,
        Function,
        Unknown
        }
    enum FnType{
      Rgb,
      Rgba,
      Fft,
      Unknown
    }
    enum Function{
      Begin,
      End
      }

    // All possible statements //
    enum Statement {
      ObjectOnly,
      ObjectWithSides,
      ObjectWithSidesAndColor,
      ObjectWithColor,
      Mul,
      WrongStatement
    }
    
    //parameter: jedno slovo
    fn get_type(word: &str) -> Type{
    if word =="screen"{return Type::Screen;}
    else if word=="cube"{return Type::Cube;}
    else if word=="cuboid"{return Type::Cuboid;}
    else if word=="sphere"{return Type::Sphere;}
    else if word=="mul"{return Type::MulEffect;}
    else if word.to_string().parse::<u32>(){return Type::QuantityParam;}
    else if word.to_string().parse::<f32>().unwrap() < 1.0{return Type::RangeParam;}
    else if Regex::new(r"[a-z]+\(\)").unwrap().is_match(word){return Type::Function;}
    else {return Type::Unknown;}
    }
    
    //parameter: slovo ako funkcia rgb()...
    fn analyze_func(word: &str) -> FnType{
    if Regex::new(r"rgb\(0(\.\d+)?|1\.0,0(\.\d+)?|1\.0,0(\.\d+)?|1\.0\)").unwrap().is_match(word){return FnType::Rgb;}
    else if Regex::new(r"rgba\(0(\.\d+)?|1\.0,0(\.\d+)?|1\.0,0(\.\d+)?|1\.0,0(\.\d+)?|1\.0\)").unwrap().is_match(word){return FnType::Rgba;}
    else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
    else {return FnType::Unknown;}
    }
  
    fn get_color_channels(word: &str) -> Color{
    let channels : Vec<f32> = vec![];
    match analyze_func(word){
      FnType::Rgb=>
      FnType::Rgba=>
      _=>
    }
    }
    
    //Parameter: jeden celý príkaz z inputu
    fn analyze_statement(string: &str) -> Statement{
    let types : Vec<&str>= string.split_whitespace().collect();
    let statement : Vec<Type> = vec![];
    for word in types{
      statement.push(get_type(word));
    }
    match statement{
    []=>Statement::ObjectOnly,
    []=>Statement::ObjectWithSides,
    []=>Statement::ObjectWithSidesAndColor,
    []>=>Statement::ObjectWithColor,
    []=>Statement::Mul,
    _=>Statement::WrongStatement
    }
    }
    
    //parameter: celý kód 
    pub fn interpret(input: &str){
    Objects.clear();
    Lights.clear();
    let statements : Vec<&str> = input.split_whitespace().collect();
    let types : Vec<Type> = vec![];
    for word in statements{
      if get_type(word) == Type::Screen || Type::Cube || Type::Cuboid || Type::Sphere || Type::MulEffect{
        types.clear();
        types.push(get_type(word));
      }
    }
      match analyze_statement(statement){
      //Definition of an object which we want to 
      //see
      ObjectDefinition::ObjectOnly=>
        match get_type(statements[0]){
        Keyword::Screen=>
        Keyword::Cube=>
        Keyword::Cuboid=>
        Keyword::Sphere=>
        },
        //object definition also giving their length of sides 
        ObjectDefinition::ObjectWithSides=>
          match get_type(statement[0]){
          Keyword::Screen=>
          if get_type(statement[1]) == Parameter::Range{
          let gray:f32 = statement[1] as f32;
          set_screen_grayscale(gray);
          }
          else if analyze_func(statement[1]) == FnType::Rgb{
          set_screen_rgb(get_color_channels(statement[1]));
          }
          else if analyze_func(statement[1]) == FnType::Rgba{ 
          set_screen_rgba(get_color_channels(statement[1]));
          }
          Keyword::Cube=>
          if get_type(statement[1]) == Type::RangeParam{
          let length:f32 = statement[1] as f32;
          }
          Keyword::Cuboid=>
          if get_type(statement[1]) == Type::RangeParam &&
          if get_type(statement[2]) == Type::RangeParam &&
          if get_type(statement[3]) == Type::RangeParam{
          let length:f32=statement[1] as f32;
          let height:f32=statement[2] as f32;
          let width:f32=statement[3] as f32;
          }
          Keyword::Sphere=>
          if get_type(statement[1]) == Type::RangeParam{
          let radius:f32 = statement[1] as f32;
          }
          }
        }
    ObjectDefinition::ObjectWithSidesAndColor=>
    ObjectDefinition::ObjectWithColor=>
    EffectDefinition::Mul=>
    Error::WrongStatement=>send_err();
    }
    fn send_err(err: &str){}