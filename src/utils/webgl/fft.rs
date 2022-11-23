use web_sys::*;
use crate::utils::primitives::enums::enums::Sound;
pub fn fft(audio: &AudioContext, analyser: &AnalyserNode, sound: Sound)->f32{
let freq_count = analyser.frequency_bin_count();
let mut frequencies:Vec<f32> = vec![];
let freq = analyser.get_float_frequency_data(&mut frequencies);
match sound{
Sound::Lo=>{
let location = (0 + freq_count / 2) as usize;
frequencies[location]
},
Sound::Mid=>{
let location = (freq_count / 2) as usize;
frequencies[location]
},
Sound::Hi=>{
let location = (freq_count / 4 * 3) as usize;
frequencies[location]
},
_=>todo!(),
}
}