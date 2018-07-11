use super::mesh::{Base as MeshBase, Mesh};

pub trait Widget: Mesh {

}

pub struct Label {
    pub text: String,
}