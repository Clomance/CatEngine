#version 120

attribute vec2 position;
attribute vec2 tex_coords;

varying vec2 v_tex_coords;

uniform float cos;
uniform float sin;
uniform vec2 rotation_center;
uniform vec2 window_center;

void main() {
    v_tex_coords = tex_coords;

    vec2 new_position = position - rotation_center;

    new_position = vec2(new_position.x * cos - new_position.y * sin, new_position.x * sin + new_position.y * cos);

    new_position = new_position + rotation_center;

    gl_Position = vec4(
        new_position.x / window_center.x,
        new_position.y / window_center.y,
        0.0,
        1.0
    );
}