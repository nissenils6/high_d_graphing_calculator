#version 330 core

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_color;
layout (location = 2) in vec3 a_normal;

uniform mat4 u_world_to_screen;
uniform vec3 u_lighting;

out vec3 f_color;

void main()
{
    gl_Position = u_world_to_screen * vec4(a_position, 1.0);

    float lighting = (1 - dot(a_normal, u_lighting)) * 0.5;

    f_color = a_color * lighting;
}