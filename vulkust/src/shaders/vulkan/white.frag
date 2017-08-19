#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
layout (binding = 0) uniform UBO {
	mat4 mvp;
} ubo;
layout (location = 0) out vec4 frag_color;
void main() {
  frag_color = vec4(1.0);
}
