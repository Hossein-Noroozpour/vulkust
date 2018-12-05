#include "common.h.hlsl"

cbuffer SceneUBO: register(b0) { Scene scene_ubo; };
cbuffer ModelUBO: register(b1) { Model model_ubo; };
cbuffer MaterialUBO: register(b2) { Material material_ubo; };

struct PSInput {
    float4 spos: SV_POSITION;
    float3 pos: POSITION;
    float3 nrm: NORMAL;
    float3 tng: TANGENT;
    float3 btg: BINORMAL;
    float2 uv: TEXCOORD;
};

PSInput vert_main(float3 pos: POSITION, float3 nrm: NORMAL, float4 tng: TANGENT, float2 uv: TEXCOORD) {
    PSInput result;
	result.pos = mul(model_ubo.model, float4(pos, 1.0)).xyz;
    result.spos = mul(scene_ubo.camera.view_projection, float4(result.pos, 1.0));
	float3x3 m3_model = (float3x3) model_ubo.model;
	result.nrm = normalize(mul(m3_model, nrm));
	result.tng = normalize(mul(m3_model, tng.xyz));
	if (tng.w < 0.0) {
		result.btg = cross(result.tng, result.nrm);
	} else {
		result.btg = cross(result.nrm, result.tng);
	}
	result.uv = uv;
    return result;
}

Texture2D base_color: register(t);
Texture2D base_color_factor: register(t1);
Texture2D metallic_roughness: register(t2);
Texture2D normal: register(t3);
Texture2D occlusion: register(t4);
Texture2D emissive: register(t5);
Texture2D emissive_factor: register(t6);

SamplerState sam : register(s0);

struct PSOutput {
    float4 pos: SV_TARGET0;
    float4 nrm: SV_TARGET1;
    float4 alb: SV_TARGET2;
};

PSOutput frag_main(PSInput input) {
    PSOutput result;
    result.alb = mul(base_color.Sample(sam, input.uv), base_color_factor.Sample(sam, input.uv));
    result.alb.w *= material_ubo.alpha;
    if(result.alb.w < material_ubo.alpha_cutoff) {
        discard;
    }
    result.pos.xyz = input.pos;
    result.pos.w = 1.0;
    result.nrm.xyz = normalize(mul(
        float3x3(input.tng, input.btg, input.nrm), 
        (normal.Sample(sam, input.uv).xyz - 0.5) * 2.0));
    result.nrm.w = 1.0;
  // todo lots of work must be done in here
  // I must add any needed output for deferred part
  // w channel can hold useful info for deferred
  // its highly depends on my pbr render model
    return result;
}
