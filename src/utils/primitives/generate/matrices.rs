use three_d::core::*;
use web_sys::*;
pub fn fill_program_matrices(camera: &Camera, program: &WebGlProgram, proj_location: &str, view_location: &str, gl: &WebGl2RenderingContext){
    let mut model_matrix_array:Vec<f32> = vec![];
    let mut view_matrix_array:Vec<f32> = vec![];
    model_matrix_array.push(camera.projection().x.x);
    model_matrix_array.push(camera.projection().x.y);
    model_matrix_array.push(camera.projection().x.z);
    model_matrix_array.push(camera.projection().x.w);
    model_matrix_array.push(camera.projection().y.x);
    model_matrix_array.push(camera.projection().y.y);
    model_matrix_array.push(camera.projection().y.z);
    model_matrix_array.push(camera.projection().y.w);
    model_matrix_array.push(camera.projection().z.x);
    model_matrix_array.push(camera.projection().z.y);
    model_matrix_array.push(camera.projection().z.z);
    model_matrix_array.push(camera.projection().z.w);
    model_matrix_array.push(camera.projection().w.x);
    model_matrix_array.push(camera.projection().w.y);
    model_matrix_array.push(camera.projection().w.z);
    model_matrix_array.push(camera.projection().w.w);

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

    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(program, proj_location).as_ref(), false, &model_matrix_array[ .. ]);
    gl.uniform_matrix4fv_with_f32_array(gl.get_uniform_location(program, view_location).as_ref(), false, &view_matrix_array[ .. ]);
}