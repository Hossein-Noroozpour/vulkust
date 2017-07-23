#include <metal_stdlib>
#include <simd/simd.h>
typedef enum BufferIndices {
    kBufferIndexMeshPositions = 0,
    kBufferIndexUniforms      = 1
} BufferIndices;
typedef enum VertexAttributes {
    kVertexAttributePosition  = 0,
} VertexAttributes;
typedef struct {
    matrix_float4x4 mvp;
} Uniforms;
using namespace metal;
typedef struct {
    float3 position [[attribute(kVertexAttributePosition)]];
} Vertex;
typedef struct {
    float4 position [[position]];
} VertexInOut;
vertex VertexInOut main_vertex_func(
        Vertex in [[stage_in]],
        constant Uniforms & uniforms [[ buffer(kBufferIndexUniforms) ]]) {
    VertexInOut out;
    float4 position = float4(in.position, 1.0);
    out.position = uniforms.mvp * position;
    return out;
}
