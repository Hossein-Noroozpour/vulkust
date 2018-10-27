use super::super::core::object::{Base as CoreBase, Object as CoreObject};
use super::super::core::types::{Id, Real};
use super::command::Buffer as CmdBuffer;
use super::engine::Engine;
use super::gx3d::Gx3DReader;
use gltf;
use math;

pub trait Object: CoreObject {
    fn get_name(&self) -> Option<String>;
    fn set_name(&mut self, name: &str);
    fn render(&self, &mut CmdBuffer, usize);
    fn disable_rendering(&mut self);
    fn enable_rendering(&mut self);
    fn is_rendarable(&self) -> bool;
    fn update(&mut self, usize);
}

pub trait Loadable: Sized {
    fn new_with_gltf(&gltf::Node, &Engine, &[u8]) -> Self;
    fn new_with_gx3d(&Engine, &mut Gx3DReader, Id) -> Self;
}

pub trait Transferable {
    fn set_orientation(&mut self, &math::Quaternion<Real>);
    fn set_location(&mut self, &math::Vector3<Real>);
    fn get_location(&self) -> &math::Vector3<Real>;
    fn move_local_z(&mut self, Real);
    fn move_local_x(&mut self, Real);
    fn rotate_local_x(&mut self, Real);
    fn rotate_global_z(&mut self, Real);
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

    fn render(&self, _cmd: &mut CmdBuffer, _: usize) {
        #[cfg(debug_mode)]
        {
            if !self.renderable {
                vxunexpected!();
            }
        }
    }

    fn disable_rendering(&mut self) {
        self.renderable = false;
    }

    fn enable_rendering(&mut self) {
        self.renderable = true;
    }
    fn is_rendarable(&self) -> bool {
        return self.renderable;
    }

    fn update(&mut self, _: usize) {}
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
