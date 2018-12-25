use super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::engine::Engine;
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use cgmath;
use cgmath::prelude::*;
use gltf;
use std::collections::BTreeMap;
use std::convert::From;
use std::sync::{Arc, RwLock, Weak};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    x: cgmath::Vector4<Real>,
    y: cgmath::Vector4<Real>,
    z: cgmath::Vector4<Real>,
    position_far: cgmath::Vector4<Real>, // far is negative
    near_aspect_ratio_reserved: cgmath::Vector4<Real>, // near is negative
    inversed_rotation: cgmath::Matrix4<Real>,
    view: cgmath::Matrix4<Real>,
    projection: cgmath::Matrix4<Real>,
    uniform_projection: cgmath::Matrix4<Real>,
    view_projection: cgmath::Matrix4<Real>,
    uniform_view_projection: cgmath::Matrix4<Real>,
}

impl Uniform {
    pub fn new() -> Self {
        let x = cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0);
        let y = cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0);
        let z = cgmath::Vector4::new(0.0, 0.0, 1.0, 0.0);
        let position_far = cgmath::Vector4::new(0.0, 0.0, 0.0, -100.0);
        let near_aspect_ratio_reserved = cgmath::Vector4::new(-1.0, 0.0, 0.0, -100.0);
        let view = cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let projection = cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        );
        let uniform_projection = cgmath::Matrix4::new(
            0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.0, 1.0,
        ) * projection;
        Uniform {
            x,
            y,
            z,
            position_far,
            near_aspect_ratio_reserved,
            inversed_rotation: view,
            view,
            projection,
            uniform_projection,
            view_projection: projection,
            uniform_view_projection: uniform_projection,
        }
    }
}

pub trait Camera: Object + Transferable {
    fn get_view_projection(&self) -> &cgmath::Matrix4<Real>;
    fn get_cascaded_shadow_frustum_partitions(&self) -> Vec<[cgmath::Vector3<Real>; 4]>;
    fn is_in_frustum(&self, Real, &cgmath::Vector3<Real>) -> bool;
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
    engine: Option<Weak<RwLock<Engine>>>,
    cameras: BTreeMap<Id, Weak<RwLock<Camera>>>,
    name_to_id: BTreeMap<String, Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        let cameras = BTreeMap::new();
        let name_to_id = BTreeMap::new();
        Manager {
            engine: None,
            cameras,
            name_to_id,
            gx3d_table: None,
        }
    }

    pub(crate) fn set_engine(&mut self, e: Weak<RwLock<Engine>>) {
        self.engine = Some(e);
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
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
        let reader: &mut Gx3DReader = table.get_mut_reader();
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

    pub fn create<C>(&mut self) -> Arc<RwLock<C>>
    where
        C: 'static + DefaultCamera,
    {
        let eng = vxunwrap!(vxunwrap!(&self.engine).upgrade());
        let eng = vxresult!(eng.read());
        let camera = C::default(&*eng);
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
    n: cgmath::Vector3<Real>,
    p: cgmath::Vector3<Real>,
    d: Real,
}

enum PlaneIntersectStatue {
    Above,
    Intersecting,
    Under,
}

impl Plane {
    fn new(p: cgmath::Vector3<Real>, f: cgmath::Vector3<Real>, s: cgmath::Vector3<Real>) -> Self {
        let pf = f - p;
        let ps = s - p;
        let n = pf.cross(ps).normalize();
        let d = -(n.dot(p));
        Self { n, p, d }
    }

    fn new_with_point_normal(p: cgmath::Vector3<Real>, n: cgmath::Vector3<Real>) -> Self {
        let n = n.normalize();
        let d = -(n.dot(p));
        Self { n, p, d }
    }

    fn intersect_sphere(
        &self,
        radius: Real,
        center: &cgmath::Vector3<Real>,
    ) -> PlaneIntersectStatue {
        let dis = self.n.dot(*center) + self.d;
        if radius <= dis {
            return PlaneIntersectStatue::Above;
        }
        if radius <= -dis {
            return PlaneIntersectStatue::Under;
        }
        return PlaneIntersectStatue::Intersecting;
    }

    fn translate(&mut self, l: &cgmath::Vector3<Real>) {
        self.p += *l;
        self.d = -(self.n.dot(self.p));
    }

    fn rotate_around(&mut self, l: &cgmath::Vector3<Real>, m: &cgmath::Matrix4<Real>) {
        let mut lp = self.p - l;
        lp = (m * lp.extend(1.0)).truncate();
        self.n = (m * self.n.extend(0.0)).truncate().normalize();
        self.p = lp + l;
        self.d = -(self.n.dot(self.p));
    }

    fn _transform(&mut self, m: &cgmath::Matrix4<Real>) {
        self.p = (m * self.p.extend(1.0)).truncate();
        self.n = (m * self.n.extend(0.0)).truncate().normalize();
        self.d = -(self.n.dot(self.p));
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            n: cgmath::Vector3::new(0.0, 0.0, 1.0),
            p: cgmath::Vector3::new(0.0, 0.0, 0.0),
            d: 0.0,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    uniform: Uniform,
    frustum_planes: [Plane; 6],
    cascades_count: usize,
}

impl Base {
    pub fn new(eng: &Engine) -> Self {
        Self::new_with_obj_base(eng, ObjectBase::new())
    }

    pub fn new_with_id(eng: &Engine, id: Id) -> Self {
        Self::new_with_obj_base(eng, ObjectBase::new_with_id(id))
    }

    pub fn new_with_obj_base(eng: &Engine, obj_base: ObjectBase) -> Self {
        let os_app = vxunwrap!(eng.get_os_app().upgrade());
        let os_app = vxresult!(os_app.read());
        let mut uniform = Uniform::new();
        uniform.near_aspect_ratio_reserved.y = os_app.get_window_aspect_ratio();
        Self {
            obj_base,
            uniform,
            frustum_planes: [
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
            ],
            cascades_count: eng.get_config().get_cascaded_shadows_count() as usize,
        }
    }

    pub fn update_view_projection(&mut self) {
        self.uniform.view_projection = self.uniform.projection * self.uniform.view;
        self.uniform.uniform_view_projection = cgmath::Matrix4::new(
            0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.0, 1.0,
        ) * self.uniform.view_projection;
    }

    pub fn update_location(&mut self) {
        let translate = cgmath::Matrix4::from_translation(-self.uniform.position_far.truncate());
        self.uniform.view = self.uniform.inversed_rotation * translate;
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

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.obj_base.is_rendarable();
    }
}

impl Transferable for Base {
    fn set_orientation(&mut self, q: &cgmath::Quaternion<Real>) {
        let rotation = cgmath::Matrix4::from(*q);
        self.uniform.x = (rotation * cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0))
            .truncate()
            .extend(self.uniform.x.w);
        self.uniform.y = (rotation * cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0))
            .truncate()
            .extend(self.uniform.y.w);
        self.uniform.z = (rotation * cgmath::Vector4::new(0.0, 0.0, 1.0, 0.0))
            .truncate()
            .extend(self.uniform.z.w);
        for fp in &mut self.frustum_planes {
            fp.rotate_around(&self.uniform.position_far.truncate(), &rotation);
        }
        let mut q = -*q;
        q.s = -q.s;
        self.uniform.inversed_rotation = cgmath::Matrix4::from(q);
        self.update_location();
    }

    fn set_location(&mut self, l: &cgmath::Vector3<Real>) {
        let t = l - self.uniform.position_far.truncate();
        for fp in &mut self.frustum_planes {
            fp.translate(&t);
        }
        self.uniform.position_far = l.extend(self.uniform.position_far.w);
        self.update_location();
    }

    fn get_location(&self) -> cgmath::Vector3<Real> {
        return self.uniform.position_far.truncate();
    }

    fn move_local_z(&mut self, v: Real) {
        let t = self.uniform.z.truncate() * v;
        for fp in &mut self.frustum_planes {
            fp.translate(&t);
        }
        self.uniform.position_far =
            (self.uniform.position_far.truncate() + t).extend(self.uniform.position_far.w);
        self.update_location();
    }

    fn move_local_x(&mut self, v: Real) {
        let t = self.uniform.x.truncate() * v;
        for fp in &mut self.frustum_planes {
            fp.translate(&t);
        }
        self.uniform.position_far =
            (self.uniform.position_far.truncate() + t).extend(self.uniform.position_far.w);
        self.update_location();
    }

    fn rotate_local_x(&mut self, v: Real) {
        let rot = cgmath::Matrix4::from_axis_angle(self.uniform.x.truncate(), cgmath::Rad(-v));
        let irot = cgmath::Matrix4::from_axis_angle(self.uniform.x.truncate(), cgmath::Rad(v));
        self.uniform.y = (irot * self.uniform.y.truncate().extend(0.0))
            .truncate()
            .extend(self.uniform.y.w);
        self.uniform.z = (irot * self.uniform.z.truncate().extend(0.0))
            .truncate()
            .extend(self.uniform.z.w);
        for fp in &mut self.frustum_planes {
            fp.rotate_around(&self.uniform.position_far.truncate(), &irot);
        }
        self.uniform.inversed_rotation = self.uniform.inversed_rotation * rot;
        self.update_location();
    }

    fn rotate_global_z(&mut self, v: Real) {
        let ax = cgmath::Vector3::new(0.0, 0.0, 1.0);
        let rot = cgmath::Matrix4::from_axis_angle(ax, cgmath::Rad(-v));
        let irot = cgmath::Matrix4::from_axis_angle(ax, cgmath::Rad(v));
        self.uniform.x = (irot * self.uniform.x.truncate().extend(0.0))
            .truncate()
            .extend(self.uniform.x.w);
        self.uniform.y = (irot * self.uniform.y.truncate().extend(0.0))
            .truncate()
            .extend(self.uniform.y.w);
        self.uniform.z = (irot * self.uniform.z.truncate().extend(0.0))
            .truncate()
            .extend(self.uniform.z.w);
        for fp in &mut self.frustum_planes {
            fp.rotate_around(&self.uniform.position_far.truncate(), &irot);
        }
        self.uniform.inversed_rotation = self.uniform.inversed_rotation * rot;
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
        myself.uniform.near_aspect_ratio_reserved.x = near;
        let (l, r, _) = node.transform().decomposed();
        myself.uniform.position_far = cgmath::Vector4::new(l[0], l[1], l[2], -far);
        let rotation = cgmath::Quaternion::new(r[3], r[0], r[1], r[2]);
        myself.set_orientation(&rotation);
        return myself;
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let mut myself = Base::new_with_id(engine, my_id);
        myself.uniform.position_far.x = reader.read();
        myself.uniform.position_far.y = reader.read();
        myself.uniform.position_far.z = reader.read();
        let r: [Real; 4] = [reader.read(), reader.read(), reader.read(), reader.read()];
        myself.uniform.near_aspect_ratio_reserved.x = -reader.read::<Real>();
        myself.uniform.position_far.w = -reader.read::<Real>();
        myself.set_orientation(&cgmath::Quaternion::new(r[0], r[1], r[2], r[3]));
        #[cfg(debug_gx3d)]
        vxlogi!("Camera position is: {:?}", &myself.uniform.position_far);
        #[cfg(debug_gx3d)]
        vxlogi!("Camera quaternion is: {:?}", &r);
        #[cfg(debug_gx3d)]
        vxlogi!("Camera view matrix is: {:?}", &myself.uniform.view);

        return myself;
    }
}

impl Camera for Base {
    fn get_view_projection(&self) -> &cgmath::Matrix4<Real> {
        &self.uniform.view_projection
    }

    fn get_cascaded_shadow_frustum_partitions(&self) -> Vec<[cgmath::Vector3<Real>; 4]> {
        vxlogf!("Base camera does not implement cascading.");
    }

    fn is_in_frustum(&self, radius: Real, location: &cgmath::Vector3<Real>) -> bool {
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
        *uniform = self.uniform;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Perspective {
    pub base: Base,
    pub fovx: Real,
    pub fovy: Real,
    pub tanx: Real,
    pub tany: Real,
    lambda: Real, // (sin(fovy/2) + sin(fovx/2)) / 2 -no proof for it just my gut, let see what I get
}

const DEFAULT_FOVX: Real = 1.396263402; // 80 degree

impl Perspective {
    pub fn new(eng: &Engine) -> Self {
        let base = Base::new_with_obj_base(eng, ObjectBase::new());
        let mut myself = Self::new_with_base(base);
        myself.set_fov_vertical(DEFAULT_FOVX);
        return myself;
    }

    fn new_with_base(base: Base) -> Self {
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
        self.tany = self.tanx / self.base.uniform.near_aspect_ratio_reserved.y;
        self.fovy = self.tany.atan() * 2.0;
        self.base.uniform.projection = cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        ) * cgmath::perspective(
            cgmath::Rad(self.fovy),
            self.base.uniform.near_aspect_ratio_reserved.y,
            -self.base.uniform.near_aspect_ratio_reserved.x,
            -self.base.uniform.position_far.w,
        );
        self.base.uniform.uniform_projection = cgmath::Matrix4::new(
            0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.0, 1.0,
        ) * self.base.uniform.projection;
        self.lambda = ((self.fovx * 0.5).sin() + (self.fovy * 0.5).sin()) * 0.5;
        self.base.update_view_projection();
        self.update_frustum_planes();
    }

    fn update_frustum_planes(&mut self) {
        let zn = self.base.uniform.position_far.truncate()
            + (self.base.uniform.z.truncate() * self.base.uniform.near_aspect_ratio_reserved.x);
        let zf = self.base.uniform.position_far.truncate()
            + (self.base.uniform.z.truncate() * self.base.uniform.position_far.w);
        let xn = self.base.uniform.x.truncate()
            * (-self.base.uniform.near_aspect_ratio_reserved.x * self.tanx);
        let yn = self.base.uniform.y.truncate()
            * (-self.base.uniform.near_aspect_ratio_reserved.x * self.tany);
        let xf = self.base.uniform.x.truncate() * (-self.base.uniform.position_far.w * self.tanx);
        let yf = self.base.uniform.y.truncate() * (-self.base.uniform.position_far.w * self.tany);
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

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.base.is_rendarable();
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
        #[cfg(debug_gx3d)]
        vxlogi!("{:?}", &myself.base.uniform);
        return myself;
    }
}

impl Transferable for Perspective {
    fn set_orientation(&mut self, q: &cgmath::Quaternion<Real>) {
        self.base.set_orientation(q);
    }

    fn set_location(&mut self, l: &cgmath::Vector3<Real>) {
        self.base.set_location(l);
    }

    fn get_location(&self) -> cgmath::Vector3<Real> {
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
    fn get_view_projection(&self) -> &cgmath::Matrix4<Real> {
        self.base.get_view_projection()
    }

    fn get_cascaded_shadow_frustum_partitions(&self) -> Vec<[cgmath::Vector3<Real>; 4]> {
        let sections_count = self.base.cascades_count;
        #[cfg(debug_mode)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }

        let mut result = vec![[cgmath::Vector3::new(0.0, 0.0, 0.0); 4]; sections_count + 1];

        let xtanx = self.base.uniform.x.truncate() * self.tanx;
        let ytany = self.base.uniform.y.truncate() * self.tany;

        let x = xtanx * -self.base.uniform.near_aspect_ratio_reserved.x;
        let y = ytany * -self.base.uniform.near_aspect_ratio_reserved.x;
        let z = self.base.uniform.position_far.truncate()
            + (self.base.uniform.z.truncate() * self.base.uniform.near_aspect_ratio_reserved.x);

        result[0][0] = (z - x) - y;
        result[0][1] = z + x - y;
        result[0][2] = z + x + y;
        result[0][3] = (z - x) + y;

        let x = xtanx * -self.base.uniform.position_far.w;
        let y = ytany * -self.base.uniform.position_far.w;
        let z = self.base.uniform.position_far.truncate()
            + (self.base.uniform.z.truncate() * self.base.uniform.position_far.w);

        result[sections_count][0] = (z - x) - y;
        result[sections_count][1] = z + x - y;
        result[sections_count][2] = z + x + y;
        result[sections_count][3] = (z - x) + y;

        if sections_count < 2 {
            return result;
        }
        // Zi = yn(f/n)^(i/N) + (1-y)(n+(i/N)(f-n))
        // Zi = yn((f/n)^(1/N))^i + (1-y)n + (1-y)((f-n)/N)i
        let oneminlambda = 1.0 - self.lambda;
        let lambda = self.lambda;
        let onedivcn = 1.0 / sections_count as Real;
        // uniform increament
        let unisecinc = oneminlambda
            * onedivcn
            * (self.base.uniform.position_far.w - self.base.uniform.near_aspect_ratio_reserved.x);
        let fdivn =
            self.base.uniform.position_far.w / self.base.uniform.near_aspect_ratio_reserved.x;
        // logarithmic multiplication
        let logsecmul = fdivn.powf(onedivcn);
        // uniform sector
        let mut unisec = oneminlambda * self.base.uniform.near_aspect_ratio_reserved.x + unisecinc;
        // logarithmic sector
        let mut logsec = lambda * self.base.uniform.near_aspect_ratio_reserved.x * logsecmul;

        let l = logsec + unisec;
        let x = xtanx * l;
        let y = ytany * l;
        let z = self.base.uniform.position_far.truncate() + (self.base.uniform.z.truncate() * l);

        result[1][0] = (z - x) - y;
        result[1][1] = z + x - y;
        result[1][2] = z + x + y;
        result[1][3] = (z - x) + y;

        for i in 2..sections_count {
            logsec *= logsecmul;
            unisec += unisecinc;

            let l = logsec + unisec;
            let x = xtanx * l;
            let y = ytany * l;
            let z =
                self.base.uniform.position_far.truncate() + (self.base.uniform.z.truncate() * l);

            result[i][0] = (z - x) - y;
            result[i][1] = z + x - y;
            result[i][2] = z + x + y;
            result[i][3] = (z - x) + y;
        }
        return result;
    }

    fn is_in_frustum(&self, radius: Real, location: &cgmath::Vector3<Real>) -> bool {
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
        let zn = self.base.uniform.position_far.truncate()
            + (self.base.uniform.z.truncate() * self.base.uniform.near_aspect_ratio_reserved.x);
        let zf = self.base.uniform.position_far.truncate()
            + (self.base.uniform.z.truncate() * self.base.uniform.position_far.w);
        let x = self.base.uniform.x.truncate()
            * (self.base.uniform.near_aspect_ratio_reserved.y * self.size);
        let y = self.base.uniform.y.truncate() * self.size;
        let xpy = x + y;
        let np = zn + xpy;
        let fp = zf - xpy;
        self.base.frustum_planes[0] =
            Plane::new_with_point_normal(np, -self.base.uniform.z.truncate());
        self.base.frustum_planes[1] =
            Plane::new_with_point_normal(np, self.base.uniform.x.truncate());
        self.base.frustum_planes[2] =
            Plane::new_with_point_normal(np, self.base.uniform.y.truncate());
        self.base.frustum_planes[3] =
            Plane::new_with_point_normal(fp, -self.base.uniform.x.truncate());
        self.base.frustum_planes[4] =
            Plane::new_with_point_normal(fp, -self.base.uniform.y.truncate());
        self.base.frustum_planes[5] =
            Plane::new_with_point_normal(fp, self.base.uniform.z.truncate());
    }

    pub fn new_with_base(mut base: Base, size: Real) -> Self {
        let size = size * 0.5;
        let w = base.uniform.near_aspect_ratio_reserved.y * size;
        base.uniform.projection = cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        ) * cgmath::ortho(
            -w,
            w,
            -size,
            size,
            -base.uniform.near_aspect_ratio_reserved.x,
            -base.uniform.position_far.w,
        );
        base.uniform.uniform_projection = cgmath::Matrix4::new(
            0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.0, 1.0,
        ) * base.uniform.projection;
        base.update_view_projection();
        let mut s = Orthographic { base, size };
        s.update_frustum_planes();
        return s;
    }

    pub fn new_with_id(eng: &Engine, id: Id) -> Self {
        let mut base = Base::new_with_id(eng, id);
        let size = 0.5;
        let w = base.uniform.near_aspect_ratio_reserved.y * size;
        base.uniform.projection = cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
        ) * cgmath::ortho(
            -w,
            w,
            -size,
            size,
            -base.uniform.near_aspect_ratio_reserved.x,
            -base.uniform.position_far.w,
        );
        base.uniform.uniform_projection = cgmath::Matrix4::new(
            0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.0, 1.0,
        ) * base.uniform.projection;
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

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.base.is_rendarable();
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
    fn set_orientation(&mut self, q: &cgmath::Quaternion<Real>) {
        self.base.set_orientation(q);
    }

    fn set_location(&mut self, l: &cgmath::Vector3<Real>) {
        self.base.set_location(l);
    }

    fn get_location(&self) -> cgmath::Vector3<Real> {
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
    fn get_view_projection(&self) -> &cgmath::Matrix4<Real> {
        &self.base.uniform.view_projection
    }

    fn get_cascaded_shadow_frustum_partitions(&self) -> Vec<[cgmath::Vector3<Real>; 4]> {
        let sections_count = self.base.cascades_count;
        #[cfg(debug_mode)]
        {
            if sections_count < 1 {
                vxlogf!("sections_count must be greater than zero.");
            }
        }
        let mut result = vec![[cgmath::Vector3::new(0.0, 0.0, 0.0); 4]; sections_count + 1];

        let w = self.size * self.base.uniform.near_aspect_ratio_reserved.y;

        let mut l = -self.base.uniform.near_aspect_ratio_reserved.x;
        let x = self.base.uniform.x.truncate() * w;
        let y = self.base.uniform.y.truncate() * self.size;
        let z = self.base.uniform.position_far.truncate() + (self.base.uniform.z.truncate() * l);

        result[0][0] = z - x - y;
        result[0][1] = z + x - y;
        result[0][2] = z + x + y;
        result[0][3] = z - x + y;

        let unisecinc = (-self.base.uniform.position_far.w
            + self.base.uniform.near_aspect_ratio_reserved.x)
            / sections_count as Real;

        let sections_count = sections_count + 1;

        for i in 1..sections_count {
            l += unisecinc;

            let x = self.base.uniform.x.truncate() * w;
            let y = self.base.uniform.y.truncate() * self.size;
            let z =
                self.base.uniform.position_far.truncate() + (self.base.uniform.z.truncate() * l);

            result[i][0] = z - x - y;
            result[i][1] = z + x - y;
            result[i][2] = z + x + y;
            result[i][3] = z - x + y;
        }
        return result;
    }

    fn is_in_frustum(&self, radius: Real, location: &cgmath::Vector3<Real>) -> bool {
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
