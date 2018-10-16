#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8

layout (location = 0) in vec2 uv;

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

layout (set = 1, binding = 1) uniform sampler2D base_color;
layout (set = 1, binding = 2) uniform sampler2D base_color_factor;
layout (set = 1, binding = 3) uniform sampler2D metallic_roughness;
layout (set = 1, binding = 4) uniform sampler2D normal;
layout (set = 1, binding = 5) uniform sampler2D occlusion;
layout (set = 1, binding = 6) uniform sampler2D emissive;
layout (set = 1, binding = 7) uniform sampler2D emissive_factor;

void main() {
    vec4 alb = texture(base_color, uv) * texture(base_color_factor, uv);
    alb.w *= material_ubo.alpha;
    if(alb.w < material_ubo.alpha_cutoff) {
        discard;
    }
}