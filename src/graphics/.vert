#version 330 core

layout (location = 0) in vec2 a_position;
layout (location = 1) in vec3 a_color;

uniform vec2 u_resolution;
uniform vec2 u_camera_position;
uniform float u_camera_scale;

out vec3 f_color;

void main()
{
    vec2 position = (a_position - u_camera_position) / u_camera_scale;
    if (u_resolution.x > u_resolution.y) {
        position.x *= u_resolution.y / u_resolution.x;
    } else {
        position.y *= u_resolution.x / u_resolution.y;
    }

    gl_Position = vec4(position, 0.0, 1.0);
    f_color = a_color;
}