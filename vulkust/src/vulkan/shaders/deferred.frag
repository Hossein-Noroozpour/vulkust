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
vec4 vpos;
vec4 spos;
vec4 tmpv;
vec3 nrm;
vec3 tng;
vec3 btg;
mat3 tbn;
vec3 eye_nrm;

void calc_lights() {
	vec2 start_uv = uv - (vec2(deferred_ubo.s.pixel_x_step, deferred_ubo.s.pixel_y_step) * float((BLUR_KERNEL_LENGTH - 1) >> 1));
	for(uint light_index = 0; light_index < scene_ubo.s.directional_point_lights_count.x; ++light_index) {
		float slope = -dot(nrm, scene_ubo.s.directional_lights[light_index].direction_strength.xyz);
		if(slope < 0.005) {
			continue;
		}
		slope = smoothstep(slope, 0.005, 0.2);
		float brightness = 1.0;
		vec2 shduv = start_uv;
		uint light_flag = 1 << light_index;
		for(uint si = 0; si < BLUR_KERNEL_LENGTH; ++si, shduv.y = start_uv.y, shduv.x += deferred_ubo.s.pixel_x_step) {
			for (uint sj = 0; sj < BLUR_KERNEL_LENGTH; ++sj, shduv.y += deferred_ubo.s.pixel_y_step) {
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

bool find_hit(vec4 p, float dis, float steps, inout vec2 hituv) {
	p = scene_ubo.s.camera.view * vec4(p.xyz, 1.0);
	vec4 dir = p - vpos;
	{
		float l = length(dir);
		if(is_zero(l)) {
			return false;
		}
		dir.xyz /= l;
	}
	if(is_zero(dir.z)) {
		return false;
	}
	p.xyz *= dis;
	p.w = p.z + vpos.z;
	if(p.w < scene_ubo.s.camera.position_far.w) {
		dis = (scene_ubo.s.camera.position_far.w - vpos.z) / p.z;
	} else if (p.w > scene_ubo.s.camera.near_reserved.x) {
		dis = (scene_ubo.s.camera.near_reserved.x - vpos.z) / p.z;
	}
	if(is_zero(dis)) {
		return false;
	}
	p = vpos + (dir * dis);
	dir.z = p.z;
	p.w = 1;
	p = scene_ubo.s.camera.projection * p;
	float t1 = 1.0 / p.w;
	float t2 = 1.0 / spos.w;
	dir.w = t1 - t2;
	dir.xy = (p.xy * t1 * 0.5 + 0.5) - uv;
	if(is_zero(dir.x) && is_zero(dir.y)) {
		return false;
	} else {
		vec2 ad = abs(dir.xy);
		float coef;
		if(ad.x > ad.y) {
			coef = deferred_ubo.s.pixel_x_step / ad.x;
		} else {
			coef = deferred_ubo.s.pixel_y_step / ad.y;
		}
		dir *= coef;
	}
	p.xy = uv;
	p.z = vpos.z;
	p.w = t2;

	p += dir;

	float prez = p.z * p.w;
	for(float step_index = 0; step_index < steps; ++step_index) {
		p += dir;
		if(p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0) {
			return false;
		}
		float curz = p.z * p.w;
		float samz = texture(screen_space_depth, p.xy).x;
		if(curz < prez) {
			t1 = prez;
			t2 = curz;
		} else {
			t2 = prez;
			t1 = curz;
		}
		prez = curz;
		if(t1 + 0.001 > samz && samz > t2) {
			hituv = p.xy;
			return true;
		}
	}
	return false;
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
	vec3 ray = reflect(-eye_nrm, nrm);
	vec2 hituv = vec2(0.0);
	if(find_hit(vec4(pos + ray * 100.0, 1.0), 100.0, 1000, hituv)) {
		// if ( 0.0 > dot(texture(normal, hituv).xyz, ray)) {
			out_color.xyz *= 0.5;
			// out_color.xyz += 0.3 * texture(albedo, hituv).xyz;
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
	vpos = scene_ubo.s.camera.view * vec4(pos, 1.0);

	out_color.xyz = alb.xyz; // todo it must come along scene
	// calc_lights();
	calc_ssr();

	// out_color.xyz *= dot(-normalize(reflect(eye_nrm, nrm)), nrm);
	// out_color.x = spos.z;
	out_color.w = 1.0;
	// todo lots of work must be done in here
}