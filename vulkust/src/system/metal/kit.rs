use super::Id;

#[link(name = "MetalKit", kind = "framework")]
extern "C" {
    // fn MTKModelIOVertexDescriptorFromMetal(vertex_descriptor: Id) -> Id;
}

// pub fn model_io_vertex_descriptor_from_metal(vertex_descriptor: Id) -> Id {
//     unsafe {
//         MTKModelIOVertexDescriptorFromMetal(vertex_descriptor)
//     }
// }
