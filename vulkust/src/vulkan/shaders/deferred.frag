#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8
#define BLUR_KERNEL_LENGTH 5
#define SSAO_SAMPLES 9
#define NORMAL_EPSILON 0.005

layout (location = 0) in vec2 uv;

layout (location = 0) out vec4 out_color;

struct Camera {
	vec4 position_radius;
	mat4 projection;
	mat4 view;
	mat4 view_projection;
};

struct PointLight {
	vec4 color;
	vec4 position_radius;
};

struct DirectionalLight {
	vec4 color;
	vec4 direction_strength;
};

layout (set = 0, binding = 0) uniform SceneUBO {
	Camera camera;
	DirectionalLight directional_lights[MAX_DIRECTIONAL_LIGHTS_COUNT];
	PointLight point_lights[MAX_POINT_LIGHTS_COUNT];
	uvec4 directional_point_lights_count;
} scene_ubo;

layout (set = 1, binding = 0) uniform DeferredUBO {
	float pixel_x_step;
	float pixel_y_step;
} deferred_ubo;

layout (set = 1, binding = 1) uniform sampler2D position;
layout (set = 1, binding = 2) uniform sampler2D normal;
layout (set = 1, binding = 3) uniform sampler2D albedo;
layout (set = 1, binding = 4) uniform sampler2D screen_space_depth;
layout (set = 1, binding = 5) uniform usampler2D shadow_directional_flagbits;

vec4 alb;
vec3 pos;
vec3 nrm;
vec3 tng;
vec3 btg;
mat3 tbn;

void calc_lights() {
	vec2 start_uv = uv - (vec2(deferred_ubo.pixel_x_step, deferred_ubo.pixel_y_step) * float((BLUR_KERNEL_LENGTH - 1) >> 1));
	for(uint light_index = 0; light_index < scene_ubo.directional_point_lights_count.x; ++light_index) {
		float slope = -dot(nrm, scene_ubo.directional_lights[light_index].direction_strength.xyz);
		if(slope < 0.005) {
			continue;
		}
		slope = smoothstep(slope, 0.005, 0.2);
		float brightness = 1.0;
		vec2 shduv = start_uv;
		uint light_flag = 1 << light_index;
		for(uint si = 0; si < BLUR_KERNEL_LENGTH; ++si, shduv.y = start_uv.y, shduv.x += deferred_ubo.pixel_x_step) {
			for (uint sj = 0; sj < BLUR_KERNEL_LENGTH; ++sj, shduv.y += deferred_ubo.pixel_y_step) {
				if ((texture(shadow_directional_flagbits, shduv).x & light_flag) == light_flag) {
					brightness -= 1.0 / float(BLUR_KERNEL_LENGTH * BLUR_KERNEL_LENGTH);
				}
			}
		}
		brightness *= slope;
		out_color.xyz += alb.xyz * scene_ubo.directional_lights[light_index].direction_strength.w * brightness; 
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

void find_hit(vec3 p, vec3 dir, uint steps, inout vec3 hit, inout vec2 hituv) {

}

float calc_ssao() {

	vec2 ssaouv = uv - (vec2(deferred_ubo.pixel_x_step, deferred_ubo.pixel_y_step) * float(SSAO_SAMPLES));
	for(int i = 0; i < SSAO_SAMPLES; ++i, ssaouv.x += deferred_ubo.pixel_x_step) {
		vec3 p = texture(position, ssaouv).xyz;
	}
	return random();
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

	out_color.xyz = alb.xyz * calc_ssao() * length(scene_ubo.camera.view_projection * pos.xyzz); // todo it must come along scene
	// calc_lights();
	out_color.w = alb.w;
	// todo lots of work must be done in here
}