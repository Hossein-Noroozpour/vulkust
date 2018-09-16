use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::engine::Engine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use gltf;
use math;
use std::collections::BTreeMap;
use std::convert::From;
use std::sync::{Arc, RwLock, Weak};

pub trait Camera: Object + Transferable {
    fn get_view_projection(&self) -> &math::Matrix4<f32>;
    fn get_cascaded_shadow_points(&self, _sections_count: usize) -> Vec<math::Vector3<f32>>;
}

pub trait DefaultCamera: Camera {
    fn default(&Arc<RwLock<Engine>>) -> Self;
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[repr(u8)]
pub enum TypeId {
    Perspective = 1,
    Orthographic = 2,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub cameras: BTreeMap<Id, Weak<RwLock<Camera>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        let cameras = BTreeMap::new();
        let name_to_id = BTreeMap::new();
        Manager {
            cameras,
            name_to_id,
            gx3d_table: None,
        }
    }

    pub fn load_gltf(&mut self, n: &gltf::Node, eng: &Arc<RwLock<Engine>>) -> Arc<RwLock<Camera>> {
        let c = vxunwrap!(n.camera());
        let data = Vec::new();
        let camera = match c.projection() {
            gltf::camera::Projection::Perspective(_) => {
                let camera: Arc<RwLock<Camera>> =
                    Arc::new(RwLock::new(Perspective::new_with_gltf(n, eng, &data)));
                camera
            }
            gltf::camera::Projection::Orthographic(_) => {
                let camera: Arc<RwLock<Camera>> =
                    Arc::new(RwLock::new(Orthographic::new_with_gltf(n, eng, &data)));
                camera
            }
        };
        let id = vxresult!(camera.read()).get_id();
        #[cfg(debug_assertions)]
        vxlogi!("Camera is: {:?}", &camera);
        self.cameras.insert(id, Arc::downgrade(&camera));
        if let Some(name) = n.name() {
            let name = name.to_string();
            self.name_to_id.insert(name, id);
        }
        camera
    }

    pub fn load_gx3d(&mut self, engine: &Arc<RwLock<Engine>>, id: Id) -> Arc<RwLock<Camera>> {
        if let Some(camera) = self.cameras.get(&id) {
            if let Some(camera) = camera.upgrade() {
                return camera;
            }
        }
        let table = vxunwrap!(&mut self.gx3d_table);
        table.goto(id);
        let reader: &mut Gx3DReader = &mut table.reader;
        let type_id = reader.read_type_id();
        let camera: Arc<RwLock<Camera>> = if type_id == TypeId::Perspective as u8 {
            Arc::new(RwLock::new(Perspective::new_with_gx3d(engine, reader, id)))
        } else if type_id == TypeId::Orthographic as u8 {
            Arc::new(RwLock::new(Orthographic::new_with_gx3d(engine, reader, id)))
        } else {
            vxunexpected!();
        };
        self.cameras.insert(id, Arc::downgrade(&camera));
        camera
    }

    pub fn create<C>(&mut self, eng: &Arc<RwLock<Engine>>) -> Arc<RwLock<C>>
    where
        C: 'static + DefaultCamera,
    {
        let camera = C::default(eng);
        let id = camera.get_id();
        if let Some(name) = camera.get_name() {
            self.name_to_id.insert(name, id);
        }
        let camera = Arc::new(RwLock::new(camera));
        let c: Arc<RwLock<Camera>> = camera.clone();
        let c: Weak<RwLock<Camera>> = Arc::downgrade(&c);
        self.cameras.insert(id, c);
        camera
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Base {
    pub obj_base: ObjectBase,
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

impl Base {
    pub fn new(eng: &Arc<RwLock<Engine>>) -> Self {
        Self::new_with_obj_base(eng, ObjectBase::new())
    }

    pub fn new_with_id(eng: &Arc<RwLock<Engine>>, id: Id) -> Self {
        Self::new_with_obj_base(eng, ObjectBase::new_with_id(id))
    }

    pub fn new_with_obj_base(eng: &Arc<RwLock<Engine>>, obj_base: ObjectBase) -> Self {
        let identity = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let eng = vxresult!(eng.read());
        let eng = vxunwrap!(eng.os_app.upgrade());
        let os_app = vxresult!(eng.read());
        Base {
            obj_base,
            near: 1.0,
            far: 100.0,
            aspect_ratio: os_app.get_window_aspect_ratio(),
            x: math::Vector3::new(1.0, 0.0, 0.0),
            y: math::Vector3::new(0.0, 1.0, 0.0),
            z: math::Vector3::new(0.0, 0.0, -1.0),
            location: math::Vector3::new(0.0, 0.0, 0.0),
            direction: identity,
            view: identity,
            projection: identity,
            view_projection: math::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
            ),
        }
    }

    pub fn update_view_projection(&mut self) {
        self.view_projection = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        ) * self.projection
            * self.view;
    }

    pub fn update_location(&mut self) {
        let translate = math::Matrix4::from_translation(-self.location);
        self.view = self.direction * translate;
        self.update_view_projection();
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
        vxunimplemented!(); // It must update corresponding manager
    }

    fn render(&self, _: &Engine) {
        vxlogf!("Base camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn update(&mut self) {}
}

impl Transferable for Base {
    fn set_orientation(&mut self, q: &math::Quaternion<f32>) {
        let rotation = math::Matrix4::from(*q);
        self.x = (rotation * self.x.extend(1.0)).truncate();
        self.y = (rotation * self.y.extend(1.0)).truncate();
        self.z = (rotation * self.z.extend(1.0)).truncate();
        let mut q = *q;
        q.s = -q.s;
        let rotation = math::Matrix4::from(q);
        let translate = math::Matrix4::from_translation(-self.location);
        self.direction = rotation;
        self.view = rotation * translate;
        self.update_view_projection();
    }

    fn set_location(&mut self, l: &math::Vector3<f32>) {
        self.location = *l;
        self.update_location();
    }

    fn move_local_z(&mut self, v: f32) {
        self.location = self.location + self.z * v;
        self.update_location();
    }

    fn move_local_x(&mut self, v: f32) {
        self.location = self.location + self.x * v;
        self.update_location();
    }

    fn rotate_local_x(&mut self, v: f32) {
        let rot = math::Matrix4::from_axis_angle(self.x, math::Rad(-v));
        let irot = math::Matrix4::from_axis_angle(self.x, math::Rad(v));
        self.y = (irot * self.y.extend(0.0)).truncate();
        self.z = (irot * self.z.extend(0.0)).truncate();
        self.direction = self.direction * rot;
        self.update_location();
    }

    fn rotate_global_z(&mut self, v: f32) {
        let ax = math::Vector3::new(0.0, 0.0, 1.0);
        let rot = math::Matrix4::from_axis_angle(ax, math::Rad(-v));
        let irot = math::Matrix4::from_axis_angle(ax, math::Rad(v));
        self.x = (irot * self.x.extend(0.0)).truncate();
        self.y = (irot * self.y.extend(0.0)).truncate();
        self.z = (irot * self.z.extend(0.0)).truncate();
        self.direction = self.direction * rot;
        self.update_location();
    }
}

impl Loadable for Base {
    fn new_with_gltf(node: &gltf::Node, eng: &Arc<RwLock<Engine>>, _: &[u8]) -> Self {
        let (near, far) = match vxunwrap!(node.camera()).projection() {
            gltf::camera::Projection::Perspective(p) => (p.znear(), vxunwrap!(p.zfar())),
            gltf::camera::Projection::Orthographic(p) => (p.znear(), p.zfar()),
        };
        let mut myself = Base::new(eng);
        myself.near = near;
        myself.near = far;
        let (l, r, _) = node.transform().decomposed();
        myself.location = math::Vector3::new(l[0], l[1], l[2]);
        let rotation = math::Quaternion::new(r[3], r[0], r[1], r[2]);
        myself.set_orientation(&rotation);
        return myself;
    }

    fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let mut myself = Base::new_with_id(engine, my_id);
        myself.location = math::Vector3::new(reader.read(), reader.read(), reader.read());
        let r = math::Quaternion::new(reader.read(), reader.read(), reader.read(), reader.read());
        myself.near = reader.read();
        myself.far = reader.read();
        myself.set_orientation(&r);
        return myself;
    }
}

impl Camera for Base {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        &self.view_projection
    }

    fn get_cascaded_shadow_points(&self, _: usize) -> Vec<math::Vector3<f32>> {
        vxlogf!("Base camera does not implement cascading.");
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Perspective {
    pub base: Base,
    pub fov_vertical: f32,
    pub fov_horizontal: f32,
    pub tan_vertical: f32,
    pub tan_horizontal: f32,
    pub div_cos_vertical: f32,
    pub div_cos_horizontal: f32,
}

impl Perspective {
    pub fn new(eng: &Arc<RwLock<Engine>>) -> Self {
        let base = Base::new_with_obj_base(eng, ObjectBase::new());
        let mut myself = Self::new_with_base(base);
        myself.set_fov_vertical(1.0);
        return myself;
    }

    pub fn new_with_base(base: Base) -> Self {
        Perspective {
            base,
            fov_vertical: 0.785398163,
            fov_horizontal: 0.785398163,
            tan_vertical: 1.0,
            tan_horizontal: 1.0,
            div_cos_vertical: 0.707106781,
            div_cos_horizontal: 0.707106781,
        }
    }

    pub fn set_fov_vertical(&mut self, fov_vertical: f32) {
        self.fov_vertical = fov_vertical;
        self.tan_vertical = (fov_vertical / 2.0).tan();
        self.tan_horizontal = self.tan_vertical * self.base.aspect_ratio;
        self.fov_horizontal = self.tan_horizontal.atan() * 2.0;
        self.div_cos_vertical = (self.tan_vertical * self.tan_vertical + 1.0).sqrt();
        self.div_cos_horizontal = (self.tan_horizontal * self.tan_horizontal + 1.0).sqrt();
        self.base.projection = math::perspective(
            math::Rad(fov_vertical),
            self.base.aspect_ratio,
            self.base.near,
            self.base.far,
        );
        self.base.update_view_projection();
    }
}

impl CoreObject for Perspective {
    fn get_id(&self) -> Id {
        self.base.get_id()
    }
}

impl Object for Perspective {
    fn get_name(&self) -> Option<String> {
        self.base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn render(&self, _: &Engine) {
        vxlogf!("Perspective camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn update(&mut self) {
        self.base.update();
    }
}

impl Loadable for Perspective {
    fn new_with_gltf(n: &gltf::Node, eng: &Arc<RwLock<Engine>>, data: &[u8]) -> Self {
        let c = vxunwrap!(n.camera());
        let p = match c.projection() {
            gltf::camera::Projection::Perspective(p) => p,
            gltf::camera::Projection::Orthographic(_) => {
                vxlogf!("Type of camera isn't perspective.")
            }
        };
        let base = Base::new_with_gltf(n, eng, data);
        let mut myself = Self::new_with_base(base);
        myself.set_fov_vertical(p.yfov());
        return myself;
    }

    fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let base = Base::new_with_gx3d(engine, reader, my_id);
        let mut myself = Self::new_with_base(base);
        myself.set_fov_vertical(reader.read());
        return myself;
    }
}

impl Transferable for Perspective {
    fn set_orientation(&mut self, q: &math::Quaternion<f32>) {
        self.base.set_orientation(q);
    }

    fn set_location(&mut self, l: &math::Vector3<f32>) {
        self.base.set_location(l);
    }

    fn move_local_z(&mut self, v: f32) {
        self.base.move_local_z(v);
    }

    fn move_local_x(&mut self, v: f32) {
        self.base.move_local_x(v);
    }

    fn rotate_local_x(&mut self, v: f32) {
        self.base.rotate_local_x(v);
    }

    fn rotate_global_z(&mut self, v: f32) {
        self.base.rotate_global_z(v);
    }
}

impl Camera for Perspective {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        self.base.get_view_projection()
    }

    fn get_cascaded_shadow_points(&self, sections_count: usize) -> Vec<math::Vector3<f32>> {
        #[cfg(debug_assertions)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }
        let mut result = vec![math::Vector3::new(0.0f32, 0.0f32, 0.0f32); sections_count + 1];
        result[0] = self.base.location + self.base.z * self.base.near;
        if sections_count > 1 {
            let oneminlambda = 0.5 / self.div_cos_horizontal + 0.5 / self.div_cos_horizontal;
            let lambda = 1.0 - oneminlambda;
            let onedivcn = 1.0 / sections_count as f32;
            let unisecinc = oneminlambda * onedivcn * (self.base.far - self.base.near);
            let fdivn = self.base.far / self.base.near;
            let logsecmul = fdivn.powf(onedivcn);
            let mut unisec = oneminlambda * self.base.near + unisecinc;
            let mut logsec = lambda * self.base.near * logsecmul;
            result[1] = self.base.location + self.base.z * (logsec + unisec);
            for i in 2..sections_count {
                logsec *= logsecmul;
                unisec += unisecinc;
                result[i] = self.base.location + self.base.z * (logsec + unisec);
            }
        }
        result[sections_count] = self.base.location + self.base.z * self.base.far;
        return result;
    }
}

impl DefaultCamera for Perspective {
    fn default(eng: &Arc<RwLock<Engine>>) -> Self {
        Perspective::new(eng)
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Orthographic {
    pub base: Base,
    pub size: f32,
}

impl Orthographic {
    pub fn new(eng: &Arc<RwLock<Engine>>, size: f32) -> Self {
        Self::new_with_base(Base::new(eng), size)
    }

    pub fn new_with_base(mut base: Base, size: f32) -> Self {
        let size = size * 0.5;
        let w = base.aspect_ratio * size;
        base.projection = math::ortho(-w, w, -size, size, base.near, base.far);
        base.update_view_projection();
        Orthographic { base, size }
    }

    pub fn new_with_id(eng: &Arc<RwLock<Engine>>, id: Id) -> Self {
        let mut base = Base::new_with_id(eng, id);
        let size = 0.5;
        let w = base.aspect_ratio * size;
        base.projection = math::ortho(-w, w, -size, size, base.near, base.far);
        base.update_view_projection();
        Orthographic { base, size }
    }
}

impl CoreObject for Orthographic {
    fn get_id(&self) -> Id {
        self.base.get_id()
    }
}

impl Object for Orthographic {
    fn get_name(&self) -> Option<String> {
        self.base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
    }

    fn render(&self, _: &Engine) {
        vxlogf!("Orthographic camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn update(&mut self) {
        self.base.update();
    }
}

impl Loadable for Orthographic {
    fn new_with_gltf(n: &gltf::Node, eng: &Arc<RwLock<Engine>>, data: &[u8]) -> Self {
        let c = vxunwrap!(n.camera());
        let o = match c.projection() {
            gltf::camera::Projection::Perspective(_) => {
                vxlogf!("Type of camera isn't perspective.")
            }
            gltf::camera::Projection::Orthographic(o) => o,
        };
        let base = Base::new_with_gltf(n, eng, data);
        Self::new_with_base(base, o.ymag())
    }

    fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let base = Base::new_with_gx3d(engine, reader, my_id);
        Self::new_with_base(base, reader.read())
    }
}

impl Transferable for Orthographic {
    fn set_orientation(&mut self, q: &math::Quaternion<f32>) {
        self.base.set_orientation(q);
    }

    fn set_location(&mut self, l: &math::Vector3<f32>) {
        self.base.set_location(l);
    }

    fn move_local_z(&mut self, v: f32) {
        self.base.move_local_z(v);
    }

    fn move_local_x(&mut self, v: f32) {
        self.base.move_local_x(v);
    }

    fn rotate_local_x(&mut self, v: f32) {
        self.base.rotate_local_x(v);
    }

    fn rotate_global_z(&mut self, v: f32) {
        self.base.rotate_global_z(v);
    }
}

impl Camera for Orthographic {
    fn get_view_projection(&self) -> &math::Matrix4<f32> {
        &self.base.view_projection
    }

    fn get_cascaded_shadow_points(&self, sections_count: usize) -> Vec<math::Vector3<f32>> {
        #[cfg(debug_assertions)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }
        let mut result = vec![math::Vector3::new(0.0f32, 0.0f32, 0.0f32); sections_count + 1];
        let mut previous = self.base.location + self.base.z * self.base.near;
        result[0] = previous;
        let unisecinc = (self.base.far - self.base.near) / sections_count as f32;
        let unisecinc = self.base.z * unisecinc;
        let sections_count = sections_count + 1;
        for i in 1..sections_count {
            previous += unisecinc;
            result[i] = previous;
        }
        return result;
    }
}

impl DefaultCamera for Orthographic {
    fn default(eng: &Arc<RwLock<Engine>>) -> Self {
        Orthographic::new(eng, 1.0)
    }
}
