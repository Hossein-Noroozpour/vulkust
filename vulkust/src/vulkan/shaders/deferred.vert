#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8

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

layout (set = 1, binding = 0) uniform UBO {
	float inverse_samples_count;
	float pixel_x_step;
	float pixel_y_step;
	uint samples_count;
	float window_height;
	float window_width;
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
