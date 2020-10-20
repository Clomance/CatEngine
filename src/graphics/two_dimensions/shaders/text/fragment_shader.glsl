#version 140

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D texture2d;
uniform vec4 colour;

void main() {
    color = vec4(colour.xyz, colour.w * texture(texture2d, v_tex_coords));
}