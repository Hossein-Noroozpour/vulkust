#define MAX_DIRECTIONAL_CASCADES_COUNT 6
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8
#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_SSAO_SAMPLES_COUNT 128
#define BLUR_KERNEL_LENGTH 5
#define SSAO_SAMPLES 32
#define SSAO_SEARCH_STEPS 4
#define NORMAL_EPSILON 0.005
#define SMALL_EPSILON 0.00001
#define VX_PI 3.1415926535897932384626433832795028841971693993751058209749445923078164062862

struct Camera {
    vec4 x;
    vec4 y;
    vec4 z;
	vec4 position_far; // far is negative
	vec4 near_aspect_ratio_reserved; // far is negative
	mat4 inversed_rotation;
	mat4 view;
	mat4 projection;
	mat4 uniform_projection; // x -> (0, 1), y -> (0, 1), z -> (0, 1)
	mat4 view_projection;
	mat4 uniform_view_projection; // x -> (0, 1), y -> (0, 1), z -> (0, 1)
};

struct PointLight {
	vec4 color;
	vec4 position_radius;
};

struct DirectionalLight {
	vec4 color;
	vec4 direction_strength;
};

struct Scene {
	Camera camera;
	DirectionalLight directional_lights[MAX_DIRECTIONAL_LIGHTS_COUNT];
	PointLight point_lights[MAX_POINT_LIGHTS_COUNT];
	uvec4 lights_count; // directional, point
	vec4 ssao_config; // samples-count, radius, z-tolerance, rezerved
};

struct SSAO {
	vec4 sample_vectors[MAX_SSAO_SAMPLES_COUNT];
};

struct Deferred {
	vec4 pixel_step;
};

struct Model {
    mat4 model;
};

struct Material {
    float alpha;
    float alpha_cutoff;
    float metallic_factor;
    float normal_scale;
    float occlusion_strength;
    float roughness_factor;
};

struct Light {
	mat4 view_projection_biases[MAX_DIRECTIONAL_CASCADES_COUNT];
    vec4 direction_strength;
    uint cascades_count;
    uint light_index;
};

struct ModelShadow {
	mat4 model_view_projection;
};

float gausssian_blur_5x5(const sampler2D s, const vec2 uv, const vec2 pixel_step) {
	const float ws[] = {
		1.0 / 256.0,  4.0 / 256.0,  6.0 / 256.0,  4.0 / 256.0, 1.0 / 256.0,
		4.0 / 256.0, 16.0 / 256.0, 24.0 / 256.0, 16.0 / 256.0, 4.0 / 256.0,
		6.0 / 256.0, 24.0 / 256.0, 36.0 / 256.0, 24.0 / 256.0, 6.0 / 256.0,
		4.0 / 256.0, 16.0 / 256.0, 24.0 / 256.0, 16.0 / 256.0, 4.0 / 256.0,
		1.0 / 256.0,  4.0 / 256.0,  6.0 / 256.0,  4.0 / 256.0, 1.0 / 256.0
	};
	float result = 0.0;
	vec2 suv = uv - (pixel_step * 2.0);
	const float y = suv.y;
	for(int c = 0, i = 0; c < 5; ++c, suv.x += pixel_step.x, suv.y = y) {
		for(int r = 0; r < 5; ++r, ++i, suv.y += pixel_step.y) {
			result += texture(s, suv).x * ws[i];
		}
	}
	return result;
}

// Normal Distribution Function Trowbridge-Reitz GGX
float NDFTRGGX(const vec3 normal, const vec3 halfway, const float roughness) {
    const float roughness2 = roughness * roughness;
    const float nh = max(dot(normal, halfway), 0.0);
    const float nh2 = nh * nh;
    const float nom = roughness2;
    const float tmpdenom = (nh2 * (roughness2 - 1.0) + 1.0);
    const float denom = VX_PI * tmpdenom * tmpdenom;
    return nom / denom;
}

float GFSCHGGX(const float nd, const float roughness) {
    const float r = roughness + 1.0;
    const float k = (r * r) * (1.0 / 8.0);
    const float nom = nd;
    const float denom = (nd * (1.0 - k)) + k;
    return nom / denom;
}

float GFSCHGGX(const vec3 normal, const vec3 view, const vec3 light, const float roughness) {
    const float nv = max(dot(normal, view), 0.0);
    const float nl = max(dot(normal, light), 0.0);
    const float ggx2 = GFSCHGGX(nv, roughness);
    const float ggx1 = GFSCHGGX(nl, roughness);
    return ggx1 * ggx2;
}

vec3 FFSCHGGX(const float nv, const vec3 f0) {
	const float inv = 1.0 - nv;
	const float inv2 = inv * inv;
	const float inv4 = inv2 * inv2;
	const float inv5 = inv4 * inv;
    return f0 + ((1.0 - f0) * inv5);
}