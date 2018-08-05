#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec4 tng;
layout (location = 3) in vec2 uv;

layout (set = 0, binding = 0) uniform SceneUBO {
	mat4 view_projection;
} scene_ubo;

layout (set = 1, binding = 0) uniform ModelUBO {
	mat4 model;
	mat4 model_view_projection;
} model_ubo;

layout (set = 2, binding = 0) uniform MaterialUBO {
    float alpha;
    float alpha_cutoff;
	float metallic_factor;
    float normal_scale;
    float occlusion_strength;
    float roughness_factor;
} material_ubo;

layout (location = 0) out vec3 out_pos;
layout (location = 1) out vec3 out_nrm;
layout (location = 2) out vec3 out_tng;
layout (location = 3) out vec3 out_btg;
layout (location = 4) out vec2 out_uv;

out gl_PerVertex 
{
    vec4 gl_Position;
};


void main() 
{
	out_pos = (model_ubo.model * vec4(pos, 1.0)).xyz;
	out_nrm = (model_ubo.model * vec4(nrm, 1.0)).xyz;
	out_tng = (model_ubo.model * vec4(tng.xyz, 1.0)).xyz;
	if ( tng.w < 0.0 ) {
		out_btg = cross(out_tng.xyz, out_nrm);
	} else {
		out_btg = cross(out_nrm, out_tng.xyz);
	}
	out_uv = uv;
	gl_Position = model_ubo.model_view_projection * vec4(pos, 1.0);
}
