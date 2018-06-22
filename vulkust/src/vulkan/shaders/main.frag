#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec2 uv;
layout (location = 1) in float diffuse;

layout (location = 0) out vec4 out_color;

layout (binding = 1) uniform sampler2D smp;

void main() 
{
  out_color = texture(smp, uv, 1.0);
}