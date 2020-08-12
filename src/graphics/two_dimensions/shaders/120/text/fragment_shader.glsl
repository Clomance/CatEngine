#version 120

varying vec2 v_tex_coords;

uniform sampler2D texture2d;
uniform vec4 colour;

void main() {
    gl_FragColor = vec4(colour.xyz, colour.w * texture(texture2d, v_tex_coords));
}