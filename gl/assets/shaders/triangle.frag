#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

in UV_OUTPUT {
    vec2 Uv;
} FRAGMENT_UV;

out vec4 Color;

uniform float time;
uniform sampler2D mySampler;

void main()
{
    vec4 textureColor = texture(mySampler, FRAGMENT_UV.Uv);

    Color = textureColor * IN.Color;

//    Color = vec4(IN.Color + vec3(
//                                1.0 * (cos(time) + 1.0) * 0.5,
//                                1.0 * (sin(time) + 1.0) * 0.2,
//                                0),
//                                1.0f
//                 );
}