pub mod render;

fn main() {
    render::create_win();
    render::start_interpreter();
    println!("Everything succeeded! Let's go!");
}
