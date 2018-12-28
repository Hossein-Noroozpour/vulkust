#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (location = 0) in vec2 uv;

layout (set = 0, binding = 0) uniform ModelUBO { Model s; } model_ubo;

layout (set = 1, binding = 0) uniform MaterialUBO { Material s; } material_ubo;
layout (set = 1, binding = 1) uniform sampler2D base_color;
layout (set = 1, binding = 2) uniform sampler2D base_color_factor;
layout (set = 1, binding = 3) uniform sampler2D metallic_roughness;
layout (set = 1, binding = 4) uniform sampler2D normal;
layout (set = 1, binding = 5) uniform sampler2D occlusion;
layout (set = 1, binding = 6) uniform sampler2D emissive;
layout (set = 1, binding = 7) uniform sampler2D emissive_factor;

layout (location = 0) out vec4 out_color;

void main() {
    out_color = texture(base_color, uv) * texture(base_color_factor, uv);
}