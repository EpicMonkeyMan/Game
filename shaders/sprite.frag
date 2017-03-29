#version 140

in vec3 v_color;
in vec2 v_tex_coords;

out vec4 f_color;

uniform sampler2D tex;

void main() {
    f_color = texture(tex, v_tex_coords) * vec4(v_color, 1.0f);
}