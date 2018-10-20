#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_DIRECTIONAL_CASCADES_COUNT 6

layout (location = 0) in vec2 uv;

layout (location = 0) out float black;

layout (set = 0, binding = 0) uniform LightUBO {
	mat4 view_projections[MAX_DIRECTIONAL_CASCADES_COUNT];
    vec4 direction_strength;
    int cascades_count;
} light_ubo;

layout (set = 0, binding = 1) uniform sampler2D position;
layout (set = 0, binding = 2) uniform sampler2D normal;
layout (set = 0, binding = 3) uniform sampler2D shadowmaps[MAX_DIRECTIONAL_CASCADES_COUNT];

const vec3 max = vec3(1.0, 1.0, 1.0);
const vec3 min = vec3(-1.0, -1.0, 0.0);

void main() {
    vec4 pos = texture(position, uv);
    vec3 nrm = texture(normal, uv).xyz;
    for(int i = 0; i < light_ubo.cascades_count; ++i) {
        vec3 ipos;
        {
            vec4 ppos = light_ubo.view_projections[i] * pos;
            ipos = ppos.xyz / ppos.w;
        }
        if(any(lessThan(max, ipos)) || any(lessThan(ipos, min))) {
            continue;
        }   
        float dist = texture(shadowmaps[i], ipos.xy).x;
        float bias = -dot(nrm, light_ubo.direction_strength.xyz);
        bias = sqrt(1.0 - (bias * bias)) / bias;
        bias = clamp(0.005 * bias, 0.0, 0.02);
        if(dist + bias < ipos.z) {
            black = light_ubo.direction_strength.w;
            return;
        }
    }
    discard;
}