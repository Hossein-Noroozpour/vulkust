use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::engine::GraphicApiEngine;
use super::object::{Basic as BasicObject, Loadable, Object, Transferable};
use gltf;
use math;
use std::collections::BTreeMap;
use std::convert::From;
#[cfg(debug_assertions)]
use std::fmt::Debug;
use std::sync::{Arc, RwLock, Weak};

#[cfg(debug_assertions)]
pub trait Camera: Object + Transferable + Debug {
    fn get_view_projection(&self) -> &math::Matrix4<f32>;
    fn get_cascaded_shadow_points(&self, _sections_count: usize) -> Vec<math::Vector3<f32>>;
}

#[cfg(not(debug_assertions))]
pub trait Camera: Object + Transferable {
    fn get_view_projection(&self) -> &math::Matrix4<f32>;
    fn get_cascaded_shadow_points(&self, _sections_count: usize) -> Vec<math::Vector3<f32>>;
}

pub struct Manager {
    pub gapi_engine: Weak<RwLock<GraphicApiEngine>>,
    pub cameras: BTreeMap<Id, Weak<RwLock<Camera>>>,
    pub name_to_id: BTreeMap<String, Id>,
}

impl Manager {
    pub fn new(gapi_engine: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        // let aspect_ratio = {
        //     let eng = vxresult!(gapi_engine.read());
        //     let asp = vxresult!(eng.os_app.read()).aspect_ratio();
        //     asp
        // };
        let gapi_engine = Arc::downgrade(gapi_engine);
        let cameras = BTreeMap::new();
        let name_to_id = BTreeMap::new();
        Manager {
            gapi_engine,
            cameras,
            name_to_id,
        }
    }

    pub fn load(&mut self, n: &gltf::Node) -> Arc<RwLock<Camera>> {
        let c = vxunwrap_o!(n.camera());
        let name = vxunwrap_o!(c.name()).to_string();
        if let Some(id) = self.name_to_id.get(&name) {
            if let Some(c) = self.cameras.get(&id) {
                if let Some(c) = c.upgrade() {
                    return c;
                }
            }
        }
        let eng = vxunwrap_o!(self.gapi_engine.upgrade());
        let camera = match c.projection() {
            gltf::camera::Projection::Perspective(p) => {
                let camera: Arc<RwLock<Camera>> = Arc::new(RwLock::new(
                    Perspective::new_with_gltf(n, &eng),
                ));
                camera
            }
            gltf::camera::Projection::Orthographic(o) => {
                let camera: Arc<RwLock<Camera>> = Arc::new(RwLock::new(
                    Orthographic::new_with_gltf(n, &eng)));
                camera
            }
        };
        let id = vxresult!(camera.read()).get_id();
        #[cfg(debug_assertions)]
        vxlogi!("Camera is: {:?}", &camera);
        self.cameras.insert(id, Arc::downgrade(&camera));
        camera
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Basic {
    pub obj_basic: BasicObject,
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
    pub fn new(name: &str) -> Self {
        let identity = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        Basic {
            obj_basic: BasicObject::new(name),
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

    pub fn update_view_projection(&mut self) {
        self.view_projection = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        ) * self.projection * self.view;
    }
}

impl CoreObject for Basic {
    fn get_id(&self) -> Id {
        self.obj_basic.get_id()
    }
}

impl Object for Basic {
    fn name(&self) -> &str {
        self.obj_basic.name()
    }

    fn render(&self) {
        vxlogf!("Basic camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.obj_basic.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_basic.enable_rendering()
    }

    fn update(&mut self) {}
}

impl Transferable for Basic {
    fn set_orientation(&mut self, q: &math::Quaternion<f32>) {
        let rotation = math::Matrix4::from(*q);
        self.x = (rotation * self.x.extend(1.0)).truncate();
        self.y = (rotation * self.y.extend(1.0)).truncate();
        self.z = (rotation * self.z.extend(1.0)).truncate();
        let mut q = q;
        q.s = -q.s;
        let rotation = math::Matrix4::from(*q);
        let translate = math::Matrix4::from_translation(-self.location);
        self.direction = rotation;
        self.view = rotation * self.view * translate;
        self.update_view_projection();
    }

    fn set_orientation_location(&mut self, q: &math::Quaternion<f32>, l: &math::Vector3<f32>) {
        self.location = *l;
        self.set_orientation(q);
    }
}

impl Loadable for Basic {
     fn new_with_gltf(node: &gltf::Node, eng: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        let eng = vxresult!(eng.read());
        let aspect_ratio = vxresult!(eng.os_app.read()).aspect_ratio();
        let identity = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let obj_basic = BasicObject::new(vxunwrap_o!(node.name()));
        let c = vxunwrap_o!(node.camera());
        let (near, far) = match c.projection() {
            gltf::camera::Projection::Perspective(p) => {
                (p.znear(), vxunwrap_o!(p.zfar()))
            },
            gltf::camera::Projection::Orthographic(p) => {
                (p.znear(), p.zfar())
            },
        };
        let mut basic = Basic {
            obj_basic,
            near,
            far,
            aspect_ratio,
            x: math::Vector3::new(1.0, 0.0, 0.0),
            y: math::Vector3::new(0.0, 1.0, 0.0),
            z: math::Vector3::new(0.0, 0.0, -1.0),
            location: math::Vector3::new(0.0, 0.0, 0.0),
            direction: identity,
            view: identity,
            projection: identity,
            view_projection: identity,
        };
        let decomposed = node.transform().decomposed(); 
        let (l, r, _) = decomposed;
        let location = math::Vector3::new(l[0], l[1], l[2]);
        let rotation = math::Quaternion::new(r[3], r[0], r[1], r[2]);
        basic.set_orientation_location(&rotation, &location);
        return basic;
    }
}

impl Camera for Basic {
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Perspective {
    pub basic: Basic,
    pub fov_vertical: f32,
    pub fov_horizontal: f32,
    pub tan_vertical: f32,
    pub tan_horizontal: f32,
    pub div_cos_vertical: f32,
    pub div_cos_horizontal: f32,
}

impl Perspective {

}

impl CoreObject for Perspective {
    fn get_id(&self) -> Id {
        self.basic.get_id()
    }
}

impl Object for Perspective {
    fn name(&self) -> &str {
        self.basic.name()
    }

    fn render(&self) {
        vxlogf!("Perspective camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.basic.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.basic.enable_rendering()
    }

    fn update(&mut self) {
        self.basic.update();
    }
}

impl Loadable for Perspective {
    fn new_with_gltf(n: &gltf::Node, eng: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        let c = vxunwrap_o!(n.camera());
        let p = match c.projection() {
            gltf::camera::Projection::Perspective(p) => p,
            gltf::camera::Projection::Orthographic(o) => 
                vxlogf!("Type of camera isn't perspective."),
        };
        let basic = Basic::new_with_gltf(n, eng);
        let fov_vertical = p.yfov();
        let tan_vertical = (fov_vertical / 2.0).tan();
        let tan_horizontal = tan_vertical * basic.aspect_ratio;
        let fov_horizontal = tan_horizontal.atan() * 2.0;
        let div_cos_vertical = (tan_vertical * tan_vertical + 1.0).sqrt();
        let div_cos_horizontal = (tan_horizontal * tan_horizontal + 1.0).sqrt();
        basic.projection =
            math::perspective(math::Rad(fov_vertical), basic.aspect_ratio, basic.near, basic.far);
        basic.update_view_projection();
        Perspective {
            basic,
            fov_vertical,
            fov_horizontal,
            tan_vertical,
            tan_horizontal,
            div_cos_vertical,
            div_cos_horizontal,
        }
    }
}

impl Transferable for Perspective {
    fn set_orientation(&mut self, q: &math::Quaternion<f32>) {
        self.basic.set_orientation(q);
    }

    fn set_orientation_location(&mut self, q: &math::Quaternion<f32>, l: &math::Vector3<f32>) {
        self.basic.set_orientation_location(q, l);
    }
}

impl Camera for Perspective {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        &self.basic.view_projection
    }

    fn get_cascaded_shadow_points(&self, sections_count: usize) -> Vec<math::Vector3<f32>> {
        #[cfg(debug_assertions)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }
        let mut result = vec![math::Vector3::new(0.0f32, 0.0f32, 0.0f32); sections_count + 1];
        result[0] = self.basic.location + self.basic.z * self.basic.near;
        if sections_count > 1 {
            let oneminlambda = 0.5 / self.div_cos_horizontal + 0.5 / self.div_cos_horizontal;
            let lambda = 1.0 - oneminlambda;
            let onedivcn = 1.0 / sections_count as f32;
            let unisecinc = oneminlambda * onedivcn * (self.basic.far - self.basic.near);
            let fdivn = self.basic.far / self.basic.near;
            let logsecmul = fdivn.powf(onedivcn);
            let mut unisec = oneminlambda * self.basic.near + unisecinc;
            let mut logsec = lambda * self.basic.near * logsecmul;
            result[1] = self.basic.location + self.basic.z * (logsec + unisec);
            for i in 2..sections_count {
                logsec *= logsecmul;
                unisec += unisecinc;
                result[i] = self.basic.location + self.basic.z * (logsec + unisec);
            }
        }
        result[sections_count] = self.basic.location + self.basic.z * self.basic.far;
        return result;
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Orthographic {
    pub basic: Basic,
    pub size: f32,
}

impl Orthographic {}

impl CoreObject for Orthographic {
    fn get_id(&self) -> Id {
        self.basic.get_id()
    }
}

impl Object for Orthographic {
    fn name(&self) -> &str {
        self.basic.name()
    }

    fn render(&self) {
        vxlogf!("Orthographic camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.basic.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.basic.enable_rendering()
    }

    fn update(&mut self) {
        self.basic.update();
    }
}

impl Loadable for Orthographic {
    fn new_with_gltf(n: &gltf::Node, eng: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        let c = vxunwrap_o!(n.camera());
        let o = match c.projection() {
            gltf::camera::Projection::Perspective(p) => 
                vxlogf!("Type of camera isn't perspective."),
            gltf::camera::Projection::Orthographic(o) => o,
        };
        let basic = Basic::new_with_gltf(n, eng);
        let size = o.ymag();
        let right = size * basic.aspect_ratio * 0.5; 
        let left = -right;
        let top = size * 0.5;
        let bottom = -top;
        basic.projection =
            math::ortho(left, right, bottom, top, basic.near, basic.far);
        basic.update_view_projection();
        Orthographic {
            basic,
            size,
        }
    }
}

impl Transferable for Orthographic {
    fn set_orientation(&mut self, q: &math::Quaternion<f32>) {
        self.basic.set_orientation(q);
    }

    fn set_orientation_location(&mut self, q: &math::Quaternion<f32>, l: &math::Vector3<f32>) {
        self.basic.set_orientation_location(q, l);
    }
}

impl Camera for Orthographic {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        &self.basic.view_projection
    }

    fn get_cascaded_shadow_points(&self, sections_count: usize) -> Vec<math::Vector3<f32>> {
        #[cfg(debug_assertions)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }
        let mut result = vec![math::Vector3::new(0.0f32, 0.0f32, 0.0f32); sections_count + 1];
        let mut previous = self.basic.location + self.basic.z * self.basic.near;
        result[0] = previous;
        let unisecinc = (self.basic.far - self.basic.near) / sections_count as f32;
        let unisecinc = self.basic.z * unisecinc;
        let sections_count = sections_count + 1;
        for i in 1..sections_count {
            previous += unisecinc;
            result[i] = previous;
        }
        return result;
    }
}
