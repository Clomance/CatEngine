#version 140

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform float cos;
uniform float sin;
uniform vec2 rotation_center;
uniform vec2 window_center;

void main() {
    v_tex_coords = tex_coords;

    vec2 new_position = vec2(position.x - rotation_center.x, rotation_center.y - position.y);

    new_position = vec2(new_position.x * cos - new_position.y * sin, new_position.x * sin + new_position.y * cos);

    vec2 new_rotation_center = vec2(rotation_center.x - window_center.x, window_center.y - rotation_center.y);

    new_position = new_position + new_rotation_center;

    gl_Position = vec4(
        new_position.x / window_center.x,
        new_position.y / window_center.y,
        0.0,
        1.0
    );
}