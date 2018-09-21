#version 450
#define VULKAN 100

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8

layout (set = 0, binding = 0) uniform SceneUBO {
	mat4 view;
	mat4 projection;
	mat4 view_projection;
	vec3 camera_pos;
} scene_ubo;

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

layout (set = 1, binding = 0) uniform UBO {
	PointLight point_lights[MAX_POINT_LIGHTS_COUNT];
	DirectionalLight directional_lights[MAX_DIRECTIONAL_LIGHTS_COUNT];
	uint point_lights_count;
	uint directional_lights_count;
	float window_width;
	float window_height;
	float pixel_x_step;
	float pixel_y_step;
} deferred_ubo;

layout (location = 0) out vec2 out_uv;
layout (location = 1) out vec2 out_texel_coord;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
	out_uv = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);
	gl_Position = vec4(out_uv * 2.0f - 1.0f, 0.0f, 1.0f);
	out_texel_coord = vec2(deferred_ubo.window_width, deferred_ubo.window_height) * out_uv;
}
