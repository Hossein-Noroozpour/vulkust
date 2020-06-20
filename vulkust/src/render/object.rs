use super::super::core::gx3d::Gx3DReader;
use super::super::core::object::{Base as CoreBase, Object as CoreObject};
use super::super::core::types::{Id, Real};
use super::engine::Engine;
use cgmath;
use gltf;

pub trait Object: CoreObject {
    fn get_name(&self) -> Option<String>;
    fn set_name(&mut self, name: &str);
    fn disable_rendering(&mut self);
    fn enable_rendering(&mut self);
    fn is_renderable(&self) -> bool;
}

pub trait Loadable: Sized {
    fn new_with_gltf(node: &gltf::Node, engine: &Engine, data: &[u8]) -> Self;
    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, id: Id) -> Self;
}

pub trait Transferable {
    fn set_orientation(&mut self, quad: &cgmath::Quaternion<Real>);
    fn set_location(&mut self, loc: &cgmath::Vector3<Real>);
    fn get_location(&self) -> cgmath::Vector3<Real>;
    fn move_local_z(&mut self, degree: Real);
    fn move_local_x(&mut self, degree: Real);
    fn rotate_local_x(&mut self, degree: Real);
    fn rotate_global_z(&mut self, degree: Real);
    fn translate(&mut self, _: &cgmath::Vector3<Real>) {
        // todo temporary
        vxunimplemented!();
    }
    fn scale(&mut self, _: Real) {
        // todo temporary
        vxunimplemented!();
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    pub name: Option<String>,
    pub renderable: bool,
    pub core_base: CoreBase,
}

impl Base {
    pub fn new() -> Self {
        let name = None;
        let renderable = true;
        let core_base = CoreBase::new();
        Base {
            name,
            renderable,
            core_base,
        }
    }

    pub fn new_with_id(id: Id) -> Self {
        let name = None;
        let renderable = true;
        let core_base = CoreBase::new_with_id(id);
        Base {
            name,
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
    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    fn disable_rendering(&mut self) {
        self.renderable = false;
    }

    fn enable_rendering(&mut self) {
        self.renderable = true;
    }

    fn is_renderable(&self) -> bool {
        return self.renderable;
    }
}

impl Loadable for Base {
    fn new_with_gltf(node: &gltf::Node, _: &Engine, _: &[u8]) -> Self {
        let name = match node.name() {
            Some(s) => Some(s.to_string()),
            None => None,
        };
        let renderable = true;
        let core_base = CoreBase::new();
        Base {
            name,
            renderable,
            core_base,
        }
    }

    fn new_with_gx3d(_: &Engine, _: &mut Gx3DReader, my_id: Id) -> Self {
        Self::new_with_id(my_id)
    }
}
