use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::engine::GraphicApiEngine;
use super::mesh::{Base as MeshBase, DefaultMesh, Mesh};
use super::object::Object;
use super::scene::Uniform as SceneUniform;
use std::sync::{Arc, RwLock};

pub trait Widget: Mesh {}

pub struct Base {
    pub mesh_base: MeshBase,
    pub sensitive: bool,
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.mesh_base.get_id()
    }
}

impl Object for Base {
    fn render(&self) {
        self.mesh_base.render();
    }

    fn disable_rendering(&mut self) {
        self.mesh_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.mesh_base.enable_rendering()
    }

    fn update(&mut self) {
        self.mesh_base.update();
    }
}

impl Mesh for Base {
    fn render(&mut self, su: &SceneUniform) {
        Mesh::render(&mut self.mesh_base, su);
    }
}

impl DefaultMesh for Base {
    fn default(gapi_engine: Arc<RwLock<GraphicApiEngine>>) -> Self {
        Base {
            mesh_base: MeshBase::default(gapi_engine),
            sensitive: true,
        }
    }
}

impl Widget for Base {}

pub struct Label {
    pub base: Base,
    pub text: String,
}

impl Label {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        // todo
    }
}

impl CoreObject for Label {
    fn get_id(&self) -> Id {
        self.base.get_id()
    }
}

impl Object for Label {
    fn render(&self) {
        self.base.render();
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn update(&mut self) {
        self.base.update();
    }
}

impl Mesh for Label {
    fn render(&mut self, su: &SceneUniform) {
        Mesh::render(&mut self.base, su);
    }
}

impl DefaultMesh for Label {
    fn default(gapi_engine: Arc<RwLock<GraphicApiEngine>>) -> Self {
        Label {
            base: Base::default(gapi_engine),
            text: String::new(),
        }
    }
}

impl Widget for Label {}
