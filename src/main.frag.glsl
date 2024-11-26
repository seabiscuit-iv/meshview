#version 330 core

in vec4 fs_col;
in vec2 fs_uv;
out vec4 frag_color;

void main() {
    // frag_color = vec4(fs_uv);  // Sample the texture
    // frag_color = vec4(1 - fs_uv, 0, 1);
    frag_color = fs_col;
}