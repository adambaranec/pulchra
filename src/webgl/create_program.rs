use web_sys::*;
fn create_program(vs: &str, fs: &str)->WebGlProgram{
    let vertex_shader_src = "
    attribute vec3 a_vertexPosition;
    attribute vec3 a_normal;
    uniform mat4 u_projection;
    uniform mat4 u_view;
    varying vec3 v_normal;
    void main(){
      gl_Position = u_projection * u_view * vec4(a_vertexPosition, 1.0);
      v_normal = a_normal;
    }
    ";
    let fragment_shader_src = "
    precision mediump float;
    varying vec3 v_normal;
    uniform vec3 u_reverseLightDirection;
    uniform vec4 u_color;
    void main(){
      vec3 normal = normalize(v_normal);
      float light = dot(normal, u_reverseLightDirection);
      gl_FragColor = u_color;
      gl_FragColor.rgb *= light;
    }
    ";
    let vertex_shader = gl.create_shader(WebGl2RenderingContext::VERTEX_SHADER).unwrap();
    let fragment_shader = gl.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER).unwrap();
    let program = gl.create_program().unwrap();
    gl.shader_source(&vertex_shader, vs);
    gl.shader_source(&fragment_shader, fs);
    gl.compile_shader(&vertex_shader);
    gl.compile_shader(&fragment_shader);
    gl.attach_shader(&program, &vertex_shader); 
    gl.attach_shader(&program, &fragment_shader);

    program
}