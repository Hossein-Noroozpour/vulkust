#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8

layout (location = 0) in vec2 uv;

layout (location = 0) out vec4 out_color;

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

layout (set = 1, binding = 0) uniform DeferredUBO {
	float pixel_x_step;
	float pixel_y_step;
} deferred_ubo;

layout (set = 1, binding = 1) uniform sampler2D position;
layout (set = 1, binding = 2) uniform sampler2D normal;
layout (set = 1, binding = 3) uniform sampler2D albedo;
layout (set = 1, binding = 4) uniform sampler2D screen_space_depth;
layout (set = 1, binding = 5) uniform sampler2D darkness;
layout (set = 1, binding = 6) uniform sampler2D light_flagbits;

float calc_shadow() {
	vec2 shduv;
	vec2 s = uv - (vec2(deferred_ubo.pixel_x_step, deferred_ubo.pixel_y_step) * 2.0);
	shduv = s;
	float d = 0.0;
	for(int i = 0; i < 4; ++i, shduv.y = s.y, shduv.x += deferred_ubo.pixel_x_step) {
		for (int j = 0; j < 4; ++j, shduv.y += deferred_ubo.pixel_y_step) {
			if (texture(darkness, shduv).x > 0.0) {
				d += 0.5;
			} else {
				d += 1.0;
			}
		}
	}
	d *= 1.0 / 16.0;
	return d;
}

void main() {
	vec4 ms_albedo = texture(albedo, uv);
	out_color = ms_albedo;
	out_color.xyz *= calc_shadow(); 
	// todo lots of work must be done in here
}