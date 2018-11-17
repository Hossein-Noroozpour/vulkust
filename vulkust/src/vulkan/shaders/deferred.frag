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
	vec4 direction_strength;
};

layout (set = 0, binding = 0) uniform SceneUBO {
	Camera camera;
	DirectionalLight directional_lights[MAX_DIRECTIONAL_LIGHTS_COUNT];
	PointLight point_lights[MAX_POINT_LIGHTS_COUNT];
	uvec4 directional_point_lights_count;
} scene_ubo;

layout (set = 1, binding = 0) uniform DeferredUBO {
	float pixel_x_step;
	float pixel_y_step;
} deferred_ubo;

layout (set = 1, binding = 1) uniform sampler2D position;
layout (set = 1, binding = 2) uniform sampler2D normal;
layout (set = 1, binding = 3) uniform sampler2D albedo;
layout (set = 1, binding = 4) uniform sampler2D screen_space_depth;
layout (set = 1, binding = 5) uniform usampler2D shadow_directional_flagbits;

vec4 alb;

void calc_lights() {
	vec2 start_uv = uv - (vec2(deferred_ubo.pixel_x_step, deferred_ubo.pixel_y_step) * 2.0);
	for(uint light_index = 0; light_index < scene_ubo.directional_point_lights_count.x; ++light_index) {
		float slope = -dot(texture(normal, uv).xyz, scene_ubo.directional_lights[light_index].direction_strength.xyz);
		if(slope < 0.005) {
			continue;
		}
		slope = smoothstep(slope, 0.005, 0.2);
		float brightness = 1.0;
		vec2 shduv = start_uv;
		uint light_flag = 1 << light_index;
		for(uint si = 0; si < 4; ++si, shduv.y = start_uv.y, shduv.x += deferred_ubo.pixel_x_step) {
			for (uint sj = 0; sj < 4; ++sj, shduv.y += deferred_ubo.pixel_y_step) {
				if ((texture(shadow_directional_flagbits, shduv).x & light_flag) == light_flag) {
					brightness -= 0.0625;
				}
			}
		}
		brightness *= slope;
		out_color.xyz += alb.xyz * scene_ubo.directional_lights[light_index].direction_strength.w * brightness; 
	}
}

void main() {
	alb = texture(albedo, uv);
	out_color.xyz = alb.xyz * 0.5; // todo it must come along scene
	calc_lights();
	out_color.w = alb.w;
	// todo lots of work must be done in here
}