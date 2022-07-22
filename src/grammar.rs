//functions such as "cube", "sphere" etc.
pub enum StandardFunctions{
    Screen,
    Cube,
    Cuboid,
    Sphere,
    Mul,
    Move,
    SinOsc,
    SqrOsc,
    SawOsc,
    NoiseOsc
}
//what type of params can be given to the functions
pub enum ParamTypes{
Float,
Int,
ParamFunction
}
//works only for visuals
pub enum ParamFunctions{
Rgb,
Rgba,
Grayscale,
Fft
}
//shapes will be initalized: <type> <side_length> <color>
//oscillators: <osc> <freq> <amp>
pub enum Expressions{
ScreenWithGrayScale,
ScreenWithRgb,
ScreenWithRgba,
Shape,
ShapeWithSide,
ShapeWithSideAndColorFunc,
ShapeWithColorFunc,
Multiplication,
Oscillator,
OscillatorWithFreq,
OscillatorWithFreqAndAmp,
OscillatorWithAmp
}
pub enum Errors{
Unexpected,
NotFound,
Mismatched
}

//example: screen 0.5 mul 5 cube 2 rgb(0.1,0.3,0.5)