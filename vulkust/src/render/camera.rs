use super::engine::GraphicApiEngine;
use super::object::{Loadable, Object, Transferable};
use gltf;
use math;
use std::collections::BTreeMap;
use std::convert::From;
#[cfg(debug_assertions)]
use std::fmt::Debug;
use std::sync::{Arc, RwLock};


pub struct Manager {
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub aspect_ratio: f32,
    pub cameras: BTreeMap<String, Arc<RwLock<Camera>>>,
    pub active_camera: Option<Arc<RwLock<Camera>>>,
}

impl Manager {
    pub fn new(gapi_engine: Arc<RwLock<GraphicApiEngine>>) -> Self {
        let aspect_ratio = {
            let eng = vxresult!(gapi_engine.read());
            let asp = vxresult!(eng.os_app.read()).aspect_ratio();
            asp
        };
        Manager {
            gapi_engine,
            aspect_ratio,
            cameras: BTreeMap::new(),
            active_camera: None,
        }
    }

    pub fn load(&mut self, n: gltf::Node) -> Arc<RwLock<Camera>> {
        let c = vxunwrap_o!(n.camera());
        let camera = match c.projection() {
            gltf::camera::Projection::Perspective(p) => {
                let camera: Arc<RwLock<Camera>> = Arc::new(RwLock::new(
                    Perspective::new_with_gltf_perspective(self.aspect_ratio, &c, p),
                ));
                camera
            }
            gltf::camera::Projection::Orthographic(o) => {
                let camera: Arc<RwLock<Camera>> =
                    Arc::new(RwLock::new(Orthographic::new_with_gltf_orthographic(o)));
                camera
            }
        };
        let name = vxunwrap_o!(c.name()).to_string();
        self.cameras.insert(name, camera.clone());
        let decomposeed = n.transform().decomposed();
        let (location, rotation, _scale) = decomposeed;
        let location = math::Vector3::new(location[0], location[1], location[2]);
        let rotation = math::Quaternion::new(rotation[3], rotation[0], rotation[1], rotation[2]);
        vxresult!(camera.write()).set_orientation_location(rotation, location);
        #[cfg(debug_assertions)]
        vxlogi!("Camera is: {:?}", &camera);
        self.active_camera = Some(camera.clone());
        camera
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Basic {
    pub near: f32,
    pub far: f32,
    pub aspect_ratio: f32,
    pub x: math::Vector3<f32>,
    pub y: math::Vector3<f32>,
    pub z: math::Vector3<f32>,
    pub location: math::Vector3<f32>,
    pub direction: math::Matrix4<f32>,
    pub view: math::Matrix4<f32>,
    pub projection: math::Matrix4<f32>,
    pub view_projection: math::Matrix4<f32>,
}

impl Basic {
    pub fn set_orientation(&mut self, q: math::Quaternion<f32>) {
        let rotation = math::Matrix4::from(q);
        self.x = (rotation * self.x.extend(1.0)).truncate();
        self.y = (rotation * self.y.extend(1.0)).truncate();
        self.z = (rotation * self.z.extend(1.0)).truncate();
        let mut q = q;
        q.s = -q.s;
        let rotation = math::Matrix4::from(q);
        let translate = math::Matrix4::from_translation(-self.location);
        self.direction = rotation;
        self.view = rotation * self.view * translate;
        self.view_projection = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 
            0.0, -1.0, 0.0, 0.0, 
            0.0, 0.0, 0.5, 0.0, 
            0.0, 0.0, 0.5, 1.0, 
        ) * self.projection * self.view;
    }

    pub fn set_orientation_location(&mut self, q: math::Quaternion<f32>, l: math::Vector3<f32>) {
        let rotation = math::Matrix4::from(q);
        self.x = (rotation * self.x.extend(1.0)).truncate();
        self.y = (rotation * self.y.extend(1.0)).truncate();
        self.z = (rotation * self.z.extend(1.0)).truncate();
        let mut q = q;
        q.s = -q.s;
        let rotation = math::Matrix4::from(q);
        self.location = l;
        let translate = math::Matrix4::from_translation(-l);
        self.direction = rotation;
        self.view = rotation * self.view * translate;
        self.view_projection = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        ) * self.projection * self.view;
    }
}

impl Default for Basic {
    fn default() -> Self {
        let identity = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        Basic {
            near: 0.0,
            far: 0.0,
            aspect_ratio: 0.0,
            x: math::Vector3::new(1.0, 0.0, 0.0),
            y: math::Vector3::new(0.0, 1.0, 0.0),
            z: math::Vector3::new(0.0, 0.0, -1.0),
            location: math::Vector3::new(0.0, 0.0, 0.0),
            direction: identity,
            view: identity,
            projection: identity,
            view_projection: identity,
        }
    }
}

#[cfg(debug_assertions)]
pub trait Camera: Object + Transferable + Debug {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        vxunimplemented!();
    }
}

#[cfg(not(debug_assertions))]
pub trait Camera: Object + Transferable {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        vxunimplemented!();
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Perspective {
    pub fov_vertical: f32,
    pub fov_horizontal: f32,
    pub tan_vertical: f32,
    pub tan_horizontal: f32,
    pub div_cos_vertical: f32,
    pub div_cos_horizontal: f32,
    pub name: String,
    pub basic: Basic,
}

impl Perspective {
    pub fn new_with_gltf_perspective(
        aspect_ratio: f32,
        c: &gltf::Camera,
        p: gltf::camera::Perspective,
    ) -> Self {
        let fov_vertical = p.yfov();
        let tan_vertical = (fov_vertical / 2.0).tan();
        let tan_horizontal = tan_vertical * aspect_ratio;
        let fov_horizontal = tan_horizontal.atan() * 2.0;
        let div_cos_vertical = (tan_vertical * tan_vertical + 1.0).sqrt();
        let div_cos_horizontal = (tan_horizontal * tan_horizontal + 1.0).sqrt();
        let name = vxunwrap_o!(c.name()).to_string();
        let mut basic = Basic::default();
        basic.aspect_ratio = aspect_ratio;
        basic.near = p.znear();
        basic.far = match p.zfar() {
            Some(f) => f,
            None => vxlogf!("Camera must have far/end (clipping) value."),
        };
        basic.projection =
            math::perspective(math::Rad(fov_vertical), aspect_ratio, basic.near, basic.far);
        basic.view_projection = basic.projection;
        Perspective {
            fov_vertical,
            fov_horizontal,
            tan_vertical,
            tan_horizontal,
            div_cos_vertical,
            div_cos_horizontal,
            name,
            basic,
        }
    }
}

impl Object for Perspective {}

impl Transferable for Perspective {
    fn set_orientation(&mut self, q: math::Quaternion<f32>) {
        self.basic.set_orientation(q);
    }

    fn set_orientation_location(&mut self, q: math::Quaternion<f32>, l: math::Vector3<f32>) {
        self.basic.set_orientation_location(q, l);
    }
}

impl Camera for Perspective {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        &self.basic.view_projection
    }
}

impl Loadable for Perspective {}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Orthographic {
    pub basic: Basic,
}

impl Orthographic {
    pub fn new_with_gltf_orthographic(_o: gltf::camera::Orthographic) -> Self {
        let basic = Basic::default();
        Orthographic { basic }
    }
}

impl Object for Orthographic {}

impl Transferable for Orthographic {}

impl Camera for Orthographic {}
