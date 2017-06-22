use super::Id;

#[link(name = "ModelIO", kind = "framework")]
extern "C" {
    pub static MDLVertexAttributePosition: Id;
    pub static MDLVertexAttributeTextureCoordinate: Id;
    pub static MDLVertexAttributeNormal: Id;
}