#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (constant_id = 0) const int DIRECTIONAL_CASCADES_COUNT = 6;

layout (location = 0) in vec2 uv;

layout (location = 0) out float flagbits;

layout (set = 0, binding = 0) uniform LightUBO { Light s; } light_ubo;
layout (set = 0, binding = 1) uniform sampler2D position;
layout (set = 0, binding = 2) uniform sampler2D normal;
layout (set = 0, binding = 3) uniform sampler2D shadowmaps[DIRECTIONAL_CASCADES_COUNT];

void shade() {
    flagbits = (float(1 << light_ubo.s.light_index) * 
        (1.0 / float(1 << MAX_DIRECTIONAL_LIGHTS_COUNT))) + 
        (1.0 / (float(1 << MAX_DIRECTIONAL_LIGHTS_COUNT) * 4.0 * 
        float(MAX_DIRECTIONAL_LIGHTS_COUNT)));
}

void main() {
    flagbits = 0;
    float bias = dot(texture(normal, uv).xyz, light_ubo.s.direction_strength.xyz);
    if(bias > -0.005) {
        shade();
        return;
    }
    vec4 pos = texture(position, uv);
    bias = abs(bias);
    if (bias < 0.242535624) {
        bias = 0.02;
    } else if (bias > 0.980580677) {
        bias = 0.001;
    } else {
        bias = sqrt(1.0 - (bias * bias)) / bias;
        bias = clamp(0.005 * bias, 0.001, 0.02);
    }
    for(int i = 0; i < light_ubo.s.cascades_count; ++i) {
        vec2 uv;
        float depth;
        {
            vec4 ppos = light_ubo.s.view_projection_biases[i] * pos;
            ppos.xyz = ppos.xyz / ppos.w;
            uv = ppos.xy;
            depth = ppos.z;
        }
        if(uv.x >= 1.0)
            continue;
        if(uv.y >= 1.0)
            continue;
        if(uv.x <= 0.0)
            continue;
        if(uv.y <= 0.0)
            continue;
        if(depth < 0.0)
            continue;
        if(depth > 1.0)
            continue;
        if(texture(shadowmaps[i], uv).x + bias < depth) {
            shade();
            return;
        }
    }
}