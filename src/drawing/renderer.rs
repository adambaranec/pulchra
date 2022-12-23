use three_d::*;
use web_sys::{HtmlCanvasElement,Window,AudioContext,AnalyserNode};
use wasm_bindgen::JsCast;
use crate::utils::primitives::enums::enums::{Variant,Sound,Channel};
use crate::utils::primitives::error::error::send_err;
use crate::fft_options::FftOptions;
use crate::utils::webgl::fft::fft;

pub fn render(variant: Variant, range: f32, color: [f32; 3], uv: Option<[f32; 2]>, fft_op: Option<FftOptions>){
let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
.dyn_into::<HtmlCanvasElement>().unwrap();
    let window = three_d::Window::new(WindowSettings {
    canvas: Some(canvas),
    ..Default:: default()
}
).unwrap();
let context = window.gl();
let mut camera:Camera = Camera::new_perspective(
    window.viewport(),
    vec3(1.0, 1.0, 3.0),
    vec3(0.0, 0.0, 0.0),
    vec3(0.0, 3.0, 0.0),
    degrees(45.0),
    0.1,
    1000.0
);
let light = DirectionalLight::new(&context, 0.5, Color::WHITE, &vec3(0.0,0.0,0.0));
let mut mesh = Gm::new(Mesh::new(&context, &CpuMesh{..Default::default()}), ColorMaterial::default());
match variant{
    Variant::Cube=>{
        mesh = Gm::new(Mesh::new(&context, &CpuMesh::cube()), ColorMaterial{color: Color::new_opaque((color[0] * 255.0) as u8,(color[1] * 255.0) as u8,(color[2] * 255.0) as u8), ..Default::default()});
        mesh.set_transformation(Mat4::from_scale(range));
        if uv != None{
            mesh.set_transformation(Mat4::from_translation(Camera::position_at_uv_coordinates(&camera,(uv.unwrap()[0],uv.unwrap()[1]))));
        }
    },
    Variant::Sphere=>{
        mesh = Gm::new(Mesh::new(&context, &CpuMesh::sphere(30)), ColorMaterial{color: Color::new_opaque((color[0] * 255.0) as u8,(color[1] * 255.0) as u8,(color[2] * 255.0) as u8), ..Default::default()});
        mesh.set_transformation(Mat4::from_scale(range));
        if uv != None{
            mesh.set_transformation(Mat4::from_translation(Camera::position_at_uv_coordinates(&camera,(uv.unwrap()[0],uv.unwrap()[1]))));
        }
    },
    _=>{},
        }
if fft_op == None {
    window.render_loop(move |mut frame_input: FrameInput|{
        camera.set_viewport(frame_input.viewport);   
        frame_input.screen().render(&camera, &[&mesh], &[&light]);
        FrameOutput::default()
    });  
} else {
let options = fft_op.unwrap();
let ac = options.ac;
let freq = options.sound;
let chan = options.channel;
let mut clear_state = ClearState{alpha: Some(1.0), ..Default::default()};
   window.render_loop(move |mut frame_input: FrameInput|{
    camera.set_viewport(frame_input.viewport); 
    if variant != Variant::Screen{
        mesh.set_transformation(Mat4::from_scale(fft(&ac, freq))); 
        frame_input.screen().render(&camera, &[&mesh], &[&light]);
    } else {
        match chan{
         Channel::Red=>{clear_state.red = Some(fft(&ac, freq))},
         Channel::Green=>{clear_state.green = Some(fft(&ac, freq))},
         Channel::Blue=>{clear_state.blue = Some(fft(&ac, freq))},
         _=>todo!(),
        }
    }
    FrameOutput::default()
   });
}
}