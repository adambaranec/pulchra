use crate::enums::enums::{Sound,Fft};
use web_sys::AnalyserNode;
#[derive(PartialEq)]
pub struct FftOptions{
    //pub index: u16,
    pub analyser: AnalyserNode,
    pub sound: Sound,
    pub param: Fft
}