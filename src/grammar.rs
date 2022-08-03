use three_d::core::prelude::*;
use three_d::renderer::*;
use regex::Regex;



// What the individual tokens mean //
#[derive(PartialEq)]
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
        #[derive(PartialEq)]
    enum FnType{
      Rgb,
      Rgba,
      Fft,
      Unknown
    }
    #[derive(PartialEq)]
    enum Function{
      Begin,
      End
      }

    // All possible statements //
    #[derive(PartialEq)]
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
    else if word.to_string().parse::<u32>().unwrap() > 0{return Type::QuantityParam;}
    else if word.to_string().parse::<f32>().unwrap() <= 1.0{return Type::RangeParam;}
    else if Regex::new(r"[a-zA-Z]+\([^\)]*\)?").unwrap().is_match(word){return Type::Function;}
    else {return Type::Unknown;}
    }
    
    //parameter: slovo ako funkcia rgb()...
    fn analyze_func(word: &str) -> FnType{
    if Regex::new(r"rgb\((0.(\d+)|1),(0.(\d+)|1),(0.(\d+)|1)\)").unwrap().is_match(word){return FnType::Rgb;}
    else if Regex::new(r"rgba\((0.(\d+)|1),(0.(\d+)|1),(0.(\d+)|1),(0.(\d+)|1)\)").unwrap().is_match(word){return FnType::Rgba;}
    else if Regex::new(r"fft\[[0-9]+]").unwrap().is_match(word){return FnType::Fft;}
    else {return FnType::Unknown;}
    }
  
    fn get_color_channels(word: &str) -> Color{
      use std::convert::TryFrom;
    match analyze_func(word){
      FnType::Rgb=>{
        let mut channels:Vec<u8> = vec![];
        for param in Regex::new(r"(0.(\d+)|1)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let value = (float.iter().collect::<String>().parse::<f32>().unwrap() * 255.0) as u8;
        channels.push(value);
      };
      return Color{r: channels[0], g: channels[1], b: channels[2], a: 255};
      }, 
      FnType::Rgba=>{
        let mut channels:Vec<u8> = vec![];
        for param in Regex::new(r"(0.(\d+)|1)").unwrap().find_iter(word){
        let mut float:Vec<char> = vec![];
        for i in param.start()..param.end(){
        float.push(word.chars().nth(i).unwrap());
        };
        let value = (float.iter().collect::<String>().parse::<f32>().unwrap() * 255.0) as u8;
        channels.push(value);
      }; 
      return Color{r: channels[0], g: channels[1], b: channels[2], a: channels[3]}},
      _=>return Color{r: 0, g: 1, b: 2, a: 3},
    }
    }
  
    fn interpret(input: &str){
    //all statements in the code to interpret
    let statements:Vec<&str> = input.split("  ").collect();
    //all shapes to render after
    //let shapes:Vec<dyn Object> = vec![];

    /* 1. Clear all redundant objects (if any) before processing. */
    //shapes.clear();
    for statement in statements{
    let words:Vec<&str> = statement.split_whitespace().collect();
    /* Rule no. 1: The first word must always be an object or effect.*/
    match words[0]{
    /*  SCREEN  */
    "screen"=>{
    if get_type(words[1])==Type::RangeParam{}
    else if analyze_func(words[1])==FnType::Rgb{}
    },
    /*  CUBE  */
    "cube"=>{
    },
    /*  CUBOID  */
    "cuboid"=>{

    },
    /*  SPHERE  */
    "sphere"=>{

    },
    /*  MULTIPLICATION  */
    "mul"=>{

    },
    _=>{}
    }
    }
    }
    fn send_err(err: &str){}