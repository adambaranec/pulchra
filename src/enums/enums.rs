use regex::Regex;
#[derive(PartialEq)]
  pub enum Medium{
    Visuals,
    Audio,
    Mixed,
    Effect,
    Unknown
  }
  #[derive(PartialEq)]
  pub enum Variant{
    Screen,
    Cube,
    Sphere,
    Quad,
    Circle,
    Line,
    Point,
    Arc,
    SinOsc,
    SawOsc,
    SqrOsc,
    TriOsc,
    NoiseOsc,
    Multiplication,
    Unknown
  }
  #[derive(PartialEq)]
  pub enum Param{
    Quantity,
    Range,
    Color,
    Function,
    Realtime,
    Unknown
  }
  #[derive(PartialEq)]
    pub enum FnType{
      Rgb,
      Rgba,
      Fft,
      Unknown,
      NotFunction
    }
    #[derive(PartialEq)]
    pub enum RealTime{
      Low,
      Mid,
      High,
      Sin,
      Saw,
      Tri,
      Rand
    }
    #[derive(PartialEq)]
    pub enum Effect{
      Multiplication
    }
    #[derive(PartialEq)]
    #[derive(Copy)]
    #[derive(Clone)]
  pub enum Channel{
      Red,
      Green,
      Blue
  }
  #[derive(PartialEq)]
  #[derive(Copy)]
 #[derive(Clone)]
  pub enum Sound{
  Lo,
  Mid,
  Hi,
  Unknown
  }

    pub fn get_medium(word: &str)->Medium{
      if Regex::new("screen").unwrap().is_match(word){Medium::Visuals}
      else if Regex::new("cube").unwrap().is_match(word){Medium::Visuals}
      else if Regex::new("sphere").unwrap().is_match(word){Medium::Visuals}
      else if Regex::new("sin").unwrap().is_match(word){Medium::Audio}
      else if Regex::new("saw").unwrap().is_match(word){Medium::Audio}
      else if Regex::new("sqr").unwrap().is_match(word){Medium::Audio}
      else if Regex::new("tri").unwrap().is_match(word){Medium::Audio}
      else if Regex::new("rnd").unwrap().is_match(word){Medium::Audio}
      else if Regex::new("mul").unwrap().is_match(word){Medium::Effect}
      else if Regex::new("(lo|mi|hi)/g").unwrap().is_match(word){Medium::Mixed}
      else {Medium::Unknown}
    }

    pub fn get_variant(word: &str)->Variant{
      match word{
      "screen"=>Variant::Screen,
      "cube"=>Variant::Cube,
      "sphere"=>Variant::Sphere,
      "sin"=>Variant::SinOsc,
      "saw"=>Variant::SawOsc,
      "sqr"=>Variant::SqrOsc,
      "tri"=>Variant::TriOsc,
      "rnd"=>Variant::NoiseOsc,
      "mul"=>Variant::Multiplication,
      _=>Variant::Unknown
      }
    }

    pub fn get_param(param: &str)->Param{
      let range:bool = Regex::new(r"((0.(\d+)|1|0))|\-(0.(\d+)|1|0))").unwrap().is_match(param);
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

    pub fn get_sound(sound: &str)->Sound{
      match sound{
        "lo"=>Sound::Lo,
        "mi"=>Sound::Mid,
        "hi"=>Sound::Hi,
        _=>Sound::Unknown,
      }
    }