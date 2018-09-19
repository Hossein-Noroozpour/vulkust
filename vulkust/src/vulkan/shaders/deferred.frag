#version 450
#define VULKAN 110

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define POINT_LIGHTS_COUNT 6

layout (location = 4) in vec2 uv;

layout (location = 0) out vec4 out_color;

layout (constant_id = 0) const int NUM_SAMPLES = 8;

layout (set = 0, binding = 0) uniform SceneUBO {
	mat4 view;
	mat4 projection;
	mat4 view_projection;
  vec3 camera_pos;
} scene_ubo;

struct PointLight {
	vec4 position;
	vec3 color;
	float radius;
};

layout (set = 1, binding = 0) uniform UBO {
	PointLight point_lights[POINT_LIGHTS_COUNT];
	float window_width;
	float window_height;
  float pixel_x_step;
  float pixel_y_step;
} deferred_ubo;

layout (set = 1, binding = 1) uniform sampler2DMS pos;
layout (set = 1, binding = 2) uniform sampler2DMS nrm;
layout (set = 1, binding = 3) uniform sampler2DMS alb;

void main() {
  // todo lot of work must be done in here
}