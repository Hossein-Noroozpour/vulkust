use super::super::core::debug::Debug as CoreDebug;
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::camera::Orthographic;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::engine::Engine;
use super::gapi::GraphicApiEngine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::model::Model;
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use super::sync::Semaphore;
use std::collections::BTreeMap;
use std::f32::MAX as F32MAX;
use std::f32::MIN as F32MIN;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use math;

#[cfg_attr(debug_mode, derive(Debug))]
#[repr(u8)]
pub enum TypeId {
    Sun = 1,
}

pub trait ShadowMakerData: CoreDebug + Send {
    fn check_shadowability(&mut self, &mut Model);
    fn to_sun(&self) -> &SunShadowMakerData {
        vxunexpected!();
    }
}

pub trait VisibilityData: CoreDebug + Send {}

pub trait Light: Object {
    fn to_directional(&self) -> Option<&Directional> {
        return None;
    }

    fn to_mut_directional(&mut self) -> Option<&mut Directional> {
        return None;
    }

    fn get_shadow_maker_data(&self) -> Option<Box<ShadowMakerData>> {
        return None;
    }

    fn update_shadow_maker_data(&mut self, _: &Box<ShadowMakerData>) {
        vxunexpected!();
    }
}

pub trait Directional: Light {
    fn update_cascaded_shadow_map_cameras(&mut self, &Vec<[math::Vector3<Real>; 4]>);
}

pub trait DefaultLighting {
    fn default(eng: &Engine) -> Self;
}

#[cfg_attr(debug_mode, derive(Debug))]
struct SunCam {
    vp: math::Matrix4<Real>,
    max_x: Real,
    max_seen_x: Real,
    min_x: Real,
    min_seen_x: Real,
    max_y: Real,
    max_seen_y: Real,
    min_y: Real,
    min_seen_y: Real,
    max_z: Real,
    max_seen_z: Real,
    min_z: Real,
    min_seen_z: Real,
}

impl SunCam {
    fn new() -> Self {
        Self {
            vp: math::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            max_x: F32MIN,
            max_seen_x: F32MIN,
            min_x: F32MAX,
            min_seen_x: F32MAX,
            max_y: F32MIN,
            max_seen_y: F32MIN,
            min_y: F32MAX,
            min_seen_y: F32MAX,
            max_z: F32MIN,
            max_seen_z: F32MIN,
            min_z: F32MAX,
            min_seen_z: F32MAX,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct SunShadowMakerDataPart {
    max_x: Real,
    max_seen_x: Real,
    min_x: Real,
    min_seen_x: Real,
    max_y: Real,
    max_seen_y: Real,
    min_y: Real,
    min_seen_y: Real,
    max_z: Real,
    max_seen_z: Real,
    min_z: Real,
    min_seen_z: Real,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct SunVisibilityData {
    is_in_cascades: Vec<bool>,
}

impl VisibilityData for SunVisibilityData {}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct SunShadowMakerData {
    id: Id,
    r: math::Matrix4<Real>,
    cascades: Vec<SunShadowMakerDataPart>,
}

impl SunShadowMakerData {
    fn new(id: Id, datas: &[SunCam], r: math::Matrix4<Real>) -> Self {
        let mut cascades = Vec::new();
        for data in datas {
            cascades.push(SunShadowMakerDataPart {
                max_x: data.max_x,
                max_seen_x: data.min_x,
                min_x: data.min_x,
                min_seen_x: data.max_x,
                max_y: data.max_y,
                max_seen_y: data.min_y,
                min_y: data.min_y,
                min_seen_y: data.max_y,
                max_z: data.max_z,
                max_seen_z: data.min_z,
                min_z: data.min_z,
                min_seen_z: data.max_z,
            });
        }
        cascades.shrink_to_fit();
        Self { id, r, cascades }
    }
}

impl ShadowMakerData for SunShadowMakerData {
    fn check_shadowability(&mut self, m: &mut Model) {
        let rd = m.get_occlusion_culling_radius();
        let mut is_in_cascades = vec![false; self.cascades.len()];
        let mut ci = 0;
        for c in &mut self.cascades {
            let v = (self.r * m.get_location().extend(1.0)).truncate();
            let rdv = math::Vector3::new(rd, rd, rd);
            let upv = v + rdv;
            let dnv = v - rdv;
            if upv.x < c.min_x {
                continue;
            }
            if upv.y < c.min_y {
                continue;
            }
            if upv.z < c.min_z {
                continue;
            }
            if c.max_x < dnv.x {
                continue;
            }
            if c.max_y < dnv.y {
                continue;
            }
            if dnv.x < c.min_seen_x {
                c.min_seen_x = dnv.x;
            }
            if dnv.y < c.min_seen_y {
                c.min_seen_y = dnv.y;
            }
            if dnv.z < c.min_seen_z {
                c.min_seen_z = dnv.z;
            }
            if c.max_seen_x < upv.x {
                c.max_seen_x = upv.x;
            }
            if c.max_seen_y < upv.y {
                c.max_seen_y = upv.y;
            }
            if c.max_z < upv.z {
                c.max_z = upv.z;
                c.max_seen_z = upv.z;
            }
            is_in_cascades[ci] = true;
            ci += 1;
        }
        m.set_light_visibility_data(self.id, Box::new(SunVisibilityData { is_in_cascades }));
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Sun {
    obj_base: ObjectBase,
    // cascaded camera rotation
    ccr: math::Matrix4<Real>,
    // cascaded camera data s
    ccds: Vec<SunCam>,
    direction: math::Vector3<Real>,
    color: (Real, Real, Real),
    strength: Real,
    shm_pcmd: CmdBuffer, // shadow mapper cmd
    shm_sem: Semaphore,
    acc_scmd: CmdBuffer, // secondary accumulator cmd
    acc_pcmd: CmdBuffer, // primary accumulator cmd
    acc_sem: Semaphore,
}

impl Sun {}

impl CoreObject for Sun {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Object for Sun {
    fn get_name(&self) -> Option<String> {
        self.obj_base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.obj_base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn render(&self, _: &mut CmdBuffer, _: usize) {
        vxlogf!("Sun light does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn update(&mut self) {
        self.obj_base.update();
        for ccd in &mut self.ccds {
            if ccd.max_seen_x < ccd.max_x {
                ccd.max_x = ccd.max_seen_x;
            }
            if ccd.max_seen_y < ccd.max_y {
                ccd.max_y = ccd.max_seen_y;
            }
            if ccd.max_seen_z < ccd.max_z {
                ccd.max_z = ccd.max_seen_z;
            }
            if ccd.min_seen_x > ccd.min_x {
                ccd.min_x = ccd.min_seen_x;
            }
            if ccd.min_seen_y > ccd.min_y {
                ccd.min_y = ccd.min_seen_y;
            }
            if ccd.min_seen_z > ccd.min_z {
                ccd.min_z = ccd.min_seen_z;
            }
        }
        for ccd in &mut self.ccds {
            let mmx = (ccd.max_x - ccd.min_x) * 0.02;
            let mmy = (ccd.max_y - ccd.min_y) * 0.02;
            let mz = ccd.max_z - ccd.min_z;
            let mmz = mz * 0.02;
            let p = math::ortho(
                ccd.min_x - mmx,
                ccd.max_x + mmx,
                ccd.min_y - mmy,
                ccd.min_y + mmy,
                mmz,
                mz + mmz,
            );
            let t =
                math::Matrix4::from_translation(math::Vector3::new(0.0, 0.0, -(ccd.max_z + mmz)));
            ccd.vp = p * self.ccr * t;
        }
    }

    fn is_rendarable(&self) -> bool {
        return self.obj_base.is_rendarable();
    }
}

impl Light for Sun {
    fn to_directional(&self) -> Option<&Directional> {
        return Some(self);
    }

    fn to_mut_directional(&mut self) -> Option<&mut Directional> {
        return Some(self);
    }

    fn get_shadow_maker_data(&self) -> Option<Box<ShadowMakerData>> {
        return Some(Box::new(SunShadowMakerData::new(
            self.get_id(),
            &self.ccds,
            self.ccr,
        )));
    }

    fn update_shadow_maker_data(&mut self, smd: &Box<ShadowMakerData>) {
        let smd = smd.to_sun();
        let cc = self.ccds.len();
        for i in 0..cc {
            let ccd = &mut self.ccds[i];
            let smdccd = &smd.cascades[i];
            if ccd.max_seen_x < smdccd.max_seen_x {
                ccd.max_seen_x = smdccd.max_seen_x;
            }
            if ccd.max_seen_y < smdccd.max_seen_y {
                ccd.max_seen_y = smdccd.max_seen_y;
            }
            if ccd.max_seen_z < smdccd.max_seen_z {
                ccd.max_seen_z = smdccd.max_seen_z;
                ccd.max_z = smdccd.max_seen_z;
            }
            if ccd.min_seen_x > smdccd.min_seen_x {
                ccd.min_seen_x = smdccd.min_seen_x;
            }
            if ccd.min_seen_y > smdccd.min_seen_y {
                ccd.min_seen_y = smdccd.min_seen_y;
            }
            if ccd.min_seen_z > smdccd.min_seen_z {
                ccd.min_seen_z = smdccd.min_seen_z;
            }
        }
    }
}

impl Directional for Sun {
    fn update_cascaded_shadow_map_cameras(&mut self, walls: &Vec<[math::Vector3<Real>; 4]>) {
        let mut walls_bnds = Vec::new();
        for w in walls {
            let mut max = math::Vector3::new(F32MIN, F32MIN, F32MIN);
            let mut min = math::Vector3::new(F32MAX, F32MAX, F32MAX);
            for p in w {
                let p = self.ccr * p.extend(1.0);
                if p.x < min.x {
                    min.x = p.x;
                }
                if max.x < p.x {
                    max.x = p.x;
                }
                if p.y < min.y {
                    min.y = p.y;
                }
                if max.y < p.y {
                    max.y = p.y;
                }
                if p.z < min.z {
                    min.z = p.z;
                }
            }
            walls_bnds.push((max, min));
        }
        let ccdsc = self.ccds.len();
        for i in 0..ccdsc {
            let ccd = &mut self.ccds[i];
            let ii = i + 1;
            if walls_bnds[i].1.x < walls_bnds[ii].1.x {
                ccd.min_x = walls_bnds[i].1.x;
            } else {
                ccd.min_x = walls_bnds[ii].1.x;
            }
            if walls_bnds[i].0.x < walls_bnds[ii].0.x {
                ccd.max_x = walls_bnds[ii].0.x;
            } else {
                ccd.max_x = walls_bnds[i].0.x;
            }
            if walls_bnds[i].1.y < walls_bnds[ii].1.y {
                ccd.min_y = walls_bnds[i].1.y;
            } else {
                ccd.min_y = walls_bnds[ii].1.y;
            }
            if walls_bnds[i].0.y < walls_bnds[ii].0.y {
                ccd.max_y = walls_bnds[ii].0.y;
            } else {
                ccd.max_y = walls_bnds[i].0.y;
            }
            if walls_bnds[i].1.z < walls_bnds[ii].1.z {
                ccd.min_z = walls_bnds[i].1.z;
            } else {
                ccd.min_z = walls_bnds[ii].1.z;
            }
            ccd.max_seen_x = ccd.min_x;
            ccd.max_seen_y = ccd.min_y;
            ccd.max_seen_z = ccd.min_z;
            ccd.min_seen_x = ccd.max_x;
            ccd.min_seen_y = ccd.max_y;
            ccd.min_seen_z = ccd.max_z;
        }
    }
}

impl DefaultLighting for Sun {
    fn default(eng: &Engine) -> Self {
        let mut ccds = Vec::new();
        let csc = eng.get_config().cascaded_shadows_count;
        let geng = vxresult!(eng.get_gapi_engine().read());
        for _ in 0..csc {
            ccds.push(SunCam::new());
        }
        ccds.shrink_to_fit();
        Sun {
            obj_base: ObjectBase::new(),
            ccr: math::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            ccds,
            direction: math::Vector3::new(0.0, 0.0, -1.0),
            color: (1.0, 1.0, 1.0),
            strength: 1.0,
            shm_pcmd: geng.create_primary_command_buffer_from_main_graphic_pool(),
            shm_sem: geng.create_semaphore(),
            acc_scmd: geng.create_secondary_command_buffer_from_main_graphic_pool(),
            acc_pcmd: geng.create_primary_command_buffer_from_main_graphic_pool(),
            acc_sem: geng.create_semaphore(),
        }
    }
}

impl Loadable for Sun {
    fn new_with_gltf(_node: &gltf::Node, _eng: &Engine, _: &[u8]) -> Self {
        vxunimplemented!();
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, id: Id) -> Self {
        let location = math::Vector3::new(reader.read(), reader.read(), reader.read());
        let r = [reader.read(), reader.read(), reader.read(), reader.read()];
        let r = math::Quaternion::new(r[3], r[0], r[1], r[2]);
        let mut camera = Orthographic::new_with_id(engine, id);
        camera.set_location(&location);
        camera.set_orientation(&r);
        let color = (reader.read(), reader.read(), reader.read());
        let strength = reader.read();
        vxtodo!(); // ccr is not correct
        vxtodo!(); // ccds is not correct
        vxtodo!(); // direction is not correct
        let geng = vxresult!(engine.get_gapi_engine().read());
        Sun {
            obj_base: ObjectBase::new_with_id(id),
            ccr: math::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            ccds: Vec::new(),
            direction: math::Vector3::new(0.0, 0.0, -1.0),
            color,
            strength,
            shm_pcmd: geng.create_primary_command_buffer_from_main_graphic_pool(),
            shm_sem: geng.create_semaphore(),
            acc_scmd: geng.create_secondary_command_buffer_from_main_graphic_pool(),
            acc_pcmd: geng.create_primary_command_buffer_from_main_graphic_pool(),
            acc_sem: geng.create_semaphore(),
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
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

    pub fn create<L>(&mut self, eng: &Engine, name: &str) -> Arc<RwLock<L>>
    where
        L: 'static + Light + DefaultLighting,
    {
        let result = L::default(eng);
        let id = result.get_id();
        let result = Arc::new(RwLock::new(result));
        let light: Arc<RwLock<Light>> = result.clone();
        self.lights.insert(id, Arc::downgrade(&light));
        self.name_to_id.insert(name.to_string(), id);
        return result;
    }

    pub fn load_gx3d(&mut self, eng: &Engine, id: Id) -> Arc<RwLock<Light>> {
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
#[cfg_attr(debug_mode, derive(Debug))]
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
#[cfg_attr(debug_mode, derive(Debug))]
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
