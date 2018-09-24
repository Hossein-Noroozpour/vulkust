#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec4 tng;
layout (location = 3) in vec2 uv;

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
	mat4 view_projection_biased;
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

layout (location = 0) out vec3 out_pos;
layout (location = 1) out vec3 out_nrm;
layout (location = 2) out vec3 out_tng;
layout (location = 3) out vec3 out_btg;
layout (location = 4) out vec2 out_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
	out_pos = (model_ubo.model * vec4(pos, 1.0)).xyz;
	mat3 m3_model = mat3(model_ubo.model);
	out_nrm = normalize(m3_model * nrm);
	out_tng = normalize(m3_model * tng.xyz);
	if ( tng.w < 0.0 ) {
		out_btg = cross(out_tng, out_nrm);
	} else {
		out_btg = cross(out_nrm, out_tng);
	}
	out_uv = uv;
	gl_Position = scene_ubo.camera.view_projection * vec4(pos, 1.0);
}
