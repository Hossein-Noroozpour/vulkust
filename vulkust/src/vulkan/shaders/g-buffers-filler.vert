#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec4 tng;
layout (location = 3) in vec2 uv;

layout (set = 0, binding = 0) uniform SceneUBO { Scene s; } scene_ubo;

layout (set = 1, binding = 0) uniform ModelUBO { Model s; } model_ubo;

layout (set = 2, binding = 0) uniform MaterialUBO { Material s; } material_ubo;

layout (location = 0) out vec3 out_pos;
layout (location = 1) out vec3 out_nrm;
layout (location = 2) out vec3 out_tng;
layout (location = 3) out vec3 out_btg;
layout (location = 4) out vec2 out_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
	out_pos = (model_ubo.s.model * vec4(pos, 1.0)).xyz;
	mat3 m3_model = mat3(model_ubo.s.model);
	out_nrm = normalize(m3_model * nrm);
	out_tng = normalize(m3_model * tng.xyz);
	if ( tng.w < 0.0 ) {
		out_btg = cross(out_tng, out_nrm);
	} else {
		out_btg = cross(out_nrm, out_tng);
	}
	out_uv = uv;
	gl_Position = scene_ubo.s.camera.view_projection * vec4(out_pos, 1.0);
}
