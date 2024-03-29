#version 330 core

in vec2 glyph_texture_coords;

out vec4 colour;

uniform sampler2D glyph_texture_2d;
uniform vec4 glyph_colour;

void main() {
    colour = vec4(glyph_colour.xyz, glyph_colour.w * texture(glyph_texture_2d, glyph_texture_coords).r);
}