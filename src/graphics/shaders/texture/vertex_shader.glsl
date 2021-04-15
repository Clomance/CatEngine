#version 330 core

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 texture_coords;
layout (location = 2) in vec4 texture_colour_filter;

out vec2 tex_coords;
out vec4 colour_filter;

uniform vec2 window_half_size;

uniform uint draw_mode;

uniform vec2 vertex_shift; // [dx, dy]

uniform vec4 vertex_rotation; // [cos, sin, rotation_center]

void main() {
    tex_coords = vec2(texture_coords);

    colour_filter = vec4(texture_colour_filter);

    vec2 position = vec2(vertex_position);

    if ((draw_mode & 1) == 1){
        position += vertex_shift;
    }

    if ((draw_mode & 2) == 2){
        float cos = vertex_rotation.x;
        float sin = vertex_rotation.y;
        vec2 rotation_center = vec2(vertex_rotation.zw);

        position -= rotation_center;
        position = vec2(position.x * cos - position.y * sin, position.x * sin + position.y * cos);
        position += rotation_center;
    }

    position = vec2(
        position.x / window_half_size.x - 1.0,
        1.0 - position.y / window_half_size.y
    );

    gl_Position = vec4(position, 0.0, 1.0);
}