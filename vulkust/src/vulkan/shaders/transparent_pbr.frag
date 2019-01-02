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

layout (location = 0) out vec4 out_color;

vec3 calc_lights(const vec3 alb, const vec3 nrm, const vec3 eye, const vec3 pos, const float roughness, const float metallic) {
	const vec3 base_reflectivity = mix(vec3(0.02), alb, metallic);
	vec3 result = vec3(0.0);
	// Directional lights
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
		result += (kd * (alb / VX_PI) + specular) * radiance * smoothstep(0.005, 1.0, slope);
	}
	// Point lights
	for(uint light_index = 0; light_index < scene_ubo.s.lights_count.y; ++light_index) {
		const vec3 lm = scene_ubo.s.point_lights[light_index].position_radius.xyz - pos;
		float ll = length(lm);
		if(ll > scene_ubo.s.point_lights[light_index].position_radius.w) {
			continue;
		} else if(ll < scene_ubo.s.point_lights[light_index].color_minradius.w) {
			ll = scene_ubo.s.point_lights[light_index].color_minradius.w;
		}
		const float ill = 1.0 / ll;
		const vec3 l = lm * ill;
		const float slope = dot(nrm, l);
		if(slope < 0.005) {
			continue;
		}
		const vec3 halfway = normalize(eye + l);
		const float attenuation = ill * ill;
		const vec3 radiance = scene_ubo.s.point_lights[light_index].color_minradius.xyz * attenuation;
		const float distribution = NDFTRGGX(nrm, halfway, roughness);
		const float geometry = GFSCHGGX(nrm, eye, l, roughness);
		const vec3 fresnel = FFSCHGGX(clamp(dot(halfway, eye), 0.0, 1.0), base_reflectivity);
		const vec3 kd = (vec3(1.0) - fresnel) * (1.0 - metallic);
		const vec3 nom = distribution * geometry * fresnel;
		const float denom = 4 * max(dot(nrm, eye), 0.0) * max(dot(nrm, l), 0.0);
        const vec3 specular = nom / max(denom, 0.001);
		result += (kd * (alb / VX_PI) + specular) * radiance * smoothstep(0.005, 1.0, slope);
	}
	return result;
}

void main() {
    vec4 tmpv4 = texture(base_color, uv) * texture(base_color_factor, uv);
    tmpv4.w *= material_ubo.s.alpha;
    if(tmpv4.w < material_ubo.s.alpha_cutoff) {
        discard;
    }
    const vec4 alb = tmpv4;
    tmpv4.xy = texture(metallic_roughness, uv).xy * vec2(material_ubo.s.metallic_factor, material_ubo.s.roughness_factor);
	const float metallic = tmpv4.x;
	const float roughness = tmpv4.y;
    const vec3 mapped_nrm = normalize(mat3(tng, btg, nrm) * ((texture(normal, uv).xyz - 0.5) * 2.0));
	const vec3 eye = normalize(scene_ubo.s.camera.position_far.xyz - pos);
	out_color.xyz = alb.xyz * 0.3; // todo it must come along scene
	out_color.xyz += calc_lights(alb.xyz, mapped_nrm, eye, pos, roughness, metallic);
	out_color.xyz = pow(out_color.xyz / (out_color.xyz + vec3(1.0)), vec3(1.0 / 2.2)); // todo gamma must come along scene
	out_color.w = alb.w;
}