use three_d::*;

pub fn render(/*code: Input,*/ f_input: FrameInput){

}

pub fn start(){
    let window = Window::new(WindowSettings{title: String::from("Pulchra"),
    min_size: (100, 100),
    ..Default::default()
}).unwrap();
    let context = window.gl().unwrap();
    let camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0
    ).unwrap();
    let mut gui = three_d::GUI::new(&context).unwrap();
    window.render_loop(move |mut frame_input| {
        /*let viewport = Viewport {
            x: (frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);*/
        frame_input
            .screen()
            .clear(ClearState::color(0.0, 0.0, 0.0, 1.0))
            .unwrap()
            .render(&camera, &[], &[])
            .unwrap()
            .write(|| gui.render());

        FrameOutput::default()
    });
}