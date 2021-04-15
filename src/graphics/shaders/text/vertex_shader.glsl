#version 330 core

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 texture_coords;

out vec2 glyph_texture_coords;

void main() {
    glyph_texture_coords = vec2(texture_coords);
    gl_Position = vec4(vertex_position.x, vertex_position.y, 0.0, 1.0);
}