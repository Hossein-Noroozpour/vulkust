#version 450
#define VULKAN 100

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (set = 0, binding = 0) uniform SceneUBO {
	mat4 view;
	mat4 projection;
	mat4 view_projection;
  	vec3 camera_pos;
} scene_ubo;

layout (location = 4) out vec2 out_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
	out_uv = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);
	gl_Position = vec4(out_uv * 2.0f - 1.0f, 0.0f, 1.0f);
}
