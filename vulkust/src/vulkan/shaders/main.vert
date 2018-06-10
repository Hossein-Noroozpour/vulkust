#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

layout (binding = 0) uniform UBO 
{
	mat4 mvp;
} ubo;

layout (location = 0) out vec3 out_color;

out gl_PerVertex 
{
    vec4 gl_Position;   
};


void main() 
{
	out_color = color;
	gl_Position = ubo.mvp * vec4(pos, 1.0);
}
