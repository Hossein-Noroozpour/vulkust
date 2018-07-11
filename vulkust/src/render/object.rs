use super::super::core::object::{Base as CoreBase, Object as CoreObject};
use super::super::core::types::Id;
use super::engine::GraphicApiEngine;
use gltf;
use math;
use std::sync::{Arc, RwLock};

pub trait Object: CoreObject {
    fn render(&self);
    fn disable_rendering(&mut self);
    fn enable_rendering(&mut self);
    fn update(&mut self);
}

pub trait Loadable: Sized {
    fn new_with_gltf(&gltf::Node, &Arc<RwLock<GraphicApiEngine>>) -> Self;
}

pub trait Transferable {
    fn set_orientation(&mut self, &math::Quaternion<f32>);
    fn set_location(&mut self, &math::Vector3<f32>);
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Base {
    pub renderable: bool,
    pub core_base: CoreBase,
}

impl Base {
    pub fn new() -> Self {
        let renderable = true;
        let core_base = CoreBase::new();
        Base {
            renderable,
            core_base,
        }
    }
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.core_base.get_id()
    }
}

impl Object for Base {
    fn render(&self) {}

    fn disable_rendering(&mut self) {
        self.renderable = false;
    }

    fn enable_rendering(&mut self) {
        self.renderable = true;
    }

    fn update(&mut self) {}
}

impl Loadable for Base {
    fn new_with_gltf(_: &gltf::Node, _: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        Self::new()
    }
}
