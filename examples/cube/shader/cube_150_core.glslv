#version 150 core

in vec4 a_Pos;
in vec2 a_TexCoord;
out vec3 v_Pos;
out vec2 v_TexCoord;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

uniform mat4 u_transform;

#define PI 3.14159265358979323846
#define cursor_ring 0.2
#define cursor_w 0.01

float random(in vec2 st) {
    return fract(
        sin( dot(st.xy, vec2(12.9898,78.233)) )
        * 43758.5453123
    );
}

// 2D Noise based on Morgan McGuire @morgan3d
// https://www.shadertoy.com/view/4dS3Wd
float noise(in vec2 st) {
    vec2 i = floor(st);
    vec2 f = fract(st);

    // Four corners in 2D of a tile
    float a = random(i);
    float b = random(i + vec2(1.0, 0.0));
    float c = random(i + vec2(0.0, 1.0));
    float d = random(i + vec2(1.0, 1.0));

    // Smooth Interpolation

    // Cubic Hermine Curve.  Same as SmoothStep()
    vec2 u = f*f*(3.0-2.0*f);
    // u = smoothstep(0.,1.,f);

    // Mix 4 coorners porcentages
    return mix(a, b, u.x) +
            (c - a)* u.y * (1.0 - u.x) +
            (d - b) * u.x * u.y;
}

float noise_overtone(vec2 st, vec2 move, int n) {
    float f = 1./pow(2.,float(n));
    return noise(move+st/f)*f;
}

vec2 move(float offset) {
    float t = u_time + 100.0 + offset;
    return vec2(t*cos(t*0.112)*0.03, t*sin(t*0.1)*0.02);
}

void main() {

    vec3 clouds = vec3(0.0);
    vec2 vst = a_Pos.xy + vec2(20., 25.0) + move(0.)*.1;
    for (int i = 0; i < 15; i++) {
        clouds.r += noise_overtone(vst, move(+.1), i);
        clouds.g += noise_overtone(vst, move( .0), i);
        clouds.b += noise_overtone(vst, move(-.1), i);
    }

    clouds *= 0.5;
    clouds = clouds*clouds*clouds;

    v_TexCoord = a_TexCoord;
    vec4 p = a_Pos + vec4(clouds, 0.0);

    v_Pos = (u_transform * p).xyz;
//    p.x += p.y*p.z*0.3*sin(u_time*1.35);

    gl_Position = u_transform * p;
    gl_ClipDistance[0] = 1.0;
}
