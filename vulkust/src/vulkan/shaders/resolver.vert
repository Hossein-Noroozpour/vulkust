#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (set = 0, binding = 0) uniform ResolverUBO {
	float inverse_samples_count;
	int samples_count;
	int window_height;
	int window_width;
} resolver_ubo;

layout (location = 0) out vec2 out_uv;
layout (location = 1) out vec2 out_texel_coord;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    ivec2 uv = ivec2(gl_VertexIndex & 2, (gl_VertexIndex << 1) & 2);
	out_uv = vec2(uv);
    uv <<= 1;
    uv -= 1;
	gl_Position = vec4(uv, 0.999f, 1.0f);
    uv *= ivec2(resolver_ubo.window_width, resolver_ubo.window_height);
	out_texel_coord = vec2(uv);
}
