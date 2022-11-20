use web_sys::*;
use crate::utils::primitives::enums::enums::Sound;
pub fn fft(audio: &AudioContext, analyser: &AnalyserNode, sound: Sound)->f32{
match sound{
Sound::Lo=>{},
Sound::Mid=>{},
Sound::Hi=>{},
_=>todo!(),
}
0.0
}