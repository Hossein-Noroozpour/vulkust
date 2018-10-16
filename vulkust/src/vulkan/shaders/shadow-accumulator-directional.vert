#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) out vec2 out_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    int uvx = gl_VertexIndex & 2;
    int uvy = (gl_VertexIndex << 1) & 2;
	out_uv = vec2(float(uvx), float(uvy));
    int posx = (uvx << 1) - 1;
    int posy = (uvy << 1) - 1;
	gl_Position = vec4(float(posx), float(posy), 0.999f, 1.0f);
}
