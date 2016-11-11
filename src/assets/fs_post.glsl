#version 140

in vec2 f_position;

out vec4 color;

uniform sampler2D screen_texture;

void main() {
    vec2 coordinates = (f_position + 1) / 2;
    color = texture(screen_texture, coordinates);
}
