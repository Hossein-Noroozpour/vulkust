#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
layout (binding = 1) uniform sampler2D texture_2d_sampler;
layout (location = 0) in vec2 uv;
layout (location = 0) out vec4 frag_color;
void main() {
	frag_color = texture(texture_2d_sampler, uv);
}
