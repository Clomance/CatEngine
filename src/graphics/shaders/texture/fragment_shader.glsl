#version 330 core

in vec2 tex_coords;
in vec4 colour_filter;

out vec4 colour;

uniform sampler2D texture_2d;

void main() {
    colour = colour_filter * texture(texture_2d, tex_coords);
}