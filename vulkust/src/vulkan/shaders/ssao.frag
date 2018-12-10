#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (location = 0) in vec2 uv;

layout (location = 0) out float ambient_occlusion;

layout (set = 0, binding = 0) uniform SceneUBO { Scene s; } scene_ubo;

layout (set = 1, binding = 0) uniform SSAOUBO { SSAO s; } ssao_ubo;
layout (set = 1, binding = 1) uniform sampler2D position;
layout (set = 1, binding = 2) uniform sampler2D normal;
layout (set = 1, binding = 3) uniform sampler2D screen_space_depth;

void main() {
	const vec3 pos = texture(position, uv).xyz;
	const vec3 nrm = texture(normal, uv).xyz;
    float tmpf = 1.0 - abs(nrm.x);
    vec3 tmpv3;
	if (tmpf < NORMAL_EPSILON && tmpf > -NORMAL_EPSILON) {
		tmpv3 = vec3(0.0, 1.0, 0.0);
	} else {
		tmpv3 = vec3(1.0, 0.0, 0.0);
	}
	const vec3 btg = cross(nrm, tmpv3);
    const vec3 tng = cross(btg, nrm);
	const mat3 tbn = mat3(tng, btg, nrm);
    const uint samples_count = uint(scene_ubo.s.ssao_config.x);
    const float radius = scene_ubo.s.ssao_config.y;
    ambient_occlusion = 1.0;
    for(uint sample_index = 0; sample_index < samples_count; ++sample_index) {
        vec4 sample_pos;
        sample_pos.xyz = ssao_ubo.s.sample_vectors[sample_index].xyz * radius;
        sample_pos.xyz = tbn * sample_pos.xyz;
        sample_pos.xyz += pos;
        sample_pos.w = 1.0;
        sample_pos = scene_ubo.s.camera.uniform_view_projection * sample_pos;
        sample_pos.w = 1 / sample_pos.w;
        sample_pos.xyz *= sample_pos.w;
        if (
            sample_pos.x > 1.0 - SMALL_EPSILON || sample_pos.x < 0.0 + SMALL_EPSILON || 
            sample_pos.y > 1.0 - SMALL_EPSILON || sample_pos.y < 0.0 + SMALL_EPSILON) {
            continue;
        }
        sample_pos.x = texture(screen_space_depth, sample_pos.xy).x;
        sample_pos.y = abs(scene_ubo.s.ssao_config.z * sample_pos.w); // z-tolerance
        if (sample_pos.x < sample_pos.z && sample_pos.x + sample_pos.y > sample_pos.z) {
            ambient_occlusion += ssao_ubo.s.sample_vectors[sample_index].w;
        }
    }
}