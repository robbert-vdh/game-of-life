#version 140

in vec2 v_position;

void main() {
    gl_Position = vec4(v_position, 0.0, 1.0);
}
