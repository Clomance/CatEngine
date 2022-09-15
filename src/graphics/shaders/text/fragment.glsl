#version 330 core

in vec3 texture_coordinates;
in vec4 colour_filter;

out vec4 colour;

uniform sampler2DArray glyph_map;

void main() {
    colour = vec4(colour_filter.xyz, colour_filter.w * texture(glyph_map, texture_coordinates).r);
}