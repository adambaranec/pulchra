use crate::utils::primitives::enums::enums::{Sound,Channel};
use web_sys::AudioContext;
#[derive(PartialEq)]
pub struct FftOptions{
    pub ac: AudioContext,
    pub sound: Sound,
    pub channel: Channel
}