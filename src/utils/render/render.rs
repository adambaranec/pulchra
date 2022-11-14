use web_sys::*;
pub fn divide_canvas(gl: &WebGl2RenderingContext, rows: u16, columns: u16){
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64();
    let height = window.inner_height().unwrap().as_f64();
    gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
    if width != None || height != None{
        let scissor_width = width.unwrap() as i32 / rows as i32;
        let scissor_height = height.unwrap() as i32 / columns as i32;
        for c in 0..columns-1{
            for r in 0..rows-1{
            gl.viewport(scissor_width * r as i32, scissor_height * c as i32, scissor_width, scissor_height);
            gl.scissor(scissor_width * r as i32, scissor_height * c as i32, scissor_width, scissor_height);
            }
        }
    }
}
