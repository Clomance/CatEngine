#version 330 core

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 texture_coords;
layout (location = 2) in vec4 texture_colour_filter;

uniform vec4 viewport;
uniform uint draw_mode;
uniform vec2 vertex_shift; // [dx, dy]
uniform vec4 vertex_rotation; // [cos, sin, rotation_center]

out vec2 tex_coords;
out vec4 colour_filter;

void main() {
    tex_coords = vec2(texture_coords);

    colour_filter = vec4(texture_colour_filter);

    vec2 position = vec2(vertex_position);

    if ((draw_mode & uint(1)) == uint(1)){
        position += vertex_shift;
    }

    if ((draw_mode & uint(2)) == uint(2)){
        float cos = vertex_rotation.x;
        float sin = vertex_rotation.y;
        vec2 rotation_center = vec2(vertex_rotation.zw);

        position -= rotation_center;
        position = vec2(position.x * cos - position.y * sin, position.x * sin + position.y * cos);
        position += rotation_center;
    }

    vec2 viewport_offset = vec2(viewport.xy);
    vec2 viewport_size = vec2(viewport.zw);

    position = vec2(
        2 * (position.x + viewport_offset.x) / viewport_size.x - 1.0,
        1.0 - 2 * (position.y + viewport_offset.y) / viewport_size.y
    );

    gl_Position = vec4(position, 0.0, 1.0);
}