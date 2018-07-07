use super::super::core::object::{Basic as CoreBasic, Object as CoreObject};
use super::super::core::types::Id;
use super::engine::GraphicApiEngine;
use std::sync::{Arc, RwLock};
use gltf;
use math;

pub trait Object: CoreObject {
    fn name(&self) -> &str;
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
    fn set_orientation_location(&mut self, &math::Quaternion<f32>, &math::Vector3<f32>);
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Basic {
    pub name: String,
    pub renderable: bool,
    pub core_basic: CoreBasic,
}

impl Basic {
    pub fn new(name: &str) -> Self {
        let name = name.to_string();
        let renderable = true;
        let core_basic = CoreBasic::new();
        Basic {
            name,
            renderable,
            core_basic,
        }
    }
}

impl CoreObject for Basic {
    fn get_id(&self) -> Id {
        self.core_basic.get_id()
    }
}

impl Object for Basic {
    fn name(&self) -> &str {
        &self.name
    }

    fn render(&self) {}

    fn disable_rendering(&mut self) {
        self.renderable = false;
    }

    fn enable_rendering(&mut self) {
        self.renderable = true;
    }

    fn update(&mut self) {}
}

impl Loadable for Basic {
    fn new_with_gltf(node: &gltf::Node, _: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        Self::new(vxunwrap_o!(node.name()))
    }
}