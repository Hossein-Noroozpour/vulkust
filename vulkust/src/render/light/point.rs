use super::super::super::core::gx3d::Gx3DReader;
use super::super::super::core::object::Object as CoreObject;
use super::super::super::core::types::{Id, Real};
use super::super::engine::Engine;
use super::super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use super::{DefaultLighting, Directional, Light, ShadowMaker};
use std::f32::consts::PI;

pub trait Point: Light {
    fn update_uniform(&self, &mut PointUniform);
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct PointUniform {
    color_minradius: cgmath::Vector4<Real>,
    position_radius: cgmath::Vector4<Real>,
}

impl PointUniform {
    pub fn new() -> Self {
        PointUniform {
            color_minradius: cgmath::Vector4::new(0.0, 0.0, 0.0, 0.0),
            position_radius: cgmath::Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    location: cgmath::Vector3<Real>,
    color: cgmath::Vector3<Real>,
    strength: Real,
    radius: Real, // by default I calculate the effective radius by `0.001 < (strength / (4 * VX_PI * radius * radius))`
    min_radius: Real,
}

impl Base {
    fn new() -> Self {
        Self::new_with_obj_base(ObjectBase::new())
    }

    fn new_with_obj_base(obj_base: ObjectBase) -> Self {
        Self {
            obj_base,
            location: cgmath::Vector3::new(0.0, 0.0, 0.0),
            color: cgmath::Vector3::new(1.0, 1.0, 1.0),
            strength: 1.0,
            radius: 80.0,
            min_radius: 0.1,
        }
    }

    fn set_strength(&mut self, strength: Real) {
        self.strength = strength;
        self.radius = (strength / (0.004 * PI)).sqrt();
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
    fn set_orientation(&mut self, _: &cgmath::Quaternion<Real>) {
        vxunexpected!();
    }

    fn set_location(&mut self, l: &cgmath::Vector3<Real>) {
        self.location = *l;
    }

    fn get_location(&self) -> cgmath::Vector3<Real> {
        return self.location;
    }

    fn move_local_z(&mut self, _: Real) {
        vxunexpected!(); // it does not have meaning for point light
    }

    fn move_local_x(&mut self, _: Real) {
        vxunexpected!();
    }

    fn rotate_local_x(&mut self, _: Real) {
        vxunexpected!();
    }

    fn rotate_global_z(&mut self, _: Real) {
        vxunexpected!();
    }

    fn translate(&mut self, t: &cgmath::Vector3<Real>) {
        self.location += *t;
    }

    fn scale(&mut self, _: Real) {
        vxunexpected!();
    }
}

impl Light for Base {
    fn to_directional(&self) -> Option<&Directional> {
        return None;
    }

    fn to_mut_directional(&mut self) -> Option<&mut Directional> {
        return None;
    }

    fn to_point(&self) -> Option<&Point> {
        return Some(self);
    }

    fn to_mut_point(&mut self) -> Option<&mut Point> {
        return Some(self);
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
        myself.location.x = reader.read();
        myself.location.y = reader.read();
        myself.location.z = reader.read();
        myself.color.x = reader.read();
        myself.color.y = reader.read();
        myself.color.z = reader.read();
        myself.set_strength(reader.read());
        #[cfg(debug_gx3d_light)]
        {
            vxlogi!("Direction {:?}", &myself.location);
            vxlogi!("Color {:?}", &myself.color);
            vxlogi!("Strength {:?}", &myself.strength);
        }
        return myself;
    }
}

impl Point for Base {
    fn update_uniform(&self, u: &mut PointUniform) {
        u.color_minradius = (self.color * self.strength).extend(self.min_radius);
        u.position_radius = self.location.extend(self.radius);
    }
}

impl DefaultLighting for Base {
    fn default(_: &Engine) -> Self {
        return Self::new();
    }
}
