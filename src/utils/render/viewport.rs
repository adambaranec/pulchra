use web_sys::*;
use three_d::core::*;
pub fn set_viewport(){
    let web_window = web_sys::window().unwrap();          
      let viewport = Viewport{
        width: web_window.inner_width().unwrap().as_f64().unwrap() as u32,
        height: web_window.inner_height().unwrap().as_f64().unwrap() as u32,
        x: 0,
        y: 0
        };
      let mut camera:Camera = Camera::new_perspective(
        viewport,
        vec3(1.0, 1.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 3.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0
    );
    camera.set_viewport(viewport); 
}