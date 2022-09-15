#version 330 core

layout (location = 0) in vec4 vertex_position;
layout (location = 1) in vec4 vertex_colour;
layout (location = 2) in vec2 vertex_texture_coordinates;

layout (std140) uniform DrawParameters{
    mat4 view;
};

uniform mat4 LayerDrawParameters;

out vec2 texture_coordinates;
out vec4 colour_filter;

void main() {
    texture_coordinates = vertex_texture_coordinates;
    colour_filter = vertex_colour;

    vec4 position = view * LayerDrawParameters * vertex_position;

    gl_Position = position;
}