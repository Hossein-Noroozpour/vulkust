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

vec4 alb;
vec3 pos;
vec3 vpos;
vec4 spos;
vec4 tmpv;
vec3 nrm;
vec3 tng;
vec3 btg;
mat3 tbn;
vec3 eye;
float roughness;
float metallic;
vec3 base_reflectivity;

void calc_lights() {
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
		out_color.xyz += (kd * (alb.xyz / VX_PI) + specular) * radiance * smoothstep(0.005, 1.0, slope) * directional_lights_shadowness[light_index];
	}
}

float mod289(float x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 mod289(vec4 x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 perm(vec4 x){return mod289(((x * 34.0) + 1.0) * x);}

float noise(vec3 p){
    vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    vec4 k1 = perm(b.xyxy);
    vec4 k2 = perm(k1.xyxy + b.zzww);

    vec4 c = k2 + a.zzzz;
    vec4 k3 = perm(c);
    vec4 k4 = perm(c + 1.0);

    vec4 o1 = fract(k3 * (1.0 / 41.0));
    vec4 o2 = fract(k4 * (1.0 / 41.0));

    vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);

    return o4.y * d.y + o4.x * (1.0 - d.y);
}

float random() {
	return smoothstep(0.0, 1.0, abs(noise(pos * 500.0)));
}

bool is_zero(float v) {
	return v < SMALL_EPSILON && v > -SMALL_EPSILON;
}

bool is_equal(vec2 a, vec2 b) {
	a -= b;
	return is_zero(a.x) && is_zero(a.y);
}

bool is_equal(vec4 a, vec4 b) {
	a -= b;
	return is_zero(a.x) && is_zero(a.y) && is_zero(a.z) && is_zero(a.w);
}

bool find_hit(vec3 p, float dis, float steps, inout vec2 hituv) {
	p = (scene_ubo.s.camera.view * vec4(p, 1.0)).xyz;
	vec3 dir = normalize(p - vpos);
	if(is_zero(dir.z)) {
		return false;
	}
	vec3 tmpv3 = dir * dis + vpos;
	if(tmpv3.z < scene_ubo.s.camera.position_far.w) {
		dis = abs((scene_ubo.s.camera.position_far.w - vpos.z) / dir.z);
		if(dis < SMALL_EPSILON) {
			return false;
		}
		tmpv3 = dir * dis + vpos;
	} else if (tmpv3.z > scene_ubo.s.camera.near_aspect_ratio_reserved.x) {
		dis = abs((scene_ubo.s.camera.near_aspect_ratio_reserved.x - vpos.z) / dir.z);
		if(dis < SMALL_EPSILON) {
			return false;
		}
		tmpv3 = dir * dis + vpos;
	}
	const vec3 end = tmpv3;

	const mat4 proj = mat4(
		500, 0.0, 0.0, 0.0,
		0.0, -350, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		500, 350, 0.0, 1.0
		) * scene_ubo.s.camera.projection;

	const vec4 h0 = proj * vec4(vpos, 1.0);
	const vec4 h1 = proj * vec4(end, 1.0);

	const float k0 = 1.0 / h0.w;
	const float k1 = 1.0 / h1.w;

	const vec3 q0 = vpos * k0;
	const vec3 q1 = end * k1;

    const vec2 p0 = h0.xy * k0;
    const vec2 p1 = h1.xy * k1;

	vec4 tmpv4 = vec4(p1 - p0, q1.z - q0.z, k1 - k0);
	if(is_zero(tmpv4.x) && is_zero(tmpv4.y)) {
		return false;
	}
	float endx = 1000;
	bool permute = false;
	{
		vec2 ad = abs(tmpv4.xy);
		float coef;
		if(ad.x > ad.y) {
			coef = 1.0 / ad.x;
		} else {
			endx = 700;
			permute = true;
			tmpv4.xy = tmpv4.yx;
			coef = 1.0 / ad.y;
		}
		tmpv4 *= coef;
	}
	const vec4 dpqk = tmpv4;
	vec4 pqk = vec4(permute? p0.yx: p0, q0.z, k0);
	pqk += dpqk * 0.2;
	// int debug1 = 0;
	// {
	// 	float maxx = 1.0, minx = 0.0;
	// 	if(dpqk.x < 0.0) {
	// 		if(p1.x > minx) {
	// 			minx = p1.x;
	// 		}
	// 	} else {
	// 		if(p1.x < maxx) {
	// 			maxx = p1.x;
	// 		}
	// 	}
	// 	float endx = pqk.x + (dpqk.x * steps);
	// 	if (endx < minx) {
	// 		debug1 = 1;
	// 		steps = abs((pqk.x - minx) / dpqk.x);
	// 	} else if (endx > maxx) {
	// 		debug1 = 2;
	// 		steps = abs((maxx - pqk.x) / dpqk.x);
	// 	}
	// }

	float prev_z_max_estimate = -abs(pqk.z / pqk.w);
    float ray_z_max = prev_z_max_estimate, ray_z_min = prev_z_max_estimate;
    float scene_z_max = ray_z_max + 10000;
	const float z_thickness = 0.1;

	// if(steps < 10.0) {
	// 	out_color.xyz *= 0.0;
	// 	if(debug1 == 1) out_color.y = 1.0;
	// 	if(debug1 == 2) out_color.z = 1.0;
	// 	return false;
	// }

	float step_index = 0;
	for(;
		(step_index < steps) &&
			(pqk.x < endx && pqk.x > 0.0) && // debug
			!is_zero(scene_z_max) &&
			((ray_z_max < (scene_z_max - z_thickness)) || (ray_z_min > scene_z_max));
		++step_index, pqk += dpqk
	) {
        ray_z_min = prev_z_max_estimate;
        ray_z_max = -abs((dpqk.z * 0.5 + pqk.z) / (dpqk.w * 0.5 + pqk.w));
		prev_z_max_estimate = ray_z_max;
        if (ray_z_min > ray_z_max) {
			float tmp = ray_z_min;
			ray_z_min = ray_z_max;
			ray_z_max = tmp;
		}
		// todo it can be replaced with something much better
		hituv = (permute? pqk.yx: pqk.xy) *  deferred_ubo.s.pixel_step.xy;
		hituv.y = 1 - hituv.y;
		if(hituv.x < 0.0 || hituv.x > 1.0 || hituv.y < 0.0 || hituv.y > 1.0) {
			return false;
		}
		scene_z_max = -abs((scene_ubo.s.camera.view * vec4(texture(position, hituv).xyz, 1.0)).z);

	}
	// if (step_index < 2 && (ray_z_max >= scene_z_max - z_thickness) && (ray_z_min <= scene_z_max)) {
	// 	out_color.xyz *= 0.0;
	// 	if(is_equal(uv, permute? pqk.yx: pqk.xy))
	// 		out_color.y = 1.0;
	// 	else
	// 		out_color.x = 1.0;
	// 	return false;
	// }
	// if((step_index < steps) &&
	// 		(pqk.x < endx && pqk.x > 0.0) && // debug
	// 		!is_zero(scene_z_max)) return false;
	return (ray_z_max >= scene_z_max - z_thickness) && (scene_z_max >= ray_z_min);
}

// float calc_ssao() {
// 	float ambient_occlusion = 1.0;
// 	for(int ssao_sample_index = 0; ssao_sample_index < SSAO_SAMPLES; ++ssao_sample_index) {
// 		vec3 hit;
// 		vec2 hituv;
// 		if(find_hit(vec4(pos + vec3(vec2(random(), random()) * 2.0 - 1.0, random()), 1.0), SSAO_SEARCH_STEPS, hituv)) {
// 			vec3 dir = hit - pos;
// 			float ld = length(dir);
// 			float hbao = abs(dir.z / ld) - abs(tng.z); 
// 			ambient_occlusion += hbao * 3.0 / float(SSAO_SAMPLES);
// 		}
// 	}
// 	return ambient_occlusion;
// }

void calc_ssr() {
	vec3 ray = reflect(-eye, nrm) * 10.0;
	vec2 hituv = vec2(0.0);
	if(find_hit(pos + ray, 10.0, 500, hituv)) {
		// if ( 0.0 > dot(texture(normal, hituv).xyz, ray)) {
			out_color.xyz *= 0.5;
			out_color.xyz += 0.3 * texture(albedo, hituv).xyz;
		// }
	}
	// {
	// 	vec4 p = scene_ubo.s.camera.view_projection * vec4(pos, 1.0);
	// 	if (p.z / p.w > 0.9) {
	// 		out_color.xyz *= 0.0;
	// 		out_color.x = 1.0;
	// 	}
	// }
}

void main() {
	alb = texture(albedo, uv);
	tmpv = texture(position, uv);
	pos = tmpv.xyz;
	roughness = tmpv.w;
	tmpv = texture(normal, uv);
	nrm = tmpv.xyz;
	metallic = tmpv.w;
	if (nrm.x < 1.0 + NORMAL_EPSILON && nrm.x > 1.0 - NORMAL_EPSILON) {
		tng = vec3(0.0, 1.0, 0.0);
		btg = vec3(1.0, 0.0, 0.0);
	} else {
		tng = normalize(cross(nrm, vec3(1.0, 0.0, 0.0)));
		btg = cross(nrm, tng);
	}
	tbn = mat3(tng, btg, nrm);
	eye = normalize(scene_ubo.s.camera.position_far.xyz - pos);
	spos = scene_ubo.s.camera.view_projection * vec4(pos, 1.0);
	spos.xyz /= spos.w;
	vpos = (scene_ubo.s.camera.view * vec4(pos, 1.0)).xyz;
	base_reflectivity = mix(vec3(0.02), alb.xyz, metallic);
	out_color.xyz = alb.xyz * 0.3; // todo it must come along scene
	// out_color.xyz *= 0.001; // todo it must come along scene
	calc_lights();
	out_color.xyz *= texture(ambient_occlusion, uv).x;
	// calc_ssr();

	// out_color.xyz *= dot(-normalize(reflect(eye, nrm)), nrm);
	// out_color.x = spos.z;
	out_color.xyz = pow(out_color.xyz / (out_color.xyz + vec3(1.0)), vec3(1.0 / 2.2));
	out_color.w = 1.0;
	// todo lots of work must be done in here
}