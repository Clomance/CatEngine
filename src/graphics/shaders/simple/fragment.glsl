#version 330 core

in vec4 fragment_colour;

out vec4 colour;

void main() {
    colour = vec4(fragment_colour);
}