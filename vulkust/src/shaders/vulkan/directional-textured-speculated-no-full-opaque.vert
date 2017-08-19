#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;
layout (binding = 0) uniform UBO {
  mat4 mvp;
  sampler2D texture_2d_sampler,
} ubo;
layout (location = 0) out vec2 out_uv;
out gl_PerVertex {
  vec4 gl_Position;
};
void main() {
	out_uv = uv;
	gl_Position = ubo.mvp * vec4(pos, 1.0);
}
