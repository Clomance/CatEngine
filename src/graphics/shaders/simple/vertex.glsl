#version 330 core

layout (location = 0) in vec4 vertex_position;
layout (location = 1) in vec4 vertex_colour;

layout (std140) uniform DrawParameters{
    mat4 view;
};

uniform mat4 LayerDrawParameters;

out vec4 fragment_colour;

void main() {
    fragment_colour = vertex_colour;

    vec4 position = view * LayerDrawParameters * vertex_position;

    gl_Position = position;
}