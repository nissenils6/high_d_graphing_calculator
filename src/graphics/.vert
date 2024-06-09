#version 330 core

// VertexBuffer: [(position: vec3, color: vec3, normal: vec3), ...]
// IndexBuffer: [i32, i32, i32, ...]
// Uniforms: Resolution: vec2, Camera Position: vec3, Camera Orientation: mat3, Lighting: vec3

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_color;
layout (location = 2) in vec3 a_normal;

uniform vec2 u_resolution;
uniform vec3 u_camera_position;
uniform mat3 u_camera_orientation;
uniform vec3 u_lighting;

out vec3 f_color;

void main()
{
    vec3 rel_to_camera_pos = a_position - u_camera_position;
    vec3 projected_pos = u_camera_orientation * rel_to_camera_pos;

    if (u_resolution.x > u_resolution.y) {
        projected_pos.x *= u_resolution.y / u_resolution.x;
    } else {
        projected_pos.y *= u_resolution.x / u_resolution.y;
    }

    // projected_pos.x /= 4.0 * projected_pos.z;
    // projected_pos.y /= 4.0 * projected_pos.z;

    gl_Position = vec4(projected_pos, 1.0);

    float lighting = (1 - dot(a_normal, u_lighting)) * 0.5;

    f_color = a_color * lighting;
}