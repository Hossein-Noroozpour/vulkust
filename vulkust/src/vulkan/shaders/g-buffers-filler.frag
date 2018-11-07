#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec3 tng;
layout (location = 3) in vec3 btg;
layout (location = 4) in vec2 uv;

struct Camera {
	vec4 position_radius;
	mat4 projection;
	mat4 view;
	mat4 view_projection;
};

struct PointLight {
	vec4 color;
	vec4 position_radius;
};

struct DirectionalLight {
	vec4 color;
	vec4 direction;
};

layout (set = 0, binding = 0) uniform SceneUBO {
	Camera camera;
	DirectionalLight directional_lights[MAX_DIRECTIONAL_LIGHTS_COUNT];
	uint directional_lights_count;
	PointLight point_lights[MAX_POINT_LIGHTS_COUNT];
	uint point_lights_count;
} scene_ubo;

layout (set = 1, binding = 0) uniform ModelUBO {
    mat4 model;
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

layout (location = 0) out vec4 out_pos;
layout (location = 1) out vec4 out_nrm;
layout (location = 2) out vec4 out_alb;

void main() {
    vec4 alb = texture(base_color, uv) * texture(base_color_factor, uv);
    alb.w *= material_ubo.alpha;
    if(alb.w < material_ubo.alpha_cutoff) {
        discard;
    }
    out_alb = alb;
    out_pos.xyz = pos;
//   out_pos.w = out_alb.a;
    out_pos.w = 1.0;
    out_nrm.xyz = normalize(mat3(tng, btg, nrm) * ((texture(normal, uv).xyz - 0.5) * 2.0));
//   out_nrm.w = out_alb.a;
    out_nrm.w = 1.0;
  // todo lots of work must be done in here
  // I must add any needed output for deferred part
  // w channel can hold useful info for deferred
  // its highly depends on my pbr render model
}