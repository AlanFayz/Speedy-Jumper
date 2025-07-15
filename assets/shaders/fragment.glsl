#version 100

precision lowp float;

uniform vec2  u_ScreenSize;
uniform float u_Time;

uniform sampler2D Texture;

void main() {
    vec2 coord = gl_FragCoord.xy / u_ScreenSize;
    gl_FragColor = vec4(coord, sin(u_Time) * 0.5 + 0.5, 1.0);
}