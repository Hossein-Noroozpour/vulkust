use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::command::Buffer as CmdBuffer;
use super::engine::Engine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use gltf;
use math;
use math::prelude::*;
use std::collections::BTreeMap;
use std::convert::From;
use std::sync::{Arc, RwLock, Weak};

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    pub position_radius: math::Vector4<Real>,
    pub projection: math::Matrix4<Real>,
    pub view: math::Matrix4<Real>,
    pub view_projection: math::Matrix4<Real>,
}

impl Uniform {
    pub fn new() -> Self {
        let view = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let projection = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        );
        let position_radius = math::Vector4::new(0.0, 0.0, 0.0, 0.0);
        Uniform {
            position_radius,
            projection,
            view,
            view_projection: projection,
        }
    }
}

pub trait Camera: Object + Transferable {
    fn get_view_projection(&self) -> &math::Matrix4<Real>;
    fn get_cascaded_shadow_frustum_partitions(&self, usize) -> Vec<[math::Vector3<Real>; 4]>;
    fn is_in_frustum(&self, Real, &math::Vector3<Real>) -> bool;
    fn update_uniform(&self, &mut Uniform);
}

pub trait DefaultCamera: Camera {
    fn default(&Engine) -> Self;
}

#[cfg_attr(debug_mode, derive(Debug))]
#[repr(u8)]
pub enum TypeId {
    Perspective = 1,
    Orthographic = 2,
}

#[cfg_attr(debug_mode, derive(Debug))]
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

    pub fn load_gltf(&mut self, n: &gltf::Node, eng: &Engine) -> Arc<RwLock<Camera>> {
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
        #[cfg(debug_mode)]
        vxlogi!("Camera is: {:?}", &camera);
        self.cameras.insert(id, Arc::downgrade(&camera));
        if let Some(name) = n.name() {
            let name = name.to_string();
            self.name_to_id.insert(name, id);
        }
        camera
    }

    pub fn load_gx3d(&mut self, engine: &Engine, id: Id) -> Arc<RwLock<Camera>> {
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

    pub fn create<C>(&mut self, eng: &Engine) -> Arc<RwLock<C>>
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

#[cfg_attr(debug_mode, derive(Debug))]
struct Plane {
    n: math::Vector3<Real>,
    p: math::Vector3<Real>,
    d: Real,
}

enum PlaneIntersectStatue {
    Above,
    Intersecting,
    Under,
}

impl Plane {
    fn new(p: math::Vector3<Real>, f: math::Vector3<Real>, s: math::Vector3<Real>) -> Self {
        let pf = f - p;
        let ps = s - p;
        let n = pf.cross(ps).normalize();
        let d = -(n.dot(p));
        Self { n, p, d }
    }

    fn new_with_point_normal(p: math::Vector3<Real>, n: math::Vector3<Real>) -> Self {
        let n = n.normalize();
        let d = -(n.dot(p));
        Self { n, p, d }
    }

    fn intersect_sphere(&self, radius: Real, center: &math::Vector3<Real>) -> PlaneIntersectStatue {
        let dis = self.n.dot(*center) + self.d;
        if radius <= dis {
            return PlaneIntersectStatue::Above;
        }
        if radius <= -dis {
            return PlaneIntersectStatue::Under;
        }
        return PlaneIntersectStatue::Intersecting;
    }

    fn translate(&mut self, l: &math::Vector3<Real>) {
        self.p += *l;
        self.d = -(self.n.dot(self.p));
    }

    fn rotate_around(&mut self, l: &math::Vector3<Real>, m: &math::Matrix4<Real>) {
        let mut lp = self.p - l;
        lp = (m * lp.extend(1.0)).truncate();
        self.n = (m * self.n.extend(0.0)).truncate().normalize();
        self.p = lp + l;
        self.d = -(self.n.dot(self.p));
    }

    fn _transform(&mut self, m: &math::Matrix4<Real>) {
        self.p = (m * self.p.extend(1.0)).truncate();
        self.n = (m * self.n.extend(0.0)).truncate().normalize();
        self.d = -(self.n.dot(self.p));
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            n: math::Vector3::new(0.0, 0.0, 1.0),
            p: math::Vector3::new(0.0, 0.0, 0.0),
            d: 0.0,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    pub obj_base: ObjectBase,
    pub near: Real,
    pub far: Real,
    pub aspect_ratio: Real,
    pub x: math::Vector3<Real>,
    pub y: math::Vector3<Real>,
    pub z: math::Vector3<Real>,
    pub location: math::Vector3<Real>,
    pub direction: math::Matrix4<Real>,
    pub view: math::Matrix4<Real>,
    pub projection: math::Matrix4<Real>,
    pub view_projection: math::Matrix4<Real>,
    frustum_planes: [Plane; 6],
}

impl Base {
    pub fn new(eng: &Engine) -> Self {
        Self::new_with_obj_base(eng, ObjectBase::new())
    }

    pub fn new_with_id(eng: &Engine, id: Id) -> Self {
        Self::new_with_obj_base(eng, ObjectBase::new_with_id(id))
    }

    pub fn new_with_obj_base(eng: &Engine, obj_base: ObjectBase) -> Self {
        let identity = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let img_transform = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        );
        let os_app = vxunwrap!(eng.os_app.upgrade());
        let os_app = vxresult!(os_app.read());
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
            view_projection: img_transform,
            frustum_planes: [
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
            ],
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

    fn render(&self, _: &mut CmdBuffer, _: usize) {
        vxlogf!("Base camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.obj_base.is_rendarable();
    }

    fn update(&mut self) {}
}

impl Transferable for Base {
    fn set_orientation(&mut self, q: &math::Quaternion<Real>) {
        let rotation = math::Matrix4::from(*q);
        self.x = (rotation * self.x.extend(1.0)).truncate();
        self.y = (rotation * self.y.extend(1.0)).truncate();
        self.z = (rotation * self.z.extend(1.0)).truncate();
        for fp in &mut self.frustum_planes {
            fp.rotate_around(&self.location, &rotation);
        }
        let mut q = *q;
        q.s = -q.s;
        let rotation = math::Matrix4::from(q);
        let translate = math::Matrix4::from_translation(-self.location);
        self.direction = rotation;
        self.view = rotation * translate;
        self.update_view_projection();
    }

    fn set_location(&mut self, l: &math::Vector3<Real>) {
        let t = l - self.location;
        for fp in &mut self.frustum_planes {
            fp.translate(&t);
        }
        self.location = *l;
        self.update_location();
    }

    fn get_location(&self) -> &math::Vector3<Real> {
        return &self.location;
    }

    fn move_local_z(&mut self, v: Real) {
        let t = self.z * v;
        for fp in &mut self.frustum_planes {
            fp.translate(&t);
        }
        self.location = self.location + t;
        self.update_location();
    }

    fn move_local_x(&mut self, v: Real) {
        let t = self.x * v;
        for fp in &mut self.frustum_planes {
            fp.translate(&t);
        }
        self.location = self.location + t;
        self.update_location();
    }

    fn rotate_local_x(&mut self, v: Real) {
        let rot = math::Matrix4::from_axis_angle(self.x, math::Rad(-v));
        let irot = math::Matrix4::from_axis_angle(self.x, math::Rad(v));
        self.y = (irot * self.y.extend(0.0)).truncate();
        self.z = (irot * self.z.extend(0.0)).truncate();
        for fp in &mut self.frustum_planes {
            fp.rotate_around(&self.location, &irot);
        }
        self.direction = self.direction * rot;
        self.update_location();
    }

    fn rotate_global_z(&mut self, v: Real) {
        let ax = math::Vector3::new(0.0, 0.0, 1.0);
        let rot = math::Matrix4::from_axis_angle(ax, math::Rad(-v));
        let irot = math::Matrix4::from_axis_angle(ax, math::Rad(v));
        self.x = (irot * self.x.extend(0.0)).truncate();
        self.y = (irot * self.y.extend(0.0)).truncate();
        self.z = (irot * self.z.extend(0.0)).truncate();
        for fp in &mut self.frustum_planes {
            fp.rotate_around(&self.location, &irot);
        }
        self.direction = self.direction * rot;
        self.update_location();
    }
}

impl Loadable for Base {
    fn new_with_gltf(node: &gltf::Node, eng: &Engine, _: &[u8]) -> Self {
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

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
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
    fn get_view_projection(&self) -> &math::Matrix4<Real> {
        &self.view_projection
    }

    fn get_cascaded_shadow_frustum_partitions(&self, _: usize) -> Vec<[math::Vector3<Real>; 4]> {
        vxlogf!("Base camera does not implement cascading.");
    }

    fn is_in_frustum(&self, radius: Real, location: &math::Vector3<Real>) -> bool {
        for f in &self.frustum_planes {
            let s = f.intersect_sphere(radius, location);
            match s {
                PlaneIntersectStatue::Above => return false,
                _ => (),
            }
        }
        return true;
    }

    fn update_uniform(&self, uniform: &mut Uniform) {
        uniform.position_radius = self.location.extend(self.near); // todo more accurate bounding shere radius
        uniform.projection = self.projection;
        uniform.view = self.view;
        uniform.view_projection = self.view_projection;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Perspective {
    pub base: Base,
    pub fovx: Real,
    pub fovy: Real,
    pub tanx: Real,
    pub tany: Real,
    lambda: Real, // (cos(fovy) + cos(fovx)) / 2 -no proof for it just my gut, let see what I get
}

const DEFAULT_FOVX: Real = 1.396263402; // 80 degree

impl Perspective {
    pub fn new(eng: &Engine) -> Self {
        let base = Base::new_with_obj_base(eng, ObjectBase::new());
        let mut myself = Self::new_with_base(base);
        myself.set_fov_vertical(DEFAULT_FOVX);
        return myself;
    }

    pub fn new_with_base(base: Base) -> Self {
        let mut s = Self {
            base,
            fovy: 0.0,
            fovx: 0.0,
            tanx: 0.0,
            tany: 0.0,
            lambda: 0.0,
        };
        s.set_fov_vertical(DEFAULT_FOVX);
        return s;
    }

    pub fn set_fov_vertical(&mut self, fovx: Real) {
        self.fovx = fovx;
        self.tanx = (fovx * 0.5).tan();
        self.tany = self.tanx / self.base.aspect_ratio;
        self.fovy = self.tany.atan() * 2.0;
        self.base.projection = math::perspective(
            math::Rad(self.fovy),
            self.base.aspect_ratio,
            self.base.near,
            self.base.far,
        );
        self.lambda = (self.fovx + self.fovy) * 0.5;
        self.base.update_view_projection();
        self.update_frustum_planes();
    }

    fn update_frustum_planes(&mut self) {
        let zn = self.base.location + (self.base.z * self.base.near);
        let zf = self.base.location + (self.base.z * self.base.far);
        let xn = self.base.x * (self.base.near * self.tanx);
        let yn = self.base.y * (self.base.near * self.tany);
        let xf = self.base.x * (self.base.far * self.tanx);
        let yf = self.base.y * (self.base.far * self.tany);
        let znpxn = zn + xn;
        let znmxn = zn - xn;
        let np1 = znpxn + yn;
        let np2 = znpxn - yn;
        let np3 = znmxn + yn;
        let np4 = znmxn - yn;
        let zfpxf = zf + xf;
        let zfmxf = zf - xf;
        let fp1 = zfpxf + yf;
        let fp2 = zfpxf - yf;
        let fp3 = zfmxf + yf;
        let fp4 = zfmxf - yf;
        self.base.frustum_planes[0] = Plane::new(np1, np3, np2);
        self.base.frustum_planes[1] = Plane::new(np1, fp1, np3);
        self.base.frustum_planes[2] = Plane::new(np1, np2, fp1);
        self.base.frustum_planes[3] = Plane::new(fp4, fp3, fp2);
        self.base.frustum_planes[4] = Plane::new(fp4, fp2, np4);
        self.base.frustum_planes[5] = Plane::new(fp4, np4, fp3);
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

    fn render(&self, _: &mut CmdBuffer, _: usize) {
        vxlogf!("Perspective camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.base.is_rendarable();
    }

    fn update(&mut self) {
        self.base.update();
    }
}

impl Loadable for Perspective {
    fn new_with_gltf(n: &gltf::Node, eng: &Engine, data: &[u8]) -> Self {
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

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let base = Base::new_with_gx3d(engine, reader, my_id);
        let mut myself = Self::new_with_base(base);
        myself.set_fov_vertical(reader.read());
        return myself;
    }
}

impl Transferable for Perspective {
    fn set_orientation(&mut self, q: &math::Quaternion<Real>) {
        self.base.set_orientation(q);
    }

    fn set_location(&mut self, l: &math::Vector3<Real>) {
        self.base.set_location(l);
    }

    fn get_location(&self) -> &math::Vector3<Real> {
        return self.base.get_location();
    }

    fn move_local_z(&mut self, v: Real) {
        self.base.move_local_z(v);
    }

    fn move_local_x(&mut self, v: Real) {
        self.base.move_local_x(v);
    }

    fn rotate_local_x(&mut self, v: Real) {
        self.base.rotate_local_x(v);
    }

    fn rotate_global_z(&mut self, v: Real) {
        self.base.rotate_global_z(v);
    }
}

impl Camera for Perspective {
    fn get_view_projection(&self) -> &math::Matrix4<Real> {
        self.base.get_view_projection()
    }

    fn get_cascaded_shadow_frustum_partitions(
        &self,
        sections_count: usize,
    ) -> Vec<[math::Vector3<Real>; 4]> {
        #[cfg(debug_mode)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }

        let mut result = vec![[math::Vector3::new(0.0, 0.0, 0.0); 4]; sections_count + 1];

        let x = self.base.x * (self.tanx * self.base.near);
        let y = self.base.y * (self.tany * self.base.near);
        let z = self.base.location + self.base.z * self.base.near;

        result[0][0] = z - x - y;
        result[0][1] = z + x - y;
        result[0][2] = z + x + y;
        result[0][3] = z - x + y;

        let x = self.base.x * (self.tanx * self.base.far);
        let y = self.base.y * (self.tany * self.base.far);
        let z = self.base.location + self.base.z * self.base.far;

        result[sections_count][0] = z - x - y;
        result[sections_count][1] = z + x - y;
        result[sections_count][2] = z + x + y;
        result[sections_count][3] = z - x + y;

        if sections_count < 2 {
            return result;
        }
        // Zi = yn(f/n)^(i/N) + (1-y)(n+(i/N)(f-n))
        // Zi = yn((f/n)^(1/N))^i + (1-y)n + (1-y)((f-n)/N)i
        let oneminlambda = 1.0 - self.lambda;
        let lambda = self.lambda;
        let onedivcn = 1.0 / sections_count as Real;
        // uniform increament
        let unisecinc = oneminlambda * onedivcn * (self.base.far - self.base.near);
        let fdivn = self.base.far / self.base.near;
        // logarithmic multiplication
        let logsecmul = fdivn.powf(onedivcn);
        // uniform sector
        let mut unisec = oneminlambda * self.base.near + unisecinc;
        // logarithmic sector
        let mut logsec = lambda * self.base.near * logsecmul;

        let l = logsec + unisec;
        let x = self.base.x * (self.tanx * l);
        let y = self.base.y * (self.tany * l);
        let z = self.base.location + self.base.z * l;

        result[1][0] = z - x - y;
        result[1][1] = z + x - y;
        result[1][2] = z + x + y;
        result[1][3] = z - x + y;

        for i in 2..sections_count {
            logsec *= logsecmul;
            unisec += unisecinc;

            let l = logsec + unisec;
            let x = self.base.x * (self.tanx * l);
            let y = self.base.y * (self.tany * l);
            let z = self.base.location + self.base.z * l;

            result[i][0] = z - x - y;
            result[i][1] = z + x - y;
            result[i][2] = z + x + y;
            result[i][3] = z - x + y;
        }
        return result;
    }

    fn is_in_frustum(&self, radius: Real, location: &math::Vector3<Real>) -> bool {
        return self.base.is_in_frustum(radius, location);
    }

    fn update_uniform(&self, uniform: &mut Uniform) {
        self.base.update_uniform(uniform);
    }
}

impl DefaultCamera for Perspective {
    fn default(eng: &Engine) -> Self {
        Perspective::new(eng)
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Orthographic {
    pub base: Base,
    pub size: Real,
}

impl Orthographic {
    pub fn new(eng: &Engine, size: Real) -> Self {
        Self::new_with_base(Base::new(eng), size)
    }

    fn update_frustum_planes(&mut self) {
        let zn = self.base.location + (self.base.z * self.base.near);
        let zf = self.base.location + (self.base.z * self.base.far);
        let x = self.base.x * (self.base.aspect_ratio * self.size);
        let y = self.base.y * self.size;
        let xpy = x + y;
        let np = zn + xpy;
        let fp = zf - xpy;
        self.base.frustum_planes[0] = Plane::new_with_point_normal(np, -self.base.z);
        self.base.frustum_planes[1] = Plane::new_with_point_normal(np, self.base.x);
        self.base.frustum_planes[2] = Plane::new_with_point_normal(np, self.base.y);
        self.base.frustum_planes[3] = Plane::new_with_point_normal(fp, -self.base.x);
        self.base.frustum_planes[4] = Plane::new_with_point_normal(fp, -self.base.y);
        self.base.frustum_planes[5] = Plane::new_with_point_normal(fp, self.base.z);
    }

    pub fn new_with_base(mut base: Base, size: Real) -> Self {
        let size = size * 0.5;
        let w = base.aspect_ratio * size;
        base.projection = math::ortho(-w, w, -size, size, base.near, base.far);
        base.update_view_projection();
        let mut s = Orthographic { base, size };
        s.update_frustum_planes();
        return s;
    }

    pub fn new_with_id(eng: &Engine, id: Id) -> Self {
        let mut base = Base::new_with_id(eng, id);
        let size = 0.5;
        let w = base.aspect_ratio * size;
        base.projection = math::ortho(-w, w, -size, size, base.near, base.far);
        base.update_view_projection();
        let mut s = Orthographic { base, size };
        s.update_frustum_planes();
        return s;
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

    fn render(&self, _: &mut CmdBuffer, _: usize) {
        vxlogf!("Orthographic camera does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.base.is_rendarable();
    }

    fn update(&mut self) {
        self.base.update();
    }
}

impl Loadable for Orthographic {
    fn new_with_gltf(n: &gltf::Node, eng: &Engine, data: &[u8]) -> Self {
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

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let base = Base::new_with_gx3d(engine, reader, my_id);
        Self::new_with_base(base, reader.read())
    }
}

impl Transferable for Orthographic {
    fn set_orientation(&mut self, q: &math::Quaternion<Real>) {
        self.base.set_orientation(q);
    }

    fn set_location(&mut self, l: &math::Vector3<Real>) {
        self.base.set_location(l);
    }

    fn get_location(&self) -> &math::Vector3<Real> {
        return self.base.get_location();
    }

    fn move_local_z(&mut self, v: Real) {
        self.base.move_local_z(v);
    }

    fn move_local_x(&mut self, v: Real) {
        self.base.move_local_x(v);
    }

    fn rotate_local_x(&mut self, v: Real) {
        self.base.rotate_local_x(v);
    }

    fn rotate_global_z(&mut self, v: Real) {
        self.base.rotate_global_z(v);
    }
}

impl Camera for Orthographic {
    fn get_view_projection(&self) -> &math::Matrix4<Real> {
        &self.base.view_projection
    }

    fn get_cascaded_shadow_frustum_partitions(
        &self,
        sections_count: usize,
    ) -> Vec<[math::Vector3<Real>; 4]> {
        #[cfg(debug_mode)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }
        let mut result = vec![[math::Vector3::new(0.0, 0.0, 0.0); 4]; sections_count + 1];

        let w = self.size * self.base.aspect_ratio;

        let mut l = self.base.near;
        let x = self.base.x * w;
        let y = self.base.y * self.size;
        let z = self.base.location + self.base.z * l;

        result[0][0] = z - x - y;
        result[0][1] = z + x - y;
        result[0][2] = z + x + y;
        result[0][3] = z - x + y;

        let unisecinc = (self.base.far - self.base.near) / sections_count as Real;

        let sections_count = sections_count + 1;

        for i in 1..sections_count {
            l += unisecinc;

            let x = self.base.x * w;
            let y = self.base.y * self.size;
            let z = self.base.location + self.base.z * l;

            result[i][0] = z - x - y;
            result[i][1] = z + x - y;
            result[i][2] = z + x + y;
            result[i][3] = z - x + y;
        }
        return result;
    }

    fn is_in_frustum(&self, radius: Real, location: &math::Vector3<Real>) -> bool {
        return self.base.is_in_frustum(radius, location);
    }

    fn update_uniform(&self, uniform: &mut Uniform) {
        self.base.update_uniform(uniform);
    }
}

impl DefaultCamera for Orthographic {
    fn default(eng: &Engine) -> Self {
        Orthographic::new(eng, 1.0)
    }
}
