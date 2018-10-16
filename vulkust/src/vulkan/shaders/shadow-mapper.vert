#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec4 tng;
layout (location = 3) in vec2 uv;

layout (set = 0, binding = 0) uniform ModelShadowUBO {
	mat4 model_view_projection;
} model_shadow_ubo;

layout (set = 1, binding = 0) uniform MaterialUBO {
    float alpha;
    float alpha_cutoff;
	float metallic_factor;
    float normal_scale;
    float occlusion_strength;
    float roughness_factor;
} material_ubo;

layout (location = 0) out vec2 out_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    out_uv = uv;
	gl_Position = model_shadow_ubo.model_view_projection * vec4(pos, 1.0);
}
