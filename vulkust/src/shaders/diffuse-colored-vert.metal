#include <metal_stdlib>
#include <simd/simd.h>


// Buffer index values shared between shader and C code to ensure Metal shader buffer inputs match
//   Metal API buffer set calls
typedef enum BufferIndices
{
    kBufferIndexMeshPositions = 0,
    kBufferIndexMeshGenerics  = 1,
    kBufferIndexUniforms      = 2
} BufferIndices;

// Attribute index values shared between shader and C code to ensure Metal shader vertex
//   attribute indices match the Metal API vertex descriptor attribute indices
typedef enum VertexAttributes
{
    kVertexAttributePosition  = 0,
    kVertexAttributeTexcoord  = 1,
    kVertexAttributeNormal    = 2,
} VertexAttributes;

// Texture index values shared between shader and C code to ensure Metal shader texture indices
//   match indices of Metal API texture set calls
typedef enum TextureIndices
{
    kTextureIndexColor    = 0,
} TextureIndices;

// Structure shared between shader and C code to ensure the layout of uniform data accessed in
//    Metal shaders matches the layout of uniform data set in C code
typedef struct
{
    // Per Frame Uniforms
    matrix_float4x4 projectionMatrix;
    matrix_float4x4 viewMatrix;

    // Per Mesh Uniforms
    float materialShininess;
    matrix_float4x4 modelViewMatrix;
    matrix_float3x3 normalMatrix;

    // Per Light Properties
    vector_float3 ambientLightColor;
    vector_float3 directionalLightDirection;
    vector_float3 directionalLightColor;

} Uniforms;

using namespace metal;

// Per-vertex inputs fed by vertex buffer laid out with MTLVertexDescriptor in Metal API
typedef struct
{
    float3 position [[attribute(kVertexAttributePosition)]];
    float2 texCoord [[attribute(kVertexAttributeTexcoord)]];
    half3 normal    [[attribute(kVertexAttributeNormal)]];
} Vertex;

// Vertex shader outputs and per-fragmeht inputs.  Includes clip-space position and vertex outputs
//  interpolated by rasterizer and fed to each fragment genterated by clip-space primitives.
typedef struct
{
    float4 position [[position]];
    float2 texCoord;
    half3  eyePosition;
    half3  normal;
} ColorInOut;

// Vertex function
vertex ColorInOut main_func(Vertex in [[stage_in]],
                                  constant Uniforms & uniforms [[ buffer(kBufferIndexUniforms) ]])
{
    ColorInOut out;

    // Make position a float4 to perform 4x4 matrix math on it
    float4 position = float4(in.position, 1.0);

    // Calculate the position of our vertex in clip space and output for clipping and rasterization
    out.position = uniforms.projectionMatrix * uniforms.modelViewMatrix * position;

    // Pass along the texture coordinate of our vertex such which we'll use to sample from texture's
    //   in our fragment function
    out.texCoord = in.texCoord;

    // Calculate the positon of our vertex in eye space
    out.eyePosition = half3((uniforms.modelViewMatrix * position).xyz);

    // Rotate our normals by the normal matrix
    half3x3 normalMatrix = half3x3(uniforms.normalMatrix);
    out.normal = normalize(normalMatrix * in.normal);
    return out;
}
