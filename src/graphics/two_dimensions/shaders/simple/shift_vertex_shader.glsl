#version 140

in vec2 position; // window coordinate system

uniform vec2 shift; // window coordinate system
uniform vec2 window_center;

void main() {
    float x = (position.x + shift.x)/ window_center.x - 1.0;
    float y = 1.0 - (position.y + shift.y)/ window_center.y;
    gl_Position = vec4(x, y, 0.0, 1.0);
}