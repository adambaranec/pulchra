use web_sys::*;
fn divide_canvas(gl: &WebGl2RenderingContext, rows: u32, columns: u32){
if gl.get_parameter(WebGl2RenderingContext::SCISSOR_TEST).unwrap().as_bool() == None{
    gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
    let scissor_width = gl.drawing_buffer_width() / rows as i32;
    let scissor_height = gl.drawing_buffer_height() / columns as i32;
    for c in 0..columns-1{
        for r in 0..rows-1{
        gl.viewport(scissor_width * r as i32, scissor_height * c as i32, scissor_width, scissor_height);
        gl.scissor(scissor_width * r as i32, scissor_height * c as i32, scissor_width, scissor_height);
        }
    }
} else {
  let scissor_width = gl.drawing_buffer_width() / rows as i32;
  let scissor_height = gl.drawing_buffer_height() / columns as i32;
  for c in 0..columns-1{
    for r in 0..rows-1{
    gl.viewport(scissor_width * r as i32, scissor_height * c as i32, scissor_width, scissor_height);
    gl.scissor(scissor_width * r as i32, scissor_height * c as i32, scissor_width, scissor_height);
    }
}
}
}