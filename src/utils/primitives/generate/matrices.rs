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
pub fn view_projection(camera: &Camera, program: &WebGlProgram, view_loc: &str, proj_loc: &str, gl: &WebGl2RenderingContext){
let mut view_matrix_array:Vec<f32> = vec![];
let mut projection_matrix_array:Vec<f32> = vec![];
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

projection_matrix_array.push(camera.projection().x.x);
projection_matrix_array.push(camera.projection().x.y);
projection_matrix_array.push(camera.projection().x.z);
projection_matrix_array.push(camera.projection().x.w);
projection_matrix_array.push(camera.projection().y.x);
projection_matrix_array.push(camera.projection().y.y);
projection_matrix_array.push(camera.projection().y.z);
projection_matrix_array.push(camera.projection().y.w);
projection_matrix_array.push(camera.projection().z.x);
projection_matrix_array.push(camera.projection().z.y);
projection_matrix_array.push(camera.projection().z.z);
projection_matrix_array.push(camera.projection().z.w);
projection_matrix_array.push(camera.projection().w.x);
projection_matrix_array.push(camera.projection().w.y);
projection_matrix_array.push(camera.projection().w.z);
projection_matrix_array.push(camera.projection().w.w);

gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(program, view_loc).as_ref(), false, &view_matrix_array[ .. ]);
gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(program, proj_loc).as_ref(), false, &projection_matrix_array[ .. ]);

}