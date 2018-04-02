#version 150 core

in vec3 v_Pos;
in vec2 v_TexCoord;

out vec4 Target0;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

uniform sampler2D t_Color;

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

float soc(vec3 p) {
    vec3 n = normalize(abs(sign(p)+1e6));
    return min(min(dot(p.xy, n.xy), dot(p.yz, n.yz)), dot(p.xz, n.xz));
}

mat2 r2d(float a) {
    float sa=sin(a);
    float ca=cos(a);
    return mat2(ca,sa,-sa,ca);
}

vec2 amod(vec2 p, float m) {
    float a=mod(atan(p.x,p.y), m)-m*.5;
    return vec2(cos(a), sin(a))*length(p);
}

vec3 map(vec3 p) {
    float d = 0.808;
    vec3 o=p;
    float a=mod(o.y+5.0+u_time*.5, 20.0)-10.;
    a = abs(o.y);

    // p.yz *= r2d(sign(a)*u_time*.3);
    p.xz *= r2d(sign(a)*u_time*.05);

    p.xz = amod(p.xz, PI/4.);

    p.xz = max(abs(p.xz)-3.244, -2.608);
    // p.x = mod(p.x+u_time*4., 3.)-1.5;
    p.x = mod(p.x, 3.)-1.5;
    p.z = mod(p.z-u_time*.05, 4.)-2.;


    // p.z += sin(p.x*2.+u_time*50.)*.1;
    // p.z += sin(p.y*2.+u_time*1.)*.1;
    // p.x += sin(p.y*.1+u_time*5.)*.6;

    p.y = mod(p.y+u_time, 6.)-3.;

    // d = min(d, soc(max(abs(p)-0.036, -0.064)));
    d *= min(d, soc(max(abs(p)-0.036, 0.)));

    return vec3(d);
}

void main() {
    //vec4 tex = texture(t_Color, v_TexCoord);
    vec4 tex = vec4(1.0);

//    tex = vec4(1.0);
//    float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
//    tex = mix(vec4(0.0,0.0,0.0,0.0), tex, .5 + blend);

//    tex = vec4(vec3(0.0), 1.0);

    vec2 ar = u_resolution/min(u_resolution.x, u_resolution.y);
    vec2 st = ar * (gl_FragCoord.xy / u_resolution.xy)*2.0 - 1.0;
    vec2 mt = ar * (u_mouse.xy / u_resolution.xy)*2.0 - 1.0;


    // #########################################################################
    vec3 clouds = vec3(0.0);
    vec2 vst = v_TexCoord + vec2(20., 25.0) + move(0.)*.1;
    for (int i = 0; i < 15; i++) {
        clouds.r += noise_overtone(vst, move(+.1), i);
        clouds.g += noise_overtone(vst, move( .0), i);
        clouds.b += noise_overtone(vst, move(-.1), i);
    }

    clouds *= 0.7;
    clouds = clouds*clouds*clouds;

    // #########################################################################
//    vec2 vt = v_TexCoord*2.0 - 1.0;
//    vec3 ro=vec3(vt, 10.),
//      rd=normalize(vec3(vt, -1.)),
//      // rd=vec3(st, -1.),
//      mp;
//    vec3 md;
//    mp = ro;
//
////     mt *= PI/2.;
////     rd.xz *= r2d(mt.x);
////     rd.yz *= r2d(mt.y);
//
//    vec3 cw = vec3(0.208,0.362,0.450);
//    for (int i=0; i<50; i++) {
//        md = map(mp)*1.;
//        if (length(md) <.001) break;
//        mp += rd*md;
//    }
//
//    vec4 rays = vec4(1.-length(ro-mp)*.03);

    // #########################################################################

//    tex = mix(tex, vec4(clouds, 1.0), 0.5);
//    tex = mix(tex, rays, 0.7);

    tex = mix(tex, vec4(0.0), .9*smoothstep(1., 7., v_Pos.z));
    Target0 = tex;
}
