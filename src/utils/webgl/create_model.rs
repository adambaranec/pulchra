use three_d::renderer::*;
use three_d::core::prelude::*;
use web_sys::*;
use js_sys::{Float32Array,Uint32Array};
use crate::utils::primitives::conversion::conversion::array_to_vec3;
use crate::utils::primitives::generate::normals::generate_normals;
pub fn create_model(gl: &WebGl2RenderingContext, program: &WebGlProgram, positions: &Vec<f32>, indices: &Vec<u32>){
let vertex_buffer = gl.create_buffer();
let element_buffer = gl.create_buffer();
let normal_buffer = gl.create_buffer();
let vertex_location = gl.get_attrib_location(program, "a_vertexPosition") as u32;
let normal_location = gl.get_attrib_location(program, "a_normal") as u32;
let color_location = gl.get_uniform_location(program, "u_color");
let direction_location = gl.get_uniform_location(program, "u_reverseLightDirection");
let normals:Vec<f32> = vec![];
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&positions[ .. ]), WebGl2RenderingContext::DYNAMIC_DRAW);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, element_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, &Uint32Array::from(&indices[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, normal_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&normals[ .. ]), WebGl2RenderingContext::DYNAMIC_DRAW);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());
gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, element_buffer.as_ref());
gl.vertex_attrib_pointer_with_f64(vertex_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(vertex_location);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, normal_buffer.as_ref());
gl.vertex_attrib_pointer_with_f64(normal_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(normal_location);
gl.draw_elements_with_f64(WebGl2RenderingContext::TRIANGLES, indices.len() as i32, WebGl2RenderingContext::UNSIGNED_INT,0.0);
}