use interpret::*;
use enums::*;
use web_sys::*;
fn check_clear_color_channel(gl: &WebGl2RenderingContext, chan: Channel, amp: f32){
    let parameter = gl.get_parameter(WebGl2RenderingContext::COLOR_CLEAR_VALUE).unwrap().dyn_into::<Float32Array>().unwrap();
    match chan{
        Red=>set_screen_color(gl, [amp, parameter[1], parameter[2]]),
        Green=>set_screen_color(gl, [parameter[0], amp, parameter[2]]),
        Blue=>set_screen_color(gl, [parameter[0], parameter[1], amp]),
        _=>todo!(),
    }
}
fn draw_fft(analyser: &AnalyserNode, gl: &WebGl2RenderingContext, array: &mut [f32], eq: Sound, chan: Channel){
    analyser.get_float_frequency_data(array);
    let bin_count = analyser.frequency_bin_count();
    match eq{
        Lo=>{
            let loc = bin_count / 4;
            let freq = ((array[loc] - min) / (max - min)) * mul;
            check_clear_color_channel(gl, chan, freq);
        },
        Mid=>{
            let loc = bin_count / 2;
            let freq = ((array[loc] - min) / (max - min)) * mul;
            check_clear_color_channel(gl, chan, freq);
        },
        Hi=>{
            let loc = bin_count - (bin_count / 4);
            let freq = ((array[loc] - min) / (max - min)) * mul;
            check_clear_color_channel(gl, chan, freq);
        },
}
}
fn screen_fft(gl: &WebGl2RenderingContext, audio: &AudioContext, channel: Channel, eq: Sound, mul: f32){
    let analyser = AnalyserNode::new(audio).unwrap();
    let bin_count = analyser.frequency_bin_count();
    let min = analyser.min_decibels();
    let max = analyser.max_decibels();
    let mut fft:f32=0.0;
    let mut chanval:f32=0.0;
    let mut frequencies:Vec<f32> = vec![];
    analyser.get_float_frequency_data(&frequencies[ .. ]);
    match eq{
    Lo=>{
        let loc = bin_count / 4;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, channel, freq);
    },
    Mid=>{
        let loc = bin_count / 2;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, channel, freq);
    },
    Hi=>{
        let loc = bin_count - (bin_count / 4);
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, channel, freq);
    },
    _=>todo!(),
    }
    let window = web_sys::window();
}
fn screen_fft_two(gl: &WebGl2RenderingContext, audio: &AudioContext, chan1: Channel, chan2: Channel, eq: Sound, mul: f32){
    let analyser = AnalyserNode::new(audio).unwrap();
    let bin_count = analyser.frequency_bin_count();
    let min = analyser.min_decibels();
    let max = analyser.max_decibels();
    let mut fft:f32=0.0;
    let mut chan1val:f32=0.0;
    let mut chan2val:f32=0.0;
    analyser.get_float_frequency_data(&frequencies[ .. ]);
    match eq{
    Lo=>{
        let loc = bin_count / 4;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, chan1, freq);
        check_clear_color_channel(gl, chan2, freq);
    },
    Mid=>{
        let loc = bin_count / 2;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, chan1, freq);
        check_clear_color_channel(gl, chan2, freq);
    },
    Hi=>{
        let loc = bin_count - (bin_count / 4);
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        check_clear_color_channel(gl, chan1, freq);
        check_clear_color_channel(gl, chan2, freq);
    },
    _=>todo!(),
    }
        let window = web_sys::window();
}
fn screen_fft_all(gl: &WebGl2RenderingContext, audio: &AudioContext, eq: Sound, mul: f32){
    let analyser = AnalyserNode::new(audio).unwrap();
    let bin_count = analyser.frequency_bin_count();
    let min = analyser.min_decibels();
    let max = analyser.max_decibels();
    let mut fft:f32=0.0;
    analyser.get_float_frequency_data(&frequencies[ .. ]);
    match eq{
    Lo=>{
        let loc = bin_count / 4;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        set_screen_color(gl, [freq,freq,freq]);
    },
    Mid=>{
        let loc = bin_count / 2;
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        set_screen_color(gl, [freq,freq,freq]);
    },
    Hi=>{
        let loc = bin_count - (bin_count / 4);
        let freq = ((frequencies[loc] - min) / (max - min)) * mul;
        set_screen_color(gl, [freq,freq,freq]);
    },
    _=>todo!(),
    }
        let window = web_sys::window();
}
