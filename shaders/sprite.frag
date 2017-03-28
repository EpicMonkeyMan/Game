#version 140

in vec2 v_tex_coords;
in vec3 v_color;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords) * vec4(v_color, 1.0f);
}