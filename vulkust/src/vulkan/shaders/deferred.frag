#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define POINT_LIGHTS_COUNT 32
#define DIRECTIONAL_LIGHTS_COUNT 8

layout (location = 0) in vec2 uv;
layout (location = 1) in vec2 texel_coord;

layout (location = 0) out vec4 out_color;

struct PointLight {
	vec3 position;
	vec3 color;
	float radius;
};

struct DirectionalLight {
	vec3 direction;
	vec3 color;
	mat4 view_projection_biased;
};

layout (set = 0, binding = 0) uniform UBO {
	PointLight point_lights[POINT_LIGHTS_COUNT];
	DirectionalLight directional_lights[DIRECTIONAL_LIGHTS_COUNT];
	uint point_lights_count;
	uint directional_lights_count;
	uint samples_count;
	float inverse_samples_count;
	float window_width;
	float window_height;
	float pixel_x_step;
	float pixel_y_step;
} deferred_ubo;

layout (set = 0, binding = 1) uniform sampler2DMS position;
layout (set = 0, binding = 2) uniform sampler2DMS normal;
layout (set = 0, binding = 3) uniform sampler2DMS albedo;
layout (set = 0, binding = 4) uniform sampler2D screen_space_depth;
// layout (set = 0, binding = 5) uniform sampler2D soft_shadow; // todo add soft shadow pass too

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