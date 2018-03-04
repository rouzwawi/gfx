#version 150 core

in vec4 a_Pos;
in vec2 a_TexCoord;
out vec3 v_Pos;
out vec2 v_TexCoord;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

uniform mat4 u_transform;

void main() {
    v_TexCoord = a_TexCoord;
    vec4 p = a_Pos;

    v_Pos = (u_transform * p).xyz;
//    p.x += p.y*p.z*0.3*sin(u_time*1.35);

    gl_Position = u_transform * p;
    gl_ClipDistance[0] = 1.0;
}
