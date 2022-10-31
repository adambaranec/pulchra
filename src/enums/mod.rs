pub mod enums;
#[derive(PartialEq)]
  enum Medium{
    Visuals,
    Audio,
    Mixed,
    Effect,
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
    TriOsc,
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
    enum FnType{
      Rgb,
      Rgba,
      Fft,
      Unknown,
      NotFunction
    }
    #[derive(PartialEq)]
    enum RealTime{
      Low,
      Mid,
      High,
      Sin,
      Saw,
      Tri,
      Rand
    }
    #[derive(PartialEq)]
    enum Effect{
      Multiplication
    }
    #[derive(PartialEq)]
  enum Channel{
      Red,
      Green,
      Blue
  }
  #[derive(PartialEq)]
  enum Sound{
  Lo,
  Mid,
  Hi
  }

    fn get_medium(word: &str)->Medium{
    match word{
      "screen"=>Medium::Visuals,
      "cube"=>Medium::Visuals,
      "sphere"=>Medium::Visuals,
      "sin"=>Medium::Audio,
      "saw"=>Medium::Audio,
      "sqr"=>Medium::Audio,
      "tri"=>Medium::Audio,
      "rnd"=>Medium::Audio,
      "mul"=>Medium::Effect,
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
      "tri"=>Variant::TriOsc,
      "rnd"=>Variant::NoiseOsc,
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