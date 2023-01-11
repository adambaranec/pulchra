use three_d::*;
use wasm_bindgen::JsCast;
use web_sys::*;
pub fn render(){
    let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap()
    .dyn_into::<HtmlCanvasElement>().unwrap();
    let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    let window = three_d::Window::new(WindowSettings {
      canvas: Some(canvas),
     ..Default:: default()
     }).unwrap();
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
    let mut mesh = Gm::new(Mesh::new(&context, &CpuMesh::sphere(30)), PositionMaterial::default());
    mesh.set_transformation(Mat4::from_scale(0.4));
    window.render_loop(move |frame_input: FrameInput|{
        mesh.render(&camera, &[&light]);
        FrameOutput::default()
    });
}