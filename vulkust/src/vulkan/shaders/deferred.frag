#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8

layout (location = 0) in vec2 uv;
layout (location = 1) in vec2 texel_coord;

layout (location = 0) out vec4 out_color;

struct Camera {
	vec3 position;
	mat4 projection;
	mat4 view;
	mat4 view_projection;
};

struct PointLight {
	vec3 color;
	vec3 position;
	float radius;
};

struct DirectionalLight {
	vec3 color;
	vec3 direction;
	mat4 view_projection_biased;
};

layout (set = 0, binding = 0) uniform SceneUBO {
	Camera camera;
	DirectionalLight directional_lights[MAX_DIRECTIONAL_LIGHTS_COUNT];
	uint directional_lights_count;
	PointLight point_lights[MAX_POINT_LIGHTS_COUNT];
	uint point_lights_count;
} scene_ubo;

layout (set = 1, binding = 0) uniform UBO {
	float inverse_samples_count;
	float pixel_x_step;
	float pixel_y_step;
	uint samples_count;
	float window_height;
	float window_width;
} deferred_ubo;

layout (set = 1, binding = 1) uniform sampler2DMS position;
layout (set = 1, binding = 2) uniform sampler2DMS normal;
layout (set = 1, binding = 3) uniform sampler2DMS albedo;
layout (set = 1, binding = 4) uniform sampler2D screen_space_depth;
// layout (set = 1, binding = 5) uniform sampler2D soft_shadow; // todo add soft shadow pass too

vec4 resolve_multisample(sampler2DMS tex, ivec2 int_texel_coord) {
	vec4 result = vec4(0.0);
	for (int i = 0; i < deferred_ubo.samples_count; i++) {
		result += texelFetch(tex, int_texel_coord, i);
	}
	return result * deferred_ubo.inverse_samples_count;
}

void main() {
	ivec2 int_texel_coord = ivec2(texel_coord); 
	vec4 ms_albedo = resolve_multisample(albedo, int_texel_coord);
	out_color = ms_albedo;
	// todo lot of work must be done in here
}