  attribute vec3 a_vertexPosition;
    attribute vec3 a_normal;
    uniform mat4 u_projection;
    uniform mat4 u_view;
    varying vec3 v_normal;
    void main(){
      gl_Position = u_projection * u_view * vec4(a_vertexPosition, 1.0);
      v_normal = a_normal;
    }