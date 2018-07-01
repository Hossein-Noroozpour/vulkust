#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec3 tng;
layout (location = 3) in vec2 uv;

layout (binding = 0) uniform UBO {
	mat4 mvp;
} ubo;

layout (location = 0) out vec2 out_uv;
layout (location = 1) out float out_diff;

out gl_PerVertex 
{
    vec4 gl_Position;
};


void main() 
{
	out_diff = dot(nrm, vec3(0.0, 0.0, 1.0)) * dot(tng, vec3(1.0, 1.0, 0.0));
	out_uv = uv;
	gl_Position = ubo.mvp * vec4(pos, 1.0);
}
