#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec2 texel_coord;

layout (location = 0) out vec4 out_position;
layout (location = 1) out vec4 out_normal;
layout (location = 2) out vec4 out_albedo;
layout (location = 3) out float out_screen_space_depth;

layout (set = 0, binding = 0) uniform ResolverUBO {
	float inverse_samples_count;
	int samples_count;
	int window_height;
	int window_width;
} resolver_ubo;

layout (set = 0, binding = 1) uniform sampler2DMS position;
layout (set = 0, binding = 2) uniform sampler2DMS normal;
layout (set = 0, binding = 3) uniform sampler2DMS albedo;
layout (set = 0, binding = 4) uniform sampler2DMS screen_space_depth;

void main() {
	ivec2 int_texel_coord = ivec2(texel_coord);
	out_position = vec4(0.0);
    out_normal = vec4(0.0);
    out_albedo = vec4(0.0);
    out_screen_space_depth = 0.0;
	for (int i = 0; i < resolver_ubo.samples_count; ++i) {
		out_position += texelFetch(position, int_texel_coord, i);
		out_normal += texelFetch(normal, int_texel_coord, i);
		out_albedo += texelFetch(albedo, int_texel_coord, i);
		out_screen_space_depth += texelFetch(screen_space_depth, int_texel_coord, i).x;
	}
    out_position *= resolver_ubo.inverse_samples_count;
    out_normal *= resolver_ubo.inverse_samples_count;
    out_albedo *= resolver_ubo.inverse_samples_count;
    out_screen_space_depth *= resolver_ubo.inverse_samples_count;
}