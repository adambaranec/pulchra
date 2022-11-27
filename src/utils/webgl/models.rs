use three_d::renderer::*;
use three_d::core::prelude::*;
use web_sys::*;
use js_sys::{Float32Array,Uint16Array};

pub fn model_with_pos(gl: &WebGl2RenderingContext, program: &WebGlProgram, vertex_attrib_name: &str, positions: &Vec<f32>){
let vertex_buffer = gl.create_buffer();
let vertex_location = gl.get_attrib_location(program, vertex_attrib_name) as u32;
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&positions[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.vertex_attrib_pointer_with_f64(vertex_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(vertex_location);
gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (positions.len() / 3) as i32);
}

pub fn model_with_pos_indices(gl: &WebGl2RenderingContext, program: &WebGlProgram, vertex_attrib_name: &str, positions: &Vec<f32>, indices: &Vec<u16>){
let vertex_buffer = gl.create_buffer();
let element_buffer = gl.create_buffer();
let vertex_location = gl.get_attrib_location(program, vertex_attrib_name) as u32;
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&positions[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, element_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, &Uint16Array::from(&indices[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());
gl.vertex_attrib_pointer_with_f64(vertex_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(vertex_location);
gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, element_buffer.as_ref());
gl.draw_elements_with_f64(WebGl2RenderingContext::TRIANGLES, (indices.len() / 3) as i32, WebGl2RenderingContext::UNSIGNED_INT,0.0);
}

pub fn model_with_pos_indices_normals(gl: &WebGl2RenderingContext, program: &WebGlProgram, vertex_attrib_name: &str, normal_attrib_name: &str, positions: &Vec<f32>, normals: &Vec<f32>, indices: &Vec<u16>){
let vertex_buffer = gl.create_buffer();
let normal_buffer = gl.create_buffer();
let element_buffer = gl.create_buffer();
let vertex_location = gl.get_attrib_location(program, vertex_attrib_name) as u32;
let normal_location = gl.get_attrib_location(program, normal_attrib_name) as u32;
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&positions[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.vertex_attrib_pointer_with_f64(vertex_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(vertex_location);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, normal_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&normals[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.vertex_attrib_pointer_with_f64(normal_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(normal_location);
gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, element_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, &Uint16Array::from(&indices[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.draw_elements_with_f64(WebGl2RenderingContext::TRIANGLES, (indices.len() / 3) as i32, WebGl2RenderingContext::UNSIGNED_INT,0.0);
}

pub fn model_with_pos_normals(gl: &WebGl2RenderingContext, program: &WebGlProgram, vertex_attrib_name: &str, normal_attrib_name: &str, positions: &Vec<f32>, normals: &Vec<f32>){
let vertex_buffer = gl.create_buffer();
let normal_buffer = gl.create_buffer();
let vertex_location = gl.get_attrib_location(program, vertex_attrib_name) as u32;
let normal_location = gl.get_attrib_location(program, normal_attrib_name) as u32;
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertex_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&positions[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.vertex_attrib_pointer_with_f64(vertex_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(vertex_location);
gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, normal_buffer.as_ref());
gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &Float32Array::from(&normals[ .. ]), WebGl2RenderingContext::STATIC_DRAW);
gl.vertex_attrib_pointer_with_f64(normal_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0.0);
gl.enable_vertex_attrib_array(normal_location);
gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (positions.len() / 3) as i32);
}