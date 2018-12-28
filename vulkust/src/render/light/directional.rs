use super::super::super::core::gx3d::Gx3DReader;
use super::super::super::core::object::Object as CoreObject;
use super::super::super::core::types::{Id, Real};
use super::super::config::MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT;
use super::super::engine::Engine;
use super::super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use super::{DefaultLighting, Light, Point, ShadowMaker, Sun};

use cgmath;

pub trait Directional: Light {
    fn to_sun(&self) -> Option<&Sun>;
    fn to_mut_sun(&mut self) -> Option<&mut Sun>;
    fn to_base(&self) -> Option<&Base>;
    fn to_mut_base(&mut self) -> Option<&mut Base>;
    fn update_uniform(&self, &mut DirectionalUniform);
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct DirectionalUniform {
    pub(super) color: cgmath::Vector4<Real>,
    pub(super) direction: cgmath::Vector4<Real>,
}

impl DirectionalUniform {
    pub(crate) fn new() -> Self {
        Self {
            color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
            direction: cgmath::Vector4::new(0.0, 0.0, -1.0, 1.0),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct ShadowAccumulatorDirectionalUniform {
    pub(super) view_projection_biases:
        [cgmath::Matrix4<Real>; MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
    pub(super) direction_strength: cgmath::Vector4<Real>,
    pub(super) cascades_count: u32,
    pub(super) light_index: u32,
}

impl ShadowAccumulatorDirectionalUniform {
    pub(super) fn new() -> Self {
        Self {
            view_projection_biases: [cgmath::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
            ); MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
            direction_strength: cgmath::Vector4::new(0.0, 0.0, -1.0, 1.0),
            cascades_count: 0,
            light_index: 0,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    direction: cgmath::Vector3<Real>,
    color: cgmath::Vector3<Real>,
    strength: Real,
}

impl Base {
    fn new() -> Self {
        Self::new_with_obj_base(ObjectBase::new())
    }

    fn new_with_obj_base(obj_base: ObjectBase) -> Self {
        Self {
            obj_base,
            direction: cgmath::Vector3::new(0.0, 0.0, -1.0),
            color: cgmath::Vector3::new(1.0, 1.0, 1.0),
            strength: 1.0,
        }
    }
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Object for Base {
    fn get_name(&self) -> Option<String> {
        self.obj_base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.obj_base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn is_renderable(&self) -> bool {
        return self.obj_base.is_renderable();
    }
}

impl Transferable for Base {
    fn set_orientation(&mut self, q: &cgmath::Quaternion<Real>) {
        let rotation = cgmath::Matrix3::from(*q);
        self.direction = rotation * cgmath::Vector3::new(0.0, 0.0, -1.0);
    }

    fn set_location(&mut self, _: &cgmath::Vector3<Real>) {
        vxunexpected!();
    }

    fn get_location(&self) -> cgmath::Vector3<Real> {
        vxunexpected!();
    }

    fn move_local_z(&mut self, _: Real) {
        vxunexpected!();
    }

    fn move_local_x(&mut self, _: Real) {
        vxunexpected!();
    }

    fn rotate_local_x(&mut self, _: Real) {
        vxunimplemented!();
    }

    fn rotate_global_z(&mut self, _: Real) {
        vxunimplemented!();
    }

    fn translate(&mut self, _: &cgmath::Vector3<Real>) {
        vxunexpected!();
    }

    fn scale(&mut self, _: Real) {
        vxunexpected!();
    }
}

impl Light for Base {
    fn to_directional(&self) -> Option<&Directional> {
        return Some(self);
    }

    fn to_mut_directional(&mut self) -> Option<&mut Directional> {
        return Some(self);
    }

    fn to_point(&self) -> Option<&Point> {
        return None;
    }

    fn to_mut_point(&mut self) -> Option<&mut Point> {
        return None;
    }

    fn to_shadow_maker(&self) -> Option<&ShadowMaker> {
        return None;
    }

    fn to_mut_shadow_maker(&mut self) -> Option<&mut ShadowMaker> {
        return None;
    }

    fn update(&mut self) {}
}

impl Loadable for Base {
    fn new_with_gltf(_node: &gltf::Node, _eng: &Engine, _: &[u8]) -> Self {
        vxunimplemented!();
    }

    fn new_with_gx3d(_: &Engine, reader: &mut Gx3DReader, id: Id) -> Self {
        let mut myself = Self::new_with_obj_base(ObjectBase::new_with_id(id));
        let r = [
            reader.read::<Real>(),
            reader.read::<Real>(),
            reader.read::<Real>(),
            reader.read::<Real>(),
        ];
        myself.set_orientation(&cgmath::Quaternion::new(r[0], r[1], r[2], r[3]));
        myself.color = cgmath::Vector3::new(
            reader.read::<Real>(),
            reader.read::<Real>(),
            reader.read::<Real>(),
        );
        myself.strength = reader.read::<Real>();
        #[cfg(debug_gx3d)]
        {
            vxlogi!("Direction {:?}", &myself.direction);
            vxlogi!("Color {:?}", &myself.color);
            vxlogi!("Strength {:?}", &myself.strength);
        }
        return myself;
    }
}

impl Directional for Base {
    fn to_sun(&self) -> Option<&Sun> {
        return None;
    }

    fn to_mut_sun(&mut self) -> Option<&mut Sun> {
        return None;
    }

    fn to_base(&self) -> Option<&Base> {
        return Some(self);
    }

    fn to_mut_base(&mut self) -> Option<&mut Base> {
        return Some(self);
    }

    fn update_uniform(&self, u: &mut DirectionalUniform) {
        u.color = (self.color * self.strength).extend(1.0);
        u.direction = self.direction.extend(self.strength);
    }
}

impl DefaultLighting for Base {
    fn default(_: &Engine) -> Self {
        return Self::new();
    }
}
