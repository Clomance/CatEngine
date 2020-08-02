#version 140

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D texture2d;
uniform vec4 colour_filter;

void main() {
    color = colour_filter * texture(texture2d, v_tex_coords);
}