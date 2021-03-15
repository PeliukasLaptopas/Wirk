#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec2 UV_IN;

out VS_OUTPUT {
    vec3 Color;
} OUT;

out UV_OUTPUT {
    vec2 Uv;
} UV;

uniform mat4 P;

void main()
{
    gl_Position = P * vec4(Position, 1.0);

    gl_Position.z = 0.0;
    gl_Position.w = 1.0;

    OUT.Color = Color.xyz;
    UV.Uv = UV_IN;
}