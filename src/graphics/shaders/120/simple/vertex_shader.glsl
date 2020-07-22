#version 120

attribute vec2 position; // window coordinate system

uniform vec2 window_center;

void main() {
    vec2 p = vec2(position.x / window_center.x - 1.0,
            1.0 - position.y / window_center.y);
    gl_Position = vec4(position, 0.0, 1.0);
}