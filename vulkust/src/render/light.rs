use super::super::core::debug::Debug as CoreDebug;
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::camera::Orthographic;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::engine::Engine;
use super::buffer::{DynamicBuffer, Manager as BufferManager};
use super::descriptor::{Set as DescriptorSet, Manager as DescriptorManager};
use super::gapi::GraphicApiEngine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::model::Model;
use super::mesh::Mesh;
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use super::sync::Semaphore;
use std::collections::BTreeMap;
use std::f32::MAX as F32MAX;
use std::f32::MIN as F32MIN;
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::mem::size_of;

use gltf;
use math;
use num_cpus;

#[cfg_attr(debug_mode, derive(Debug))]
#[repr(u8)]
pub(crate) enum TypeId {
    Sun = 1,
}

pub trait ShadowMakerKernelData: CoreDebug + Send {
    fn shadow(&mut self, &mut Model, &Arc<RwLock<Model>>, usize); // old name was check_shadowability
    fn create_commands(&mut self, &GraphicApiEngine, &Arc<CmdPool>);
    fn render(&mut self, usize);
}

pub trait Light: Object {
    fn to_directional(&self) -> Option<&Directional>;
    fn to_mut_directional(&mut self) -> Option<&mut Directional>;
    fn has_shadow(&self) -> bool;
}

pub trait Directional: Light {
    fn update_cascaded_shadow_map_cameras(&mut self, &Vec<[math::Vector3<Real>; 4]>);
}

pub trait DefaultLighting {
    fn default(eng: &Engine) -> Self;
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
struct SunCascadeCamera {
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

impl SunCascadeCamera {
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
    
    fn update_limits(&mut self, other: &Self) {
        self.max_x = other.max_x;
        self.max_seen_x = other.min_x;
        self.min_x = other.min_x;
        self.min_seen_x = other.max_x;
        self.max_y = other.max_y;
        self.max_seen_y = other.min_y;
        self.min_y = other.min_y;
        self.min_seen_y = other.max_y;
        self.max_z = other.max_z;
        self.max_seen_z = other.min_z;
        self.min_z = other.min_z;
        self.min_seen_z = other.max_z;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct SunShadowMakerKernelFrameData {
    cascades_cmds: Vec<CmdBuffer>,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct SunShadowMapperRenderData {
    uniform_buffer: DynamicBuffer,
    cascade_index: usize,
    model: Option<Weak<RwLock<Model>>>,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct SunShadowMakerKernelData {
    zero_located_view: math::Matrix4<Real>,
    cascade_cameras: Vec<SunCascadeCamera>,
    frames_data: Vec<SunShadowMakerKernelFrameData>,
    render_data: Vec<SunShadowMapperRenderData>, // per model and cascade
    descriptor_set: DescriptorSet,
    last_render_data_index: usize,
}

impl SunShadowMakerKernelData {
    fn new(
        zero_located_view: math::Matrix4<Real>,
        cascades_count: usize,
        frames_count: usize,
        max_render_data_count: usize, // model-cascade
        buffer_manager: &mut BufferManager,
        descriptor_manager: &mut DescriptorManager,
    ) -> Self {
        let cascade_cameras = vec![SunCascadeCamera::new(); cascades_count];
        let mut frames_data = Vec::with_capacity(frames_count);
        let mut render_data = Vec::with_capacity(max_render_data_count);
        for _ in 0..max_render_data_count {
            render_data.push(SunShadowMapperRenderData {
                uniform_buffer: buffer_manager.create_dynamic_buffer(size_of::<math::Matrix4<Real>>() as isize),
                model: None,
                cascade_index: 0,
            });
        }
        let descriptor_set = descriptor_manager.create_buffer_only_set(&render_data[0].uniform_buffer);
        for _ in 0..frames_count {
            frames_data.push( SunShadowMakerKernelFrameData {
                cascades_cmds: Vec::with_capacity(cascades_count),
            });
        }
        Self {
            zero_located_view,
            cascade_cameras,
            frames_data,
            render_data,
            descriptor_set,
            last_render_data_index: 0,
        }
    }

    // calls at very first time in a render loop
    fn update_camera_limits(&mut self, cascade_cameras: &[SunCascadeCamera], zero_located_view: math::Matrix4<Real>) {
        self.zero_located_view = zero_located_view;
        let cc = cascade_cameras.len();
        for i in 0..cc {
            self.cascade_cameras[i].update_limits(&cascade_cameras[i]);
        }
        self.last_render_data_index = 0;
    }

    // calls after all shadow model is done
    fn update_camera_view_projection_matrices(&mut self, cascade_cameras: &[SunCascadeCamera]) {
        let cc = cascade_cameras.len();
        for i in 0..cc {
            self.cascade_cameras[i].vp = cascade_cameras[i].vp;
        }
    }
}

unsafe impl Send for SunShadowMakerKernelData {}

impl ShadowMakerKernelData for SunShadowMakerKernelData {
    fn shadow(&mut self, m: &mut Model, model: &Arc<RwLock<Model>>, frame_number: usize) {
        let rd = m.get_occlusion_culling_radius();
        let v = (self.zero_located_view * m.get_location().extend(1.0)).truncate();
        let rdv = math::Vector3::new(rd, rd, rd);
        let upv = v + rdv;
        let dnv = v - rdv;
        let mut cascade_index = 0;
        for c in &mut self.cascade_cameras {
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
            let render_data = &mut self.render_data[self.last_render_data_index];
            render_data.model = Some(Arc::downgrade(model));
            render_data.cascade_index = cascade_index;
            self.last_render_data_index += 1;
            cascade_index += 1;
        }
    }

    fn create_commands(&mut self, geng: &GraphicApiEngine, cmd_pool: &Arc<CmdPool>) {
        let cascades_count = self.cascade_cameras.len();
        for fd in &mut self.frames_data {
            for _ in 0..cascades_count {
                fd.cascades_cmds.push(geng.create_secondary_command_buffer(cmd_pool.clone()));
            }
        }
    }

    fn render(&mut self, frame_number: usize) {
        for i in 0..self.last_render_data_index {
            let rd = &mut self.render_data[i];
            let ci = rd.cascade_index;
            let model = vxunwrap!(&rd.model).upgrade();
            if model.is_none() {
                continue;
            }
            let model = vxunwrap!(model);
            let model = vxresult!(model.read());
            let mvp = &self.cascade_cameras[ci].vp * model.get_uniform().get_model();
            rd.uniform_buffer.update(&mvp, frame_number);
            let cmd = &mut self.frames_data[frame_number].cascades_cmds[ci];
            cmd.bind_shadow_mapper_light_descriptor(
                &self.descriptor_set,
                &*vxresult!(rd.uniform_buffer.get_buffer(frame_number).read()));
            model.render_shadow(cmd, frame_number);
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Sun {
    obj_base: ObjectBase,
    zero_located_view: math::Matrix4<Real>,
    cascade_cameras: Vec<SunCascadeCamera>,
    kernels_data: Vec<Arc<Mutex<SunShadowMakerKernelData>>>,
    direction: math::Vector3<Real>,
    color: (Real, Real, Real),
    strength: Real,
    shadow_mapper_primary_command: CmdBuffer,
    shadow_mapper_semaphore: Semaphore,
    shadow_accumulator_secondary_command: CmdBuffer,
    shadow_accumulator_primary_command: CmdBuffer,
    shadow_accumulator_semaphore: Semaphore,
}

impl Sun {
    fn update_with_kernel_data(&mut self, smd: &SunShadowMakerKernelData) {
        let cc = self.cascade_cameras.len();
        for i in 0..cc {
            let ccd = &mut self.cascade_cameras[i];
            let smdccd = &smd.cascade_cameras[i];
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

    fn update(&mut self, frame_number: usize) {
        self.obj_base.update(frame_number);
        for smd in &self.kernels_data {
            let smd = vxresult!(smd.lock());
            self.update_with_kernel_data(&*smd);
        }
        for ccd in &mut self.cascade_cameras {
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
        for ccd in &mut self.cascade_cameras {
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
            ccd.vp = p * self.zero_located_view * t;
        }
        for smd in &self.kernels_data {
            let smd = vxresult!(smd.lock());
            smd.update_camera_view_projection_matrices(&self.cascade_cameras);
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

    fn has_shadow(&self) -> bool {
        return true;
    }
}

impl Directional for Sun {
    fn update_cascaded_shadow_map_cameras(&mut self, walls: &Vec<[math::Vector3<Real>; 4]>) {
        let mut walls_bnds = Vec::new();
        for w in walls {
            let mut max = math::Vector3::new(F32MIN, F32MIN, F32MIN);
            let mut min = math::Vector3::new(F32MAX, F32MAX, F32MAX);
            for p in w {
                let p = self.zero_located_view * p.extend(1.0);
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
        let ccdsc = self.cascade_cameras.len();
        for i in 0..ccdsc {
            let ccd = &mut self.cascade_cameras[i];
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
        let csc = eng.get_config().cascaded_shadows_count as usize;
        let max_render_data_count = eng.get_config().max_shadow_maker_kernek_render_data_count as usize;
        let mut cascade_cameras = Vec::with_capacity(csc);
        let geng = vxresult!(eng.get_gapi_engine().read());
        let frames_count = geng.get_frames_count();
        for _ in 0..csc {
            cascade_cameras.push(SunCascadeCamera::new());
        }
        let zero_located_view = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let num_cpus = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(num_cpus);
        let buffer_manager = vxresult!(geng.get_buffer_manager().write());
        let descriptor_manager = vxresult!(geng.get_descriptor_manager().write());
        for _ in 0..num_cpus {
            let kernel_data = Arc::new(Mutex::new(SunShadowMakerKernelData::new(
                zero_located_view,
                csc,
                frames_count,
                max_render_data_count,
                &mut *buffer_manager,
                &mut *descriptor_manager,
            )));
            kernels_data.push(kernel_data);
        }
        Sun {
            obj_base: ObjectBase::new(),
            zero_located_view,
            cascade_cameras,
            kernels_data,
            direction: math::Vector3::new(0.0, 0.0, -1.0),
            color: (1.0, 1.0, 1.0),
            strength: 1.0,
            shadow_mapper_primary_command: geng.create_primary_command_buffer_from_main_graphic_pool(),
            shadow_mapper_semaphore: geng.create_semaphore(),
            shadow_accumulator_secondary_command: geng.create_secondary_command_buffer_from_main_graphic_pool(),
            shadow_accumulator_primary_command: geng.create_primary_command_buffer_from_main_graphic_pool(),
            shadow_accumulator_semaphore: geng.create_semaphore(),
        }
    }
}

impl Loadable for Sun {
    fn new_with_gltf(_node: &gltf::Node, _eng: &Engine, _: &[u8]) -> Self {
        vxunimplemented!();
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, id: Id) -> Self {
        let mut myself = Self::default(engine);
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
        return myself;
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
