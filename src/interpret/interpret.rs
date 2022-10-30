use webgl::*;
use regex::Regex;
fn create_visual(gl: &WebGl2RenderingContext, shape: Variant, range: f32, color: Vec<f32>){
    let program = create_program( "
    attribute vec3 a_vertexPosition;
    attribute vec3 a_normal;
    uniform mat4 u_projection;
    uniform mat4 u_view;
    varying vec3 v_normal;
    void main(){
      gl_Position = u_projection * u_view * vec4(a_vertexPosition, 1.0);
      v_normal = a_normal;
    }
    ",
    "
    precision mediump float;
    varying vec3 v_normal;
    uniform vec3 u_reverseLightDirection;
    uniform vec4 u_color;
    void main(){
      vec3 normal = normalize(v_normal);
      float light = dot(normal, u_reverseLightDirection);
      gl_FragColor = u_color;
      gl_FragColor.rgb *= light;
    }
    ");
    gl.link_program(&program);
    gl.use_program(Some(&program));
    let vertex_buffer = gl.create_buffer();
    let element_buffer = gl.create_buffer();
    let normal_buffer = gl.create_buffer();
    let vertex_location = gl.get_attrib_location(&program, "a_vertexPosition") as u32;
    let normal_location = gl.get_attrib_location(&program, "a_normal") as u32;
    let color_location = gl.get_uniform_location(&program, "u_color");
    let direction_location = gl.get_uniform_location(&program, "u_reverseLightDirection");
    gl.uniform4f(color_location.as_ref(), color[0], color[1], color[2], 1.0);
           match shape{
            Variant::Cube=>{
              let cube_pos = vec![
                -range, -range, range,
                range, -range, range,
                range, range, range,
                -range, range, range, 
                -range, -range, -range,
                range, -range, -range,
                range, range, -range,
                -range, range, -range   
                  ];
                let cube_indices = &[
                //front face
                0,1,2,0,3,2, 
                //back face  
                4,5,6,4,7,6, 
                //upper face
                2,3,4,2,4,5,
                //bottom face
                0,1,4,0,5,4, 
                //right face
                1,2,5,2,6,5,
                //left face 
                0,3,4,0,4,7
                ];
              },
            Variant::Sphere=>{
            let sphere_pos = sphere_vertices(30,30);
            let sphere_ind = &sphere_indices(30,30)[ .. ];
            },
            _=>todo!(),
           }  
           gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
           gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
      }
      fn prepare_effect(w: &str){}
      fn create_osc(audio_context: &AudioContext, wave: Variant, freq: f32, gain: f32){
        let gain_node = GainNode::new(&audio_context).unwrap();
        gain_node.gain().set_value(gain);
        let osc = OscillatorNode::new(&audio_context).unwrap();
        osc.frequency().set_value(freq);
        match wave{
        Variant::SinOsc=>osc.set_type(OscillatorType::Sine),
        Variant::SawOsc=>osc.set_type(OscillatorType::Sawtooth),
        Variant::SqrOsc=>osc.set_type(OscillatorType::Square),
        Variant::TriOsc=>osc.set_type(OscillatorType::Triangle),
        _=>todo!(),
        }
        osc.connect_with_audio_node(&gain_node);
        gain_node.connect_with_audio_node(&audio_context.destination());
        osc.start();
        }
    
        fn create_noise(gain: f32){
          use rand::prelude::*;
          let ctx = AudioContext::new().unwrap();
          let gain_node = GainNode::new(&ctx).unwrap();
          gain_node.gain().set_value(gain);
          let noise_buf = AudioBuffer::new(&AudioBufferOptions::new(2 * ctx.sample_rate() as u32, ctx.sample_rate())).unwrap();
          let mut output:Vec<f32> = noise_buf.get_channel_data(0).unwrap();
          for mut val in output{
           val = rand::thread_rng().gen::<f32>() - gain * 2.0;
          }
          let mut options = AudioBufferSourceOptions::new();
          options.buffer(Some(&noise_buf));
          options.loop_(true);
          let buf_player = AudioBufferSourceNode::new_with_options(&ctx, &options).unwrap();
          buf_player.start();
        }
    
      fn create_audio(audio_context: &AudioContext, first_word: &str, freq: f32, gain: f32){
        let variant = get_variant(first_word);
        if variant != Variant::Unknown {
          match variant{
            Variant::SinOsc=>create_osc(audio_context, Variant::SinOsc, freq, gain),
            Variant::SawOsc=>create_osc(audio_context, Variant::SawOsc, freq, gain),
            Variant::SqrOsc=>create_osc(audio_context, Variant::SqrOsc, freq, gain),
            Variant::TriOsc=>create_osc(audio_context, Variant::TriOsc, freq, gain),
          _=>todo!(),
          }
        } else {
          send_err("Unknown oscillator.")
        }
      }
      fn prepare_audio(w: &str, audio: &AudioContext){
        let words:Vec<&str> = w.split_whitespace().collect();
        if words.len() == 1{
          if get_variant(words[0]) != Variant::NoiseOsc{
            send_err("Missing frequency for oscillator.");
          } else {
            send_err("Missing gain for noise.");
          }
        }
        else if words.len() == 2{
          if get_variant(words[0]) != Variant::NoiseOsc{
            if Regex::new(r"([\d]+|[\d]+\.[\d]+)\*(0.(\d+)|1|0)").unwrap().is_match(words[1]){
              let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
              let gain_result = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap().find(w);
              let mut freq:f32=0.0;
              let mut gain:f32=0.0;
              if freq_result != None && gain_result != None{
                match String::from(freq_result.unwrap().as_str()).parse::<f32>(){
                  Ok(val)=>{freq = val;},
                  Err(err)=>send_err("Invalid frequency."),
                }
                match String::from(gain_result.unwrap().as_str()).parse::<f32>(){
                  Ok(val)=>{
                    if val <= 1.0{
                      gain = val;
                    } else {
                      send_err("Invalid gain.");
                    }
                  },
                  Err(err)=>send_err("Invalid gain."),
                }
                if freq != 0.0 && gain <= 1.0{
                  create_audio(audio, words[0], freq, gain);
                } else {
                  send_err("Invalid oscillator.")
                }
              } else if freq_result != None && gain_result == None{
                send_err("Gain must not be greater than 1.");
              }
            }
            else if Regex::new(r"([\d]+|[\d]+\.[\d]+)").unwrap().is_match(words[1]){
              let freq_result = Regex::new(r"[\d]+").unwrap().find(w);
              let mut freq:f32=0.0;
              match String::from(freq_result.unwrap().as_str()).parse::<f32>(){
                Ok(val)=>{freq = val},
                Err(err)=>send_err("Invalid frequency."),
              }
              if freq != 0.0{
                create_audio(audio, words[0], freq, 1.0);
              }
              else{
                send_err("Invalid oscillator. Frequency must be always greater than zero.");
              }
            }
          } else {
            if Regex::new(r"(0.(\d+)|1|0)").unwrap().is_match(words[1]){
              let mut gain:f32=0.0;
             match String::from(words[1]).parse::<f32>(){
              Ok(val)=>{
                if val <= 1.0{
                gain = val;
              } else {
                send_err("Unknown parameters for the noise.");
              }},
              Err(err)=>send_err("Invalid gain."),
             }
             if gain != 0.0{
              create_noise(gain);
             }
            } else {
             send_err("Unknown parameters for the noise.")
            }
          }
          } else {
            send_err("Too many parameters for audio.")
          }
      }
      fn prepare_visual(w: &str, gl: &WebGl2RenderingContext){
        let expr:Vec<&str> = w.split_whitespace().collect();
        let range = Regex::new(r"(0\.[\d]*$)|(1*$)|(0*$)").unwrap();
        let rgb = Regex::new(r"rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap();
        let uv = Regex::new(r"\[(0.(\d+)|1|0),(0.(\d+)|1|0)\]").unwrap();
         
        if expr.len() == 1{
          match get_variant(expr[0]){
            Variant::Screen=>send_err("Missing grayscale or rgb for the screen."),
            Variant::Cube=>create_visual(gl, Variant::Cube, 1.0, vec![1.0,1.0,1.0]),
            Variant::Sphere=>create_visual(gl, Variant::Sphere, 1.0, vec![1.0,1.0,1.0]),
            _=>todo!(),
          }
        }
        else if expr.len() == 2{
            match get_variant(expr[0]){
              Variant::Screen=>{
                if Regex::new(r"screen (0.(\d+)|1|0)").unwrap().is_match(w){
                    if range.is_match(expr[1]){
                      match String::from(expr[1]).parse::<f32>(){
                        Ok(val)=>{
                          if val <= 1.0{
                            set_screen_color(gl, [val,val,val]);
                          } else {
                            send_err("Grayscale must not be greater than 1.");
                          }
                        },
                        Err(err)=>send_err("Invalid grayscale for the screen."),
                      }
                    }
                } else if Regex::new(r"screen rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(w){
                  if rgb.is_match(expr[1]){
                    let channels = floats_from(expr[1]);
                    match channels{
                      Ok(val)=>set_screen_color(gl, [val[0], val[1], val[2]]),
                      Err(err)=>send_err(err),
                    }
                  } else {
                    send_err("Invalid color function for the screen.");
                  }
                } else {
                  send_err("Unknown parameters for the screen.");
                }
              },
              Variant::Cube=>{
                if Regex::new(r"cube (0.(\d+)|1|0)").unwrap().is_match(w){
                  if range.is_match(expr[1]) {
                    match String::from(expr[1]).parse::<f32>(){
                      Ok(val)=>{
                        if val <= 1.0{
                          create_visual(gl, Variant::Cube, val, vec![1.0,1.0,1.0]);
                        } else {
                          send_err("Radius must not be greater than 1.")
                        }
                      },
                      Err(err)=>send_err("Invalid radius for the object."),
                    }
                  } else {
                    send_err("Radius must not be greater than 1.");
                  }
                } else if Regex::new(r"cube rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(w){
                  let color = rgb.find(w);
                  if color != None {
                   let channels = floats_from(expr[1]);
                   create_visual(gl, Variant::Cube, 1.0, channels.unwrap());
                  } else {
                  send_err("Invalid rgb for the object.");
                  }
                } else {
                  send_err("Unknown parameters for the object.");
                }
              },
              Variant::Sphere=>{ 
                if Regex::new(r"sphere (0.(\d+)|1|0)").unwrap().is_match(w){
                  if range.is_match(expr[1]) {
                    match String::from(expr[1]).parse::<f32>(){
                    Ok(val)=>{
                      if val <= 1.0{
                        create_visual(gl, Variant::Sphere, val, vec![1.0,1.0,1.0]);
                      } else {
                        send_err("Radius must not be greater than 1.")
                      }
                    },
                    Err(err)=>send_err("Invalid radius for the object."),
                  }
                } else {
                  send_err("Radius must not be greater than 1.");
                }
              } else if Regex::new(r"sphere rgb\((0.(\d+)|1|0),(0.(\d+)|1|0),(0.(\d+)|1|0)\)").unwrap().is_match(w){
                if rgb.is_match(expr[1]) {
                  let channels = floats_from(expr[1]);
                  create_visual(gl, Variant::Sphere, 1.0, channels.unwrap());
                } else {
                send_err("Invalid rgb for the object.");
                }
              } else {
                send_err("Unknown parameters for the object.");
              }},
              _=>todo!(),
            }
        }
        else if expr.len() == 3{
          match get_variant(expr[0]){
           Variant::Cube=>{},
           Variant::Sphere=>{},
           _=>todo!(),
          }
        } else if expr.len() > 2 || get_variant(expr[0]) == Variant::Screen{
         send_err("Too many parameters for the screen.");
        } else if expr.len() > 3 || get_variant(expr[0]) == Variant::Cube ||
        get_variant(expr[0]) == Variant::Sphere{
          send_err("Too many parameters for the object.");
        } else {
          todo!();
        }
      }

      fn interpret(input: &str, gl_ctx: &WebGl2RenderingContext, audio: &AudioContext){
      //at first, the error log must be cleaned
      send_err("");
      //individual words of the expression
      let words:Vec<&str> = input.split_whitespace().collect();
      let medium = || -> Medium {get_medium(words[0])};
      let variant = || -> Variant {get_variant(words[0])};
      let param = |w: &str| -> Param {get_param(w)};

        if medium() != Medium::Unknown {
          match medium(){
            Medium::Visuals=>prepare_visual(&input, &gl_ctx),
            Medium::Audio=>prepare_audio(&input, &audio),
            Medium::Effect=>prepare_effect(&input),
            _=>todo!(),
          }
        } else {
          send_err("Unknown media.");
        }
     }