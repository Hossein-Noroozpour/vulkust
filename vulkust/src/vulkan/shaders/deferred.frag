#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (location = 0) in vec2 uv;

layout (location = 0) out vec4 out_color;

layout (set = 0, binding = 0) uniform SceneUBO { Scene s; } scene_ubo;

layout (set = 1, binding = 0) uniform DeferredUBO { Deferred s; } deferred_ubo;
layout (set = 1, binding = 1) uniform sampler2D position;
layout (set = 1, binding = 2) uniform sampler2D normal;
layout (set = 1, binding = 3) uniform sampler2D albedo;
layout (set = 1, binding = 4) uniform sampler2D screen_space_depth;
layout (set = 1, binding = 5) uniform sampler2D ambient_occlusion;
layout (set = 1, binding = 6) uniform sampler2D shadow_directional_flagbits;

vec3 calc_lights(const vec3 alb, const vec3 nrm, const vec3 eye, const float roughness, const float metallic) {
	const vec2 start_uv = uv - (vec2(deferred_ubo.s.pixel_step.x, deferred_ubo.s.pixel_step.y) * float((BLUR_KERNEL_LENGTH - 1) >> 1));
	float directional_lights_shadowness[MAX_DIRECTIONAL_LIGHTS_COUNT];
	for(uint i = 0; i < MAX_DIRECTIONAL_LIGHTS_COUNT; ++i) {
		directional_lights_shadowness[i] = 1.0;
	}
	vec2 shduv = start_uv;
	for(uint si = 0; si < BLUR_KERNEL_LENGTH; ++si, shduv.y = start_uv.y, shduv.x += deferred_ubo.s.pixel_step.x) {
		for (uint sj = 0; sj < BLUR_KERNEL_LENGTH; ++sj, shduv.y += deferred_ubo.s.pixel_step.y) {
			const vec4 flags_f = texture(shadow_directional_flagbits, shduv); // it can be used for future
			const uint directional_flags = uint((flags_f.x * float(1 << MAX_DIRECTIONAL_LIGHTS_COUNT)) + 0.5);
			for(uint light_index = 0, light_flag = 1; light_index < scene_ubo.s.lights_count.x; ++light_index, light_flag <<= 1) {
				if ((directional_flags & light_flag) == light_flag) {
					directional_lights_shadowness[light_index] += (-1.0 / float(BLUR_KERNEL_LENGTH * BLUR_KERNEL_LENGTH));
				}
			}
		}
	}
	const vec3 base_reflectivity = mix(vec3(0.02), alb, metallic);
	vec3 result = vec3(0.0);
	for(uint light_index = 0; light_index < scene_ubo.s.lights_count.x; ++light_index) {
		const vec3 l = normalize(-scene_ubo.s.directional_lights[light_index].direction_strength.xyz);
		const float slope = dot(nrm, l);
		if(slope < 0.005) {
			continue;
		}
		const vec3 halfway = normalize(eye + l);
		const vec3 radiance = scene_ubo.s.directional_lights[light_index].color.xyz;
		const float distribution = NDFTRGGX(nrm, halfway, roughness);
		const float geometry = GFSCHGGX(nrm, eye, l, roughness);
		const vec3 fresnel = FFSCHGGX(clamp(dot(halfway, eye), 0.0, 1.0), base_reflectivity);
		const vec3 kd = (vec3(1.0) - fresnel) * (1.0 - metallic);
		const vec3 nom = distribution * geometry * fresnel;
		const float denom = 4 * max(dot(nrm, eye), 0.0) * max(dot(nrm, l), 0.0);
        const vec3 specular = nom / max(denom, 0.001);
		result += (kd * (alb / VX_PI) + specular) * radiance * smoothstep(0.005, 1.0, slope) * directional_lights_shadowness[light_index];
	}
	return result;
}

void main() {
	const vec4 alb = texture(albedo, uv);
	const vec4 pos_txt = texture(position, uv);
	const vec3 pos = pos_txt.xyz;
	const float roughness = pos_txt.w;
	const vec4 nrm_txt = texture(normal, uv);
	const vec3 nrm = nrm_txt.xyz;
	const float metallic = nrm_txt.w;
	const vec3 eye = normalize(scene_ubo.s.camera.position_far.xyz - pos);
	out_color.xyz = alb.xyz * 0.3; // todo it must come along scene
	out_color.xyz += calc_lights(alb.xyz, nrm, eye, roughness, metallic);
	out_color.xyz *= texture(ambient_occlusion, uv).x;
	out_color.xyz = pow(out_color.xyz / (out_color.xyz + vec3(1.0)), vec3(1.0 / 2.2)); // todo gamma must come along scene
	out_color.w = alb.w;
}