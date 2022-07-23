use three_d::*;
use twang::*;

/*VARIABLES FOR RENDERING AND VISUALS*/
// 1. 3D shapes to be rendered //
static mut Objects : &[&dyn Object] = &[];
// 2. Lights for them //
static mut Lights : &[&dyn Light] = &[];

/*VARIABLES FOR OSCILLATORS AND SYNTH*/

pub fn start(){
    println!("Creating a window...");
    let window = Window::new(WindowSettings {
        title: "Pulchra".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0
    ).unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let light = AmbientLight::new(&context, 1.0, Color{r: 1, g: 200, b: 10, a: 1}).unwrap();
    let mut model = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()).unwrap(),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                roughness: 0.2,
                metallic: 0.8,
                ..Default::default()
            },
        ).unwrap()
    );
    let mut gui = three_d::GUI::new(&context).unwrap();
    start_interpreter();
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
        control.handle_events(&mut camera, &mut frame_input.events);
        if frame_input.first_frame {println!("Pulchra is running!")}
        /*match frame_input.events.last().unwrap(){
         Event::KeyPress{kind: Key::Enter, 
             modifiers: Modifiers{alt: false, ctrl: false, shift: true, command: false},
             handled: false
            }=>
        };*/
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .unwrap()
            .render(&camera, &[&model], &[&light])
            .unwrap()
            .write(|| gui.render());

        FrameOutput::default()
    });
}

//functions for the interpreter
fn start_interpreter(){
    println!("Starting the interpreter...");
}
fn interpret(input: &[&str]){
for word in input.iter(){

}
}

//creating shapes
fn create_cube(side: f32){}
fn create_cuboid(a: f32, b: f32, c: f32){}
fn create_cuboid_oneside(side: f32){}
fn create_sphere(side: f32){}
//colors
fn set_rgb(red: u8, green: u8, blue: u8){}
fn set_rgba(red: u8, green: u8, blue: u8, alpha: u8){}
fn set_grayscale(gray: f32){}
fn set_screen_rgb(red: u8, green: u8, blue: u8){}
fn set_screen_rgba(red: u8, green: u8, blue: u8, alpha: u8){}
fn set_screen_gray(gray: f32){}
//multiplication
fn mul(many: u8){}

//sounds
fn init(){}
fn create_sine(){}
fn create_square(){}
fn create_saw(){}
fn create_noise(){}