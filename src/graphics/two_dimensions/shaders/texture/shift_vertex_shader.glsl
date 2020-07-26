#version 140

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform vec2 shift;
uniform vec2 window_center;

void main() {
    v_tex_coords = tex_coords;

    float x = (position.x + shift.x)/ window_center.x - 1.0;
    float y = 1.0 - (position.y + shift.y)/ window_center.y;

    gl_Position = vec4(x, y, 0.0, 1.0);
}