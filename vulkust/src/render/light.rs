use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::camera::Orthographic;
use super::engine::Engine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::object::{Loadable, Object, Transferable};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use math;

#[cfg_attr(debug_assertions, derive(Debug))]
#[repr(u8)]
pub enum TypeId {
    Sun = 1,
}

pub trait Light: Object {
    // fn set_cascaded_frustums() // todo
}

pub trait DefaultLighting {
    fn default(eng: &Arc<RwLock<Engine>>, size: f32) -> Self;
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Sun {
    camera: Orthographic,
    color: (f32, f32, f32),
    strength: f32,
}

impl Sun {}

impl CoreObject for Sun {
    fn get_id(&self) -> Id {
        self.camera.get_id()
    }
}

impl Object for Sun {
    fn get_name(&self) -> Option<String> {
        self.camera.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.camera.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn render(&self, _: &Engine) {
        vxlogf!("Sun light does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.camera.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.camera.enable_rendering()
    }

    fn update(&mut self) {
        self.camera.update();
    }
}

impl Light for Sun {}

impl DefaultLighting for Sun {
    fn default(eng: &Arc<RwLock<Engine>>, size: f32) -> Self {
        Sun {
            camera: Orthographic::new(eng, size),
            color: (1.0, 1.0, 1.0),
            strength: 1.0,
        }
    }
}

impl Loadable for Sun {
    fn new_with_gltf(_node: &gltf::Node, _eng: &Arc<RwLock<Engine>>, _: &[u8]) -> Self {
        vxunimplemented!();
    }

    fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader, id: Id) -> Self {
        let location = math::Vector3::new(reader.read(), reader.read(), reader.read());
        let r = [reader.read(), reader.read(), reader.read(), reader.read()];
        let r = math::Quaternion::new(r[3], r[0], r[1], r[2]);
        let mut camera = Orthographic::new_with_id(engine, id);
        camera.set_location(&location);
        camera.set_orientation(&r);
        let color = (reader.read(), reader.read(), reader.read());
        let strength = reader.read();
        Sun {
            camera,
            color,
            strength,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub lights: BTreeMap<Id, Weak<RwLock<Light>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            lights: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub fn create<L>(&mut self, eng: &Arc<RwLock<Engine>>, name: &str) -> Arc<RwLock<L>>
    where
        L: 'static + Light + DefaultLighting,
    {
        let result = L::default(eng, 1.0);
        let id = result.get_id();
        let result = Arc::new(RwLock::new(result));
        let light: Arc<RwLock<Light>> = result.clone();
        self.lights.insert(id, Arc::downgrade(&light));
        self.name_to_id.insert(name.to_string(), id);
        return result;
    }

    pub fn load_gx3d(&mut self, eng: &Arc<RwLock<Engine>>, id: Id) -> Arc<RwLock<Light>> {
        if let Some(light) = self.lights.get(&id) {
            if let Some(light) = light.upgrade() {
                return light;
            }
        }
        let table = vxunwrap!(&mut self.gx3d_table);
        table.goto(id);
        let reader: &mut Gx3DReader = &mut table.reader;
        let type_id = reader.read_type_id();
        let result: Arc<RwLock<Light>> = if type_id == TypeId::Sun as u8 {
            Arc::new(RwLock::new(Sun::new_with_gx3d(eng, reader, id)))
        } else {
            vxunexpected!();
        };
        self.lights.insert(id, Arc::downgrade(&result));
        return result;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PointUniform {
    color: math::Vector4<Real>,
    position_radius: math::Vector4<Real>,
}

impl PointUniform {
    pub fn new() -> Self {
        PointUniform {
            color: math::Vector4::new(0.0, 0.0, 0.0, 0.0),
            position_radius: math::Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct DirectionalUniform {
    color: math::Vector4<Real>,
    direction: math::Vector4<Real>,
    view_projection_biased: math::Matrix4<Real>,
}

impl DirectionalUniform {
    pub fn new() -> Self {
        DirectionalUniform {
            color: math::Vector4::new(0.0, 0.0, 0.0, 0.0),
            direction: math::Vector4::new(0.0, 0.0, -1.0, 0.0),
            view_projection_biased: math::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
            ),
        }
    }
}
