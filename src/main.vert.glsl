#version 330

layout(location = 0) in vec4 vs_pos;
layout(location = 1) in vec4 vs_col;
layout(location = 2) in vec2 vs_uv;

out vec4 fs_col;
out vec2 fs_uv; 

uniform mat4 u_ViewProj;
uniform float u_Offset;

void main() {
    // fs_col = vs_col;
    fs_col = vs_col;
    fs_uv = vs_uv;

    vec4 pos = vs_pos;

    pos.z += u_Offset;
    
    pos =  u_ViewProj * pos;
    // pos /= pos.w;

    gl_Position = pos;
    // gl_Position = vs_pos;
}