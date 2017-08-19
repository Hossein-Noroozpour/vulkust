#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
layout (binding = 0) uniform UBO {
  mat4 mvp;
  sampler2D texture_2d_sampler,
} ubo;
layout (location = 0) in vec2 uv;
layout (location = 0) out vec4 frag_color;
void main() {
	frag_color = texture(ubo.texture_2d_sampler, uv);
}
