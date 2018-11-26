#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 nrm;
layout (location = 2) in vec3 tng;
layout (location = 3) in vec3 btg;
layout (location = 4) in vec2 uv;

layout (set = 0, binding = 0) uniform SceneUBO { Scene s; } scene_ubo;

layout (set = 1, binding = 0) uniform ModelUBO { Model s; } model_ubo;

layout (set = 2, binding = 0) uniform MaterialUBO { Material s; } material_ubo;
layout (set = 2, binding = 1) uniform sampler2D base_color;
layout (set = 2, binding = 2) uniform sampler2D base_color_factor;
layout (set = 2, binding = 3) uniform sampler2D metallic_roughness;
layout (set = 2, binding = 4) uniform sampler2D normal;
layout (set = 2, binding = 5) uniform sampler2D occlusion;
layout (set = 2, binding = 6) uniform sampler2D emissive;
layout (set = 2, binding = 7) uniform sampler2D emissive_factor;

layout (location = 0) out vec4 out_pos;
layout (location = 1) out vec4 out_nrm;
layout (location = 2) out vec4 out_alb;

void main() {
    vec4 alb = texture(base_color, uv) * texture(base_color_factor, uv);
    alb.w *= material_ubo.s.alpha;
    if(alb.w < material_ubo.s.alpha_cutoff) {
        discard;
    }
    out_alb = alb;
    out_pos.xyz = pos;
//   out_pos.w = out_alb.a;
    out_pos.w = 1.0;
    out_nrm.xyz = normalize(mat3(tng, btg, nrm) * ((texture(normal, uv).xyz - 0.5) * 2.0));
//   out_nrm.w = out_alb.a;
    out_nrm.w = 1.0;
  // todo lots of work must be done in here
  // I must add any needed output for deferred part
  // w channel can hold useful info for deferred
  // its highly depends on my pbr render model
}