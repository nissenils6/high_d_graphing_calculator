#version 330 core

layout (location = 0) in vec2 a_position;
layout (location = 1) in vec2 a_uv;
layout (location = 2) in vec3 a_color;

out vec2 f_uv;
out vec3 f_color;

void main()
{
    gl_Position = vec4(a_position, 0.0, 1.0);
    f_uv = a_uv;
    f_color = a_color;
}