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
layout (set = 1, binding = 5) uniform usampler2D shadow_directional_flagbits;

vec4 alb;
vec3 pos;
vec3 vpos;
vec4 spos;
vec4 tmpv;
vec3 nrm;
vec3 tng;
vec3 btg;
mat3 tbn;
vec3 eye_nrm;

void calc_lights() {
	vec2 start_uv = uv - (vec2(deferred_ubo.s.pixel_step.x, deferred_ubo.s.pixel_step.y) * float((BLUR_KERNEL_LENGTH - 1) >> 1));
	for(uint light_index = 0; light_index < scene_ubo.s.directional_point_lights_count.x; ++light_index) {
		float slope = -dot(nrm, scene_ubo.s.directional_lights[light_index].direction_strength.xyz);
		if(slope < 0.005) {
			continue;
		}
		slope = smoothstep(slope, 0.005, 0.2);
		float brightness = 1.0;
		vec2 shduv = start_uv;
		uint light_flag = 1 << light_index;
		for(uint si = 0; si < BLUR_KERNEL_LENGTH; ++si, shduv.y = start_uv.y, shduv.x += deferred_ubo.s.pixel_step.x) {
			for (uint sj = 0; sj < BLUR_KERNEL_LENGTH; ++sj, shduv.y += deferred_ubo.s.pixel_step.y) {
				if ((texture(shadow_directional_flagbits, shduv).x & light_flag) == light_flag) {
					brightness -= 1.0 / float(BLUR_KERNEL_LENGTH * BLUR_KERNEL_LENGTH);
				}
			}
		}
		brightness *= slope;
		out_color.xyz += alb.xyz * scene_ubo.s.directional_lights[light_index].direction_strength.w * brightness; 
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

bool find_hit(vec3 p, float dis, float steps, inout vec2 hituv) {
	p = (scene_ubo.s.camera.view * vec4(p, 1.0)).xyz;
	vec3 dir = normalize(p - vpos);
	if(is_zero(dir.z)) {
		return false;
	}
	vec3 tmpv3 = dir * dis + vpos;
	if(tmpv3.z < -100/*scene_ubo.s.camera.position_far.w*/) {
		dis = abs((-100/*scene_ubo.s.camera.position_far.w*/ - vpos.z) / dir.z);
		if(dis < SMALL_EPSILON) {
			return false;
		}
		tmpv3 = dir * dis + vpos;
	} else if (tmpv3.z > -1/*scene_ubo.s.camera.near_reserved.x*/) {
		dis = abs((-1/*scene_ubo.s.camera.near_reserved.x*/ - vpos.z) / dir.z);
		if(dis < SMALL_EPSILON) {
			return false;
		}
		tmpv3 = dir * dis + vpos;
	}
	const vec3 end = tmpv3;

	const mat4 proj = mat4(
		0.5, 0.0, 0.0, 0.0,
		0.0, 0.5, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.5, 0.5, 0.0, 1.0
		) * scene_ubo.s.camera.projection;

	const vec4 h0 = proj * vec4(vpos, 1.0);
	const vec4 h1 = proj * vec4(end, 1.0);

	const float k0 = 1.0 / h0.w;
	const float k1 = 1.0 / h1.w;

	const vec3 q0 = vpos * k0;
	const vec3 q1 = end * k1; 

    const vec2 p0 = h0.xy * k0; // uv; // (h0.xy * (k0 * 0.5)) + 0.5;
    const vec2 p1 = h1.xy * k1; // ((h1.xy * 0.5) + 0.5) * k1;

	vec4 dpqk = vec4(p1 - p0, q1.z - q0.z, k1 - k0);
	if(is_zero(dpqk.x) && is_zero(dpqk.y)) {
		return false;
	}
	bool permute = false;
	{
		vec2 ad = abs(dpqk.xy);
		float coef;
		if(ad.x > ad.y) {
			coef = deferred_ubo.s.pixel_step.x / ad.x;
		} else {
			permute = true;
			dpqk.xy = dpqk.yx;
			coef = deferred_ubo.s.pixel_step.y / ad.y;
		}
		if ( is_zero(dpqk.x)) {
			return false;
		}
		dpqk *= coef;
	}
	vec4 pqk = vec4(permute? p0.yx: p0, q0.z, k0);
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

	float prev_z_max_estimate = abs(vpos.z);
    float ray_z_max = prev_z_max_estimate, ray_z_min = prev_z_max_estimate;
    float scene_z_max = ray_z_max + 10000;
	const float z_thickness = 1.0;

	// if(steps < 10.0) {
	// 	out_color.xyz *= 0.0;
	// 	if(debug1 == 1) out_color.y = 1.0;
	// 	if(debug1 == 2) out_color.z = 1.0;
	// 	return false;
	// }

	pqk += dpqk * 0.2;
	float step_index = 0;
	for(;
		(step_index < steps) &&
			(pqk.x < 1.0 && pqk.x > 0.0) && // debug
			((ray_z_max < (scene_z_max - z_thickness)) || (ray_z_min > scene_z_max));
		++step_index, pqk += dpqk
	) {
        ray_z_min = prev_z_max_estimate;
        ray_z_max = abs((dpqk.z * 0.5 + pqk.z) / (dpqk.w * 0.5 + pqk.w));
		prev_z_max_estimate = ray_z_max;
        if (ray_z_min > ray_z_max) {
			float tmp = ray_z_min;
			ray_z_min = ray_z_max;
			ray_z_max = tmp;
		}
		// todo it can be replaced with something much better
		scene_z_max = abs((scene_ubo.s.camera.view * vec4(texture(position, permute? pqk.yx: pqk.xy).xyz, 1.0)).z);
	}
	hituv = permute? pqk.yx: pqk.xy;
	if (step_index < 10 && (ray_z_max >= scene_z_max - z_thickness) && (ray_z_min <= scene_z_max)) {
		out_color.xyz *= 0.0;
		if(is_equal(uv, permute? pqk.yx: pqk.xy))
			out_color.y = 1.0;
		else
			out_color.x = 1.0;
		return false;
	}
	return (ray_z_max >= scene_z_max - z_thickness) && (ray_z_min <= scene_z_max);
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
	vec3 ray = reflect(-eye_nrm, nrm) * 10.0;
	vec2 hituv = vec2(0.0);
	if(find_hit(pos + ray, 100.0, 500, hituv)) {
		// if ( 0.0 > dot(texture(normal, hituv).xyz, ray)) {
			out_color.xyz *= 0.5;
			out_color.xyz += 0.5 * texture(albedo, hituv).xyz;
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
	pos = texture(position, uv).xyz;
	nrm = texture(normal, uv).xyz;
	if (nrm.x < 1.0 + NORMAL_EPSILON && nrm.x > 1.0 - NORMAL_EPSILON) {
		tng = vec3(0.0, 1.0, 0.0);
		btg = vec3(1.0, 0.0, 0.0);
	} else {
		tng = normalize(cross(nrm, vec3(1.0, 0.0, 0.0)));
		btg = cross(nrm, tng);
	}
	tbn = mat3(tng, btg, nrm);
	eye_nrm = normalize(scene_ubo.s.camera.position_far.xyz - pos);
	spos = scene_ubo.s.camera.view_projection * vec4(pos, 1.0);
	spos.xyz /= spos.w;
	vpos = (scene_ubo.s.camera.view * vec4(pos, 1.0)).xyz;

	out_color.xyz = alb.xyz; // todo it must come along scene
	// calc_lights();
	calc_ssr();

	// out_color.xyz *= dot(-normalize(reflect(eye_nrm, nrm)), nrm);
	// out_color.x = spos.z;
	out_color.w = 1.0;
	// todo lots of work must be done in here
}