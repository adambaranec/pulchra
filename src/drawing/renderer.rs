use three_d::*;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use crate::enums::enums::{Variant,Channel};
use crate::fft::fft_options::FftOptions;
use crate::fft::fft::fft;

pub fn render(variant: Variant, range: f32, color: [f32; 3], uv: Option<[f32; 2]>, rotation: Option<f64>, fft_op: Option<FftOptions>){
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
let mut clear_state = ClearState{alpha: Some(1.0), ..Default::default()};
match variant{
    Variant::Cube=>{
        mesh = Gm::new(Mesh::new(&context, &CpuMesh::cube()), ColorMaterial{color: Color::new_opaque((color[0] * 255.0) as u8,(color[1] * 255.0) as u8,(color[2] * 255.0) as u8), ..Default::default()});
        mesh.set_transformation(Mat4::from_scale(range));
    },
    Variant::Sphere=>{
        mesh = Gm::new(Mesh::new(&context, &CpuMesh::sphere(30)), ColorMaterial{color: Color::new_opaque((color[0] * 255.0) as u8,(color[1] * 255.0) as u8,(color[2] * 255.0) as u8), ..Default::default()});
        mesh.set_transformation(Mat4::from_scale(range));
    },
    _=>{},
        }
if uv != None{
    mesh.set_transformation(Mat4::from_translation(Camera::position_at_uv_coordinates(&camera,(uv.unwrap()[0],uv.unwrap()[1]))));
 }
if fft_op == None {
    window.render_loop(move |frame_input: FrameInput|{
        camera.set_viewport(frame_input.viewport);   
        if variant != Variant::Screen{
            frame_input.screen().render(&camera, &[&mesh], &[&light]);
            if rotation != None {
                mesh.set_transformation(Mat4::from_angle_x(radians((frame_input.accumulated_time * rotation.unwrap()) as f32)));
                }
        } else {
            frame_input.screen().clear(ClearState::color(color[0],color[1],color[2],1.0));
        }
        FrameOutput::default()
    });  
} else {
let options = fft_op.unwrap();
let analyser = options.analyser;
let freq = options.sound;
let chan = options.channel;
   window.render_loop(move |frame_input: FrameInput|{
    camera.set_viewport(frame_input.viewport); 
    if variant != Variant::Screen{
        frame_input.screen().render(&camera, &[&mesh], &[&light]);
        mesh.set_transformation(Mat4::from_scale(fft(&analyser, freq))); 
        if rotation != None {
            mesh.set_transformation(Mat4::from_angle_x(radians((frame_input.accumulated_time * rotation.unwrap()) as f32)));
        }
    } else {
        match chan{
         Channel::Red=>{clear_state.red = Some(fft(&analyser, freq))},
         Channel::Green=>{clear_state.green = Some(fft(&analyser, freq))},
         Channel::Blue=>{clear_state.blue = Some(fft(&analyser, freq))},
         _=>todo!(),
        }
        frame_input.screen().clear(clear_state);
    }
    FrameOutput::default()
   });
}
}