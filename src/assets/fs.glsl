#version 140

in vec2 f_position;

out vec4 color;

void main() {
    color = vec4(f_position.xy, 1.0, 1.0);
}
