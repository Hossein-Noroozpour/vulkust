#define MAX_DIRECTIONAL_CASCADES_COUNT 6
#define MAX_DIRECTIONAL_LIGHTS_COUNT 8
#define MAX_POINT_LIGHTS_COUNT 32
#define MAX_SSAO_SAMPLES_COUNT 128
#define BLUR_KERNEL_LENGTH 5
#define SSAO_SAMPLES 32
#define SSAO_SEARCH_STEPS 4
#define NORMAL_EPSILON 0.005
#define SMALL_EPSILON 0.00001

struct Camera {
	vec4 position_far;
	vec4 near_reserved;
	mat4 projection;
	mat4 view;
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