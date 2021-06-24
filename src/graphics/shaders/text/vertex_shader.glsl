#version 330 core

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 texture_coords;

layout (std140) uniform DrawParameters{
    vec4 viewport;
    uint draw_mode;
    vec2 vertex_shift; // [dx, dy]
    vec4 vertex_rotation; // [cos, sin, rotation_center]
};

out vec2 glyph_texture_coords;

void main() {
    glyph_texture_coords = vec2(texture_coords);

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

    vec2 viewport_offset = vec2(viewport.xy);
    vec2 viewport_size = vec2(viewport.zw);

    position = vec2(
        2 * (position.x + viewport_offset.x) / viewport_size.x - 1.0,
        1.0 - 2 * (position.y + viewport_offset.y) / viewport_size.y
    );

    gl_Position = vec4(position, 0.0, 1.0);
}