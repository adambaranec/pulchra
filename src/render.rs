use three_d::*;
use regex::Regex;

struct Canvas{
    window: Window,
    context: Context,
    camera: Camera
}

struct Input {
    models: [&dyn Object],
    clear_state: Option<ClearState>,
    effects: Option<Vec<Effect>>
  }

static canvas = Canvas{
    window: Window::new(WindowSettings {
        title: "Pulchra".to_string(),
        min_size: (100, 100),
        ..Default::default()
})
    .unwrap(),
    context: window.gl().unwrap(),
    camera: Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0
    ).unwrap()
};

fn interpret(input: &str)->Result<Input,()>{
    let statements:Vec<&str> = input.split(";").collect();
    let models = [&dyn Object];
    let clear_state:Option<ClearState> = None;
    let effects:Option<Vec<Effect>> = None;
    for statement in statements{
      let words:Vec<&str> = statement.split_whitespace().collect();
      match words[0]{
      "screen"=>{
        let float = words[1].iter().collect::<String>().parse::<f32>().unwrap();
        if float{
        let clear_state = ClearState::color(float,float,float);
        }
        else if Regex::new(r"(rgb)+\([^\)]*\)?").is_match(words[1]){
        let clear_state = ClearState::color(get_color[0], get_color[1], get_color[2]);
        }
       },
      "mul"=>{
        let num = words[1].iter().collect::<String>().parse::<u8>().unwrap();
      },
      "cube"=>{},
      "sphere"=>{},
      } 
    }
}

pub fn render(context: Context, window: Window, camera: Camera){
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
            .clear(ClearState::color(0.5, 0.5, 0.5, 1.0))
            .unwrap()
            .render(&camera, &[], &[])
            .unwrap()
            .write(|| gui.render());

        FrameOutput::default()
    });
}