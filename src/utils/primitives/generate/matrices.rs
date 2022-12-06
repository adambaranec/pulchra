use three_d::core::*;
use web_sys::*;
pub fn view_mat_to_program(camera: &Camera, program: &WebGlProgram, view_location: &str, gl: &WebGl2RenderingContext){
    let mut view_matrix_array:Vec<f32> = vec![];
    view_matrix_array.push(camera.view().x.x);
    view_matrix_array.push(camera.view().x.y);
    view_matrix_array.push(camera.view().x.z);
    view_matrix_array.push(camera.view().x.w);
    view_matrix_array.push(camera.view().y.x);
    view_matrix_array.push(camera.view().y.y);
    view_matrix_array.push(camera.view().y.z);
    view_matrix_array.push(camera.view().y.w);
    view_matrix_array.push(camera.view().z.x);
    view_matrix_array.push(camera.view().z.y);
    view_matrix_array.push(camera.view().z.z);
    view_matrix_array.push(camera.view().z.w);
    view_matrix_array.push(camera.view().w.x);
    view_matrix_array.push(camera.view().w.y);
    view_matrix_array.push(camera.view().w.z);
    view_matrix_array.push(camera.view().w.w);
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(program, view_location).as_ref(), false, &view_matrix_array[ .. ]);
}