#version 140

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

// transform shift:
// column 0 (for opengl) - transform shift
// column 1 (for opengl) - posttransform shift
uniform mat2 transform_shift;
uniform mat2 transform_matrix;

uniform vec2 window_center;

void main() {
    v_tex_coords = tex_coords;

    vec2 new_position = vec2(
        position.x  - window_center.x + transform_shift[0].x,
        window_center.y - position.y + transform_shift[0].y
    );

    new_position = (transform_matrix * new_position + transform_shift[1]);

    gl_Position = vec4(
        new_position.x / window_center.x,
        new_position.y / window_center.y,
        0.0,
        1.0
    );
}