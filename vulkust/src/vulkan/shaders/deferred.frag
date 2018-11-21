#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8
#define BLUR_KERNEL_LENGTH 5
#define SSAO_SAMPLES 32
#define SSAO_SEARCH_STEPS 4
#define NORMAL_EPSILON 0.005
#define SMALL_EPSILON 0.00001

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
vec3 spos;
float sposw;
vec4 tmpv;
vec3 nrm;
vec3 tng;
vec3 btg;
mat3 tbn;
vec3 eye_nrm;

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

bool is_zero(float v) {
	return v < SMALL_EPSILON && v > -SMALL_EPSILON;
}

bool find_hit(vec4 p, uint steps, inout vec2 hituv) {
	p = scene_ubo.camera.view_projection * vec4(p.xyz, 1.0);
	// p.y = -p.y;
	p.xy /= p.w;
	p.w = 1.0 / p.w;
	float sposzw = spos.z * sposw;
	// vec4 dir = p - vec4(spos.x, -spos.y, sposzw, sposw);
	vec4 dir = p - vec4(spos.xy, sposzw, 1.0 / sposw);
	if(is_zero(dir.x) && is_zero(dir.y)) {
		return false;
	} else {
		vec2 ad = abs(dir.xy);
		float coef;
		if(ad.x > ad.y) {
			coef = deferred_ubo.pixel_x_step / ad.x;
		} else {
			coef = deferred_ubo.pixel_y_step / ad.y;
		}
		dir *= coef;
	}
	p.xy = uv;
	p.z = sposzw;
	p.w = 1.0 / sposw;
	float prez = spos.z;
	for(int step_index = 0; step_index < steps; ++step_index) {
		p += dir;
		if(p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0) {
			return false;
		}
		float curz = p.z * p.w;
		float samz = texture(screen_space_depth, p.xy).x;
		if(((curz + 0.001 >= samz && samz >= prez && curz >= prez) || (curz - 0.001 <= samz && samz <= prez && curz <= prez)) && samz < 0.999 && samz > 0.001) {
			hituv = p.xy;
			return true;
		} 
		// else if(prez > samz) {
		// 	return false;
		// }
		prez = curz;
	}
	return false;
}

float calc_ssao() {
	float ambient_occlusion = 1.0;
	for(int ssao_sample_index = 0; ssao_sample_index < SSAO_SAMPLES; ++ssao_sample_index) {
		vec3 hit;
		vec2 hituv;
		if(find_hit(vec4(pos + vec3(vec2(random(), random()) * 2.0 - 1.0, random()), 1.0), SSAO_SEARCH_STEPS, hituv)) {
			vec3 dir = hit - pos;
			float ld = length(dir);
			float hbao = abs(dir.z / ld) - abs(tng.z); 
			ambient_occlusion += hbao * 3.0 / float(SSAO_SAMPLES);
		}
	}
	return ambient_occlusion;
}

void calc_ssr() {
	vec3 ray = reflect(-eye_nrm, nrm);
	vec2 hituv = vec2(0.0);
	if(find_hit(vec4(pos + ray * 100.0, 1.0), 1000, hituv)) {
		// if ( 0.0 > dot(texture(normal, hituv).xyz, ray)) {
			out_color.xyz *= 0.5;
			// out_color.xyz += 0.3 * texture(albedo, hituv).xyz;
		// }
	}
	// {
	// 	vec4 p = scene_ubo.camera.view_projection * vec4(pos, 1.0);
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
	eye_nrm = normalize(scene_ubo.camera.position_radius.xyz - pos);
	tmpv = scene_ubo.camera.view_projection * vec4(pos, 1.0);
	spos = tmpv.xyz / tmpv.w;
	sposw = tmpv.w;

	out_color.xyz = alb.xyz; // todo it must come along scene
	// calc_lights();
	calc_ssr();

	// out_color.xyz *= dot(-normalize(reflect(eye_nrm, nrm)), nrm);
	// out_color.x = spos.z;
	out_color.w = alb.w;
	// todo lots of work must be done in here
}

























// #define point2 vec2
// #define point3 vec3

// float distanceSquared(vec2 a, vec2 b) { a -= b; return dot(a, a); }

// // Returns true if the ray hit something
// bool traceScreenSpaceRay1(
//  // Camera-space ray origin, which must be within the view volume
//  point3 csOrig, 

//  // Unit length camera-space ray direction
//  vec3 csDir,

//  // A projection matrix that maps to pixel coordinates (not [-1, +1]
//  // normalized device coordinates)
//  mat4x4 proj, 

//  // The camera-space Z buffer (all negative values)
//  sampler2D csZBuffer,

//  // Dimensions of csZBuffer
//  vec2 csZBufferSize,

//  // Camera space thickness to ascribe to each pixel in the depth buffer
//  float zThickness, 

//  // (Negative number)
//  float nearPlaneZ, 

//  // Step in horizontal or vertical pixels between samples. This is a float
//  // because integer math is slow on GPUs, but should be set to an integer >= 1
//  float stride,

//  // Number between 0 and 1 for how far to bump the ray in stride units
//  // to conceal banding artifacts
//  float jitter,

//  // Maximum number of iterations. Higher gives better images but may be slow
//  const float maxSteps, 

//  // Maximum camera-space distance to trace before returning a miss
//  float maxDistance, 

//  // Pixel coordinates of the first intersection with the scene
//  out point2 hitPixel, 

//  // Camera space location of the ray hit
//  out point3 hitPoint) {

//     // Clip to the near plane    
//     float rayLength = ((csOrig.z + csDir.z * maxDistance) > nearPlaneZ) ?
//         (nearPlaneZ - csOrig.z) / csDir.z : maxDistance;
//     point3 csEndPoint = csOrig + csDir * rayLength;

//     // Project into homogeneous clip space
//     vec4 H0 = proj * vec4(csOrig, 1.0);
//     vec4 H1 = proj * vec4(csEndPoint, 1.0);
//     float k0 = 1.0 / H0.w, k1 = 1.0 / H1.w;

//     // The interpolated homogeneous version of the camera-space points  
//     point3 Q0 = csOrig * k0, Q1 = csEndPoint * k1;

//     // Screen-space endpoints
//     point2 P0 = H0.xy * k0, P1 = H1.xy * k1;

//     // If the line is degenerate, make it cover at least one pixel
//     // to avoid handling zero-pixel extent as a special case later
//     P1 += vec2((distanceSquared(P0, P1) < 0.0001) ? 0.01 : 0.0);
//     vec2 delta = P1 - P0;

//     // Permute so that the primary iteration is in x to collapse
//     // all quadrant-specific DDA cases later
//     bool permute = false;
//     if (abs(delta.x) < abs(delta.y)) { 
//         // This is a more-vertical line
//         permute = true; delta = delta.yx; P0 = P0.yx; P1 = P1.yx; 
//     }

//     float stepDir = sign(delta.x);
//     float invdx = stepDir / delta.x;

//     // Track the derivatives of Q and k
//     vec3  dQ = (Q1 - Q0) * invdx;
//     float dk = (k1 - k0) * invdx;
//     vec2  dP = vec2(stepDir, delta.y * invdx);

//     // Scale derivatives by the desired pixel stride and then
//     // offset the starting values by the jitter fraction
//     dP *= stride; dQ *= stride; dk *= stride;
//     P0 += dP * jitter; Q0 += dQ * jitter; k0 += dk * jitter;

//     // Slide P from P0 to P1, (now-homogeneous) Q from Q0 to Q1, k from k0 to k1
//     point3 Q = Q0; 

//     // Adjust end condition for iteration direction
//     float  end = P1.x * stepDir;

//     float k = k0, stepCount = 0.0, prevZMaxEstimate = csOrig.z;
//     float rayZMin = prevZMaxEstimate, rayZMax = prevZMaxEstimate;
//     float sceneZMax = rayZMax + 100;
//     for (point2 P = P0; 
//          ((P.x * stepDir) <= end) && (stepCount < maxSteps) &&
//          ((rayZMax < sceneZMax - zThickness) || (rayZMin > sceneZMax)) &&
//           (sceneZMax != 0); 
//          P += dP, Q.z += dQ.z, k += dk, ++stepCount) {
        
//         rayZMin = prevZMaxEstimate;
//         rayZMax = (dQ.z * 0.5 + Q.z) / (dk * 0.5 + k);
//         prevZMaxEstimate = rayZMax;
//         if (rayZMin > rayZMax) { 
//            float t = rayZMin; rayZMin = rayZMax; rayZMax = t;
//         }

//         hitPixel = permute ? P.yx : P;
//         // You may need hitPixel.y = csZBufferSize.y - hitPixel.y; here if your vertical axis
//         // is different than ours in screen space
//         sceneZMax = texelFetch(csZBuffer, int2(hitPixel), 0);
//     }
    
//     // Advance Q based on the number of steps
//     Q.xy += dQ.xy * stepCount;
//     hitPoint = Q * (1.0 / k);
//     return (rayZMax >= sceneZMax - zThickness) && (rayZMin < sceneZMax);
// }