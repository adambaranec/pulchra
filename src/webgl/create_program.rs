use web_sys::*;
fn create_program(vs: &str, fs: &str)->WebGlProgram{
    let vertex_shader = gl.create_shader(WebGl2RenderingContext::VERTEX_SHADER).unwrap();
    let fragment_shader = gl.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER).unwrap();
    let program = gl.create_program().unwrap();
    gl.shader_source(&vertex_shader, vs);
    gl.shader_source(&fragment_shader, fs);
    gl.compile_shader(&vertex_shader);
    gl.compile_shader(&fragment_shader);
    gl.attach_shader(&program, &vertex_shader); 
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);
    
    program
}