#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (constant_id = 0) const int MAX_DIRECTIONAL_CASCADES_COUNT = 6;

layout (location = 0) in vec2 uv;

layout (location = 0) out float shadow;
layout (location = 1) out uvec2 flagbits;

layout (set = 0, binding = 0) uniform LightUBO {
	mat4 view_projection_biases[MAX_DIRECTIONAL_CASCADES_COUNT];
    vec4 direction_strength;
    uint cascades_count;
    uint light_index;
} light_ubo;

layout (set = 0, binding = 1) uniform sampler2D position;
layout (set = 0, binding = 2) uniform sampler2D normal;
layout (set = 0, binding = 3) uniform sampler2D shadowmaps[MAX_DIRECTIONAL_CASCADES_COUNT];

const vec2 maxxy = vec2(1.0, 1.0);
const vec2 minxy = vec2(0.0, 0.0);

void main() {
    vec4 pos = texture(position, uv);
    vec3 nrm = texture(normal, uv).xyz;
    for(int i = 0; i < light_ubo.cascades_count; ++i) {
        vec3 ipos;
        {
            vec4 ppos = light_ubo.view_projection_biases[i] * pos;
            ipos = ppos.xyz / ppos.w;
        }
        if(any(lessThan(maxxy, ipos.xy)) || any(lessThan(ipos.xy, minxy))) {
            continue;
        }
        float dist = texture(shadowmaps[i], ipos.xy).x;
        float bias = -dot(nrm, light_ubo.direction_strength.xyz);
        bias = sqrt(1.0 - (bias * bias)) / bias;
        bias = clamp(0.005 * bias, 0.0, 0.02);
        if(dist + bias < ipos.z) {
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