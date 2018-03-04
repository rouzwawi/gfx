#version 150 core

in vec2 a_Position;
in vec2 a_Translate;
in uint a_Color;

uniform float u_Scale;
uniform float u_time;

out vec4 v_Color;

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
    vec2 noise_move = vec2(0.0);
    for (int i = 0; i < 15; i++) {
        noise_move.x += noise_overtone(a_Position+a_Translate, move(+.1), i);
        noise_move.y += noise_overtone(a_Position+a_Translate, move(-.1), i);
    }

    noise_move = 0.05*noise_move*noise_move*noise_move;

    gl_Position = vec4((a_Position*u_Scale) + a_Translate + noise_move, 0.0, 1.0);

    uint u8mask = 0x000000FFu;
    v_Color = vec4(float( a_Color >> 16),
                   float((a_Color >> 12) & u8mask),
                   float((a_Color >>  8) & u8mask),
                   float( a_Color        & u8mask)) / 255.0;
}
