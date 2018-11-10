#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (constant_id = 0) const int MAX_DIRECTIONAL_CASCADES_COUNT = 6;
const int MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT = 6;

layout (location = 0) in vec2 uv;

layout (location = 0) out float shadow;
layout (location = 1) out uvec2 flagbits;

layout (set = 0, binding = 0) uniform LightUBO {
	mat4 view_projection_biases[MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT];
    vec4 direction_strength;
    uint cascades_count;
    uint light_index;
} light_ubo;

layout (set = 0, binding = 1) uniform sampler2D position;
layout (set = 0, binding = 2) uniform sampler2D normal;
layout (set = 0, binding = 3) uniform sampler2D shadowmaps[MAX_DIRECTIONAL_CASCADES_COUNT];

void main() {
    vec4 pos = texture(position, uv);
    vec3 nrm = texture(normal, uv).xyz;
    for(int i = 0; i < light_ubo.cascades_count; ++i) {
        vec2 uv;
        float depth;
        {
            vec4 ppos = light_ubo.view_projection_biases[i] * pos;
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
        float dist = texture(shadowmaps[i], uv).x;
        float bias = abs(dot(nrm, light_ubo.direction_strength.xyz)); 
        if (bias < 0.242535624) {
            bias = 0.02;
        } else if (bias > 0.980580677) {
            bias = 0.001;
        } else {
            bias = sqrt(1.0 - (bias * bias)) / bias;
            bias = clamp(0.005 * bias, 0.001, 0.02);
        }
        if(dist + bias < depth) {
            shadow = light_ubo.direction_strength.w;
            if (light_ubo.light_index > 32) {
                flagbits = uvec2(0, 1 << (light_ubo.light_index - 32));
            } else {
                flagbits = uvec2(1 << light_ubo.light_index, 0);
            }
            return;
        }
    }
    discard;
}