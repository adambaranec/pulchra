use three_d::*;


fn canvas() -> WindowSettings{
    WindowSettings {
        title: String::from("Pulchra"),
        min_size: (100, 100),
        ..Default::default()
} 
}

fn render(window_setings: WindowSettings){
    let window = Window::new(canvas()).unwrap();
    let context = window.gl().unwrap();
    let mut camera =  Camera::new_perspective(
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
        let mut panel_width = 0.0;
        gui.update(&mut frame_input, |gui_context| {
            egui::SidePanel::left("").show(gui_context, |ui| {
                ui.heading("");
                });
            panel_width = 0.0;
        });
        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        if frame_input.first_frame {println!("Pulchra is running!")}
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