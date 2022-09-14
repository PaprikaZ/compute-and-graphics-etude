#version 450

layout(binding = 1) uniform sampler2D textureSampler;

layout(location = 0) in vec3 inputFragmentColor;
layout(location = 1) in vec2 inputFragmentTextureCoordinate;

layout(location = 0) out vec4 outputColor;


void main() {
    outputColor = texture(textureSampler, inputFragmentTextureCoordinate);
}