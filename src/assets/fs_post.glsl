#version 140

in vec2 f_position;

out vec4 color;

uniform sampler2D screen_texture;
uniform float time_remaining;

vec4 blur13(sampler2D image, vec2 uv, vec2 resolution, vec2 direction);

const float BLUR_AMOUNT = 700;
const float INITIAL_BLUR = 25;

void main() {
    vec2 coordinates = (f_position + 1) / 2;

    vec4 texture_color = texture(screen_texture, coordinates);
    vec4 blurred = blur13(
        screen_texture,
        coordinates,
        vec2(BLUR_AMOUNT, BLUR_AMOUNT) * (1 - time_remaining) + INITIAL_BLUR,
        f_position
    );

    color = blurred * time_remaining + texture_color * (1 - time_remaining);
}

// Copied from https://github.com/Jam3/glsl-fast-gaussian-blur
vec4 blur13(sampler2D image, vec2 uv, vec2 resolution, vec2 direction) {
    vec4 color = vec4(0.0);
    vec2 off1 = vec2(1.411764705882353) * direction;
    vec2 off2 = vec2(3.2941176470588234) * direction;
    vec2 off3 = vec2(5.176470588235294) * direction;
    color += texture2D(image, uv) * 0.1964825501511404;
    color += texture2D(image, uv + (off1 / resolution)) * 0.2969069646728344;
    color += texture2D(image, uv - (off1 / resolution)) * 0.2969069646728344;
    color += texture2D(image, uv + (off2 / resolution)) * 0.09447039785044732;
    color += texture2D(image, uv - (off2 / resolution)) * 0.09447039785044732;
    color += texture2D(image, uv + (off3 / resolution)) * 0.010381362401148057;
    color += texture2D(image, uv - (off3 / resolution)) * 0.010381362401148057;
    return color;
}
