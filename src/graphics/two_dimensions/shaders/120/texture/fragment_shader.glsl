#version 120

varying vec2 v_tex_coords;


uniform sampler2D texture2d;
uniform vec4 colour_filter;

void main() {
    gl_FragColor = colour_filter * texture2D(texture2d, v_tex_coords);
}