#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec3 tng;
layout (location = 3) in vec3 btg;
layout (location = 4) in vec2 uv;

layout (location = 0) out vec4 out_color;

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

layout (set = 2, binding = 1) uniform sampler2D base_color;
layout (set = 2, binding = 2) uniform sampler2D base_color_factor;
layout (set = 2, binding = 3) uniform sampler2D metallic_roughness;
layout (set = 2, binding = 4) uniform sampler2D normal;
layout (set = 2, binding = 5) uniform sampler2D occlusion;
layout (set = 2, binding = 6) uniform sampler2D emissive;
layout (set = 2, binding = 7) uniform sampler2D emissive_factor;

void main() 
{
  out_color = texture(base_color, uv);
  // todo lot of work must be done in here
}