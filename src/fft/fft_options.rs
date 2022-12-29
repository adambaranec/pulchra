use crate::enums::enums::{Sound,Channel};
use web_sys::AnalyserNode;
#[derive(PartialEq)]
pub struct FftOptions{
    pub analyser: AnalyserNode,
    pub sound: Sound,
    pub channel: Channel
}