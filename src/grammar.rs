use three_d::core::prelude::Color;

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
    enum Type{
    Keyword,
    DecimalSeparator,
    Function,
    Number,
    Effect,
    Error
    }
    enum Keyword{
    Screen,
    Cube,
    Cuboid,
    Sphere
    }
    enum Number{
    Range, //floats 0 - 1
    Quantity //u8
    }
    enum Function{
    Name,
    FnType,
    Begin,
    Parameter,
    ParamSeparator,
    End
    }
    enum Name{
    RgbString,
    RgbaString
    }
    enum FnType{
    Rgb,
    Rgba
    }
    enum Parameter{
    Number
    }
    enum Effect{
    Multiplication 
    }
    enum Error{
    MismatchedTypes,
    LessThanExpected,
    MoreThanExpected,
    NotFound,
    Syntax 
    }
    
    static keywords : Vec<&str> = vec!["screen","cube","cuboid","sphere"];
    static fx : Vec<&str> = vec!["mul"];
    static func_names: Vec<&str> = vec!["rgb","rgba","gray"];
    static rgb : Vec<Function> = vec![Function::Name::RgbString,Function::Begin,Function::Parameter::Number::Range,Function:: ParamSeparator,Function::Parameter::Number::Range,Function:: ParamSeparator,Function::Parameter::Number::Range, Function::End];
    static rgba : Vec<Function> = vec![Function::Name::RgbaString,Function::Begin,Function::Parameter::Number::Range,Function:: ParamSeparator,Function::Parameter::Number::Range,Function:: ParamSeparator,Function::Parameter::Number::Range, Function::ParamSeparator, Function::Parameter::Number::Range,Function::End];
    
    enum Statement{
    ObjectDefinition,
    EffectDefinition
    }
    enum ObjectDefinition {
    ObjectNoParams,
    ObjectWithSides,
    ObjectWithSidesAndColor,
    ObjectWithColor
    }
    enum EffectDefinition {
    Mul
    }
    
    //parameter: jeden znak z inputu
    /*fn token_type(character: char) -> Token{
    match character{
    ""=>Token::Whitespace
    new Regex::/a-z=>Token::Character
    new Regex::/0-9=>Token::Figure
    "("=>Token::LeftBracket
    ")"=>Token::RightBracket
    ","=>Token::Comma
    "."=>Token::Dot
    _=>Type::Error::Syntax
    }
    }*/
    
    //parameter: jedno slovo
    fn get_type(word: &str) -> Type{
    match word{
    "screen"=>Type::Keyword::Screen
    "cube"=>Type::Keyword::Cube
    "cuboid"=>Type::Keyword::Cuboid
    "sphere"=>Type::Keyword::Sphere
    "mul"=>Type::Effect::Multiplication
    new Regex(/0-9)=>Type::Number::Quantity
    new Regex(/všetky čísla aj desatinné od 0 do 1)=>Type::Number::Range
    new Regex(/ma to aj zatvorky a ciarky)=>analyse_func(word);
    _=>Type::Error::MismatchedTypes
    }
    }
    
    //parameter: slovo ako funkcia rgb()...
    fn analyze_func(word: &str) -> Type{
    match word{
    regex pre rgb=>Type::Function::FnType::Rgb
    regex pre rgba=>Type:: Function::FnType::Rgba
    }
    }
    static mut color = Color{r: 0, g: 0, b: 0, a: 0};
    fn get_color_channels(word: &str) -> color{
    let channels : Vec<f32> = vec![];
    match word{
    "regex pre rgb"=>for type in word.split(&['(', ','][..]){
    if get_type(type) == Type::Number::Range{
        channels.push((type.parse::<f32>().unwrap() * 255).round() as u8);
       }
       color.r=channels[0];
       color.g=channels[1];
       color.b=channels[2];
    return color;
    }
    "regex pre rgba"=>for type in word.split(&['(', ','][..]){
    if get_type(type) == Type::Number::Range{
         channels.push((type.parse::<f32>().unwrap() * 255).round() as u8);
        }
       color.r=channels[0];
       color.g=channels[1];
       color.b=channels[2];
       color.a=channels[3];
    return color;
    }
    }
    }
    
    //Parameter: jeden celý príkaz z inputu
    fn analyze_statement(string: &str) -> Statement{
    let types= string.split_whitespace();
    let statement : Vec<Type> = vec![];
    for word in types{
      statement.push(get_type(word));
    }
    match statement{
    <objectNoParam>=>Statement::ObjectDefinition::ObjectOnly
    <ObjectWithSides>=>Statement::ObjectDefinition::ObjectWithSides
    <ObjectWithSidesAndColor>=>Statement::ObjectDefinition::ObjectWithSidesAndColor
    <ObjectWithColor>=>Statement::ObjectDefinition::ObjectWithColor
    <mul>=>Statement::EffectDefinition::Mul
    _=>Type::Error::WrongStatement
    }
    }
    
    //parameter: celý kód 
    pub fn interpret(input: &str){
    Objects.clear();
    Lights.clear();
    let statements = input.split(&[String::from("screen"), String::from("cube"), 
    String::from("cuboid"), String::from("sphere"), String::from("mul")][..]);
    for statement in statements{
      match analyze_statement(statement){
      //Definition of an object which we want to 
      //see
      Statement::ObjectDefinition::ObjectOnly=>
        match get_type(statement[0]){
        Type::Keyword::Screen=>
        Type::Keyword::Cube=>
        Type::Keyword::Cuboid=>
        Type::Keyword::Sphere=>
        }
        //object definition also giving their length of sides Statement::ObjectDefinition::ObjectWithSides=>
          match get_type(statement[0]){
          Type::Keyword::Screen=>
          if get_type(statement[1]) == Type::Parameter::Number::Range{
          let gray:f32 = statement[1] as f32;
          set_screen_grayscale(gray);
          }
          else if analyze_func(statement[1]) == Type::Function::FnType::Rgb{
    set_screen_rgb(get_color_channels(statement[1]));
          }
          else if analyze_func(statement[1]) == Type::Function::FnType::Rgba{ 
            set_screen_rgba(get_color_channels(statement[1]));
          }
          Type::Keyword::Cube=>
          if get_type(statement[1]) == Type::Parameter::Number::Range{
          let length:f32 = statement[1] as f32;
          }
          Type::Keyword::Cuboid=>
          if get_type(statement[1]) == Type::Parameter::Number::Range &&
          if get_type(statement[2]) == Type::Parameter::Number::Range &&
          if get_type(statement[3]) == Type::Parameter::Number::Range{
          let length:f32=statement[1] as f32;
          let height:f32=statement[2] as f32;
          let width:f32=statement[3] as f32;
          }
          Type::Keyword::Sphere=>
          if get_type(statement[1]) == Type::Parameter::Number::Range{
          let radius:f32 = statement[1] as f32;
          }
          }
    Statement::ObjectDefinition::ObjectWithSidesAndColor=>
    Statement::ObjectDefinition::ObjectWithColor=>
    Statement::EffectDefinition::Mul=>
    Type::Error::WrongStatement=>send_err();
    }
    }
    }
    fn send_err(err: &str){}