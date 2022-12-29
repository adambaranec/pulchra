use web_sys::*;
use js_sys::Float32Array;
use wasm_bindgen::JsCast;
use crate::enums::enums::{Channel,Sound};
fn set_screen_color(context: &WebGl2RenderingContext, channels: [f32; 3]){
    context.clear_color(channels[0], channels[1], channels[2], 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
  }
fn check_clear_color_channel(gl: &WebGl2RenderingContext, chan: Channel, amp: f32){
    let parameter = gl.get_parameter(WebGl2RenderingContext::COLOR_CLEAR_VALUE).unwrap().dyn_into::<Float32Array>().unwrap();
    match chan{
        Channel::Red=>set_screen_color(gl, [amp, parameter.get_index(1), parameter.get_index(2)]),
        Channel::Green=>set_screen_color(gl, [parameter.get_index(0), amp, parameter.get_index(2)]),
        Channel::Blue=>set_screen_color(gl, [parameter.get_index(0), parameter.get_index(1), amp]),
        _=>todo!(),
    }
}
pub fn screen_fft(gl: &WebGl2RenderingContext, audio: &AudioContext, channel: Channel, eq: Sound, mul: f32){
    let analyser = AnalyserNode::new(audio).unwrap();
    let bin_count = analyser.frequency_bin_count();
    let min = analyser.min_decibels() as f32;
    let max = analyser.max_decibels() as f32;
    let mut fft:f32=0.0;
    let mut chanval:f32=0.0;
    let mut frequencies:Vec<f32> = vec![];
    analyser.get_float_frequency_data(&mut frequencies[ .. ]);
    match eq{
    Sound::Lo=>{
        let loc = (bin_count / 4) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, channel, freq);
    },
    Sound::Mid=>{
        let loc = (bin_count / 2) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, channel, freq);
    },
    Sound::Hi=>{
        let loc = (bin_count - (bin_count / 4)) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, channel, freq);
    },
    _=>todo!(),
    }
    let window = web_sys::window();
}
pub fn screen_fft_two(gl: &WebGl2RenderingContext, audio: &AudioContext, chan1: Channel, chan2: Channel, eq: Sound, mul: f32){
    let analyser = AnalyserNode::new(audio).unwrap();
    let bin_count = analyser.frequency_bin_count();
    let min = analyser.min_decibels() as f32;
    let max = analyser.max_decibels() as f32;
    let mut fft:f32=0.0;
    let mut chan1val:f32=0.0;
    let mut chan2val:f32=0.0;
    let mut frequencies:Vec<f32> = vec![];
    analyser.get_float_frequency_data(&mut frequencies[ .. ]);
    match eq{
    Sound::Lo=>{
        let loc = (bin_count / 4) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, chan1, freq);
        check_clear_color_channel(gl, chan2, freq);
    },
    Sound::Mid=>{
        let loc = (bin_count / 2) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, chan1, freq);
        check_clear_color_channel(gl, chan2, freq);
    },
    Sound::Hi=>{
        let loc = (bin_count - (bin_count / 4)) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, chan1, freq);
        check_clear_color_channel(gl, chan2, freq);
    },
    _=>todo!(),
    }
}
pub fn screen_fft_all(gl: &WebGl2RenderingContext, audio: &AudioContext, eq: Sound, mul: f32){
    let analyser = AnalyserNode::new(audio).unwrap();
    let bin_count = analyser.frequency_bin_count();
    let min = analyser.min_decibels() as f32;
    let max = analyser.max_decibels() as f32;
    let mut fft:f32=0.0;
    let mut frequencies:Vec<f32> = vec![];
    analyser.get_float_frequency_data(&mut frequencies[ .. ]);
    match eq{
    Sound::Lo=>{
        let loc = (bin_count / 4) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        set_screen_color(gl, [freq,freq,freq]);
    },
    Sound::Mid=>{
        let loc = (bin_count / 2) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        set_screen_color(gl, [freq,freq,freq]);
    },
    Sound::Hi=>{
        let loc = (bin_count - (bin_count / 4)) as usize;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        set_screen_color(gl, [freq,freq,freq]);
    },
    _=>todo!(),
    }
}