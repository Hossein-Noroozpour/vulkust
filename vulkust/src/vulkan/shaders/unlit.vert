#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec4 tng;
layout (location = 3) in vec2 uv;

layout (set = 0, binding = 0) uniform ModelUBO { Model s; } model_ubo;

layout (set = 1, binding = 0) uniform MaterialUBO { Material s; } material_ubo;

layout (location = 0) out vec2 out_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
	out_uv = uv;
	gl_Position = model_ubo.s.model_view_projection * vec4(pos, 1.0);
}
