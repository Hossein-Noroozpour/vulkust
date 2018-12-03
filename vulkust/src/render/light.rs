use super::super::core::constants::EPSILON;
use super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::buffer::{Dynamic as DynamicBuffer, Manager as BufferManager};
use super::camera::Orthographic;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::config::MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT;
use super::engine::Engine;
use super::gapi::GraphicApiEngine;
use super::model::Model;
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use super::shadower::Shadower;
use super::sync::Semaphore;
use std::collections::BTreeMap;
use std::f32::MAX as F32MAX;
use std::f32::MIN as F32MIN;
use std::mem::size_of;
use std::sync::{Arc, Mutex, RwLock, Weak};

use gltf;
use math;
use num_cpus;

#[cfg_attr(debug_mode, derive(Debug))]
#[repr(u8)]
pub(crate) enum TypeId {
    Sun = 1,
}

pub trait Light: Object {
    fn to_directional(&self) -> Option<&Directional>;
    fn to_mut_directional(&mut self) -> Option<&mut Directional>;
    fn to_point(&self) -> Option<&Point>;
    fn to_mut_point(&mut self) -> Option<&mut Point>;
    fn to_shadow_maker(&self) -> Option<&ShadowMaker>;
    fn to_mut_shadow_maker(&mut self) -> Option<&mut ShadowMaker>;
    fn update(&mut self);
}

pub trait ShadowMaker: Light {
    fn shadow(&self, &mut Model, &Arc<RwLock<Model>>, usize);
    fn begin_secondary_commands(&self, &GraphicApiEngine, &Arc<CmdPool>, &Shadower, usize, usize);
    fn render_shadow_mapper(&self, &Shadower, usize, usize);
    fn submit_shadow_mapper(
        &mut self,
        &Semaphore,
        &GraphicApiEngine,
        &Shadower,
        usize,
    ) -> Arc<Semaphore>;
}

pub trait Directional: Light {
    fn update_cascaded_shadow_map_cameras(&mut self, &Vec<[math::Vector3<Real>; 4]>, usize);
    fn update_uniform(&self, &mut DirectionalUniform);
}

pub trait Point: Light {
    fn update_uniform(&self, &mut PointUniform);
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
    last_render_data_index: usize,
}

impl SunShadowMakerKernelData {
    fn new(
        zero_located_view: math::Matrix4<Real>,
        cascades_count: usize,
        frames_count: usize,
        max_render_data_count: usize, // model-cascade
        buffer_manager: &mut BufferManager,
    ) -> Self {
        let cascade_cameras = vec![SunCascadeCamera::new(); cascades_count];
        let mut frames_data = Vec::with_capacity(frames_count);
        let mut render_data = Vec::with_capacity(max_render_data_count);
        for _ in 0..max_render_data_count {
            let uniform_buffer =
                buffer_manager.create_dynamic_buffer(size_of::<math::Matrix4<Real>>() as isize);
            render_data.push(SunShadowMapperRenderData {
                uniform_buffer,
                model: None,
                cascade_index: 0,
            });
        }
        for _ in 0..frames_count {
            frames_data.push(SunShadowMakerKernelFrameData {
                cascades_cmds: Vec::with_capacity(cascades_count),
            });
        }
        Self {
            zero_located_view,
            cascade_cameras,
            frames_data,
            render_data,
            last_render_data_index: 0,
        }
    }

    // calls at very first time in a render loop
    fn update_camera_limits(
        &mut self,
        cascade_cameras: &[SunCascadeCamera],
        zero_located_view: math::Matrix4<Real>,
    ) {
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

    fn shadow(&mut self, m: &mut Model, model: &Arc<RwLock<Model>>) {
        let rd = m.get_occlusion_culling_radius();
        let v = (self.zero_located_view * m.get_location().extend(0.0)).truncate();
        let rdv = math::Vector3::new(rd, rd, rd);
        let upv = v + rdv;
        let dnv = v - rdv;
        let ccc = self.cascade_cameras.len();
        for ci in 0..ccc {
            let c = &mut self.cascade_cameras[ci];
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
            if c.max_seen_z < upv.z {
                c.max_seen_z = upv.z;
            }
            let render_data = &mut self.render_data[self.last_render_data_index];
            render_data.model = Some(Arc::downgrade(model));
            render_data.cascade_index = ci;
            self.last_render_data_index += 1;
        }
    }

    fn begin_secondary_commands(
        &mut self,
        geng: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        shadower: &Shadower,
        frame_number: usize,
    ) {
        let cascades_count = self.cascade_cameras.len();
        let fd = &mut self.frames_data[frame_number];
        let cmds_len = fd.cascades_cmds.len();
        for _ in cmds_len..cascades_count {
            fd.cascades_cmds
                .push(geng.create_secondary_command_buffer(cmd_pool.clone()));
        }
        shadower.begin_secondary_shadow_mappers(&mut fd.cascades_cmds);
        self.last_render_data_index = 0;
    }

    fn render_shadow_mapper(&mut self, shadower: &Shadower, frame_number: usize) {
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
            // vxloge!("{:?}", mvp);
            let cmd = &mut self.frames_data[frame_number].cascades_cmds[ci];
            cmd.bind_shadow_mapper_light_descriptor(
                shadower.get_shadow_map_descriptor_set(),
                &*vxresult!(rd.uniform_buffer.get_buffer(frame_number).read()),
            );
            model.render_shadow(cmd, frame_number);
        }
        for cmd in &mut self.frames_data[frame_number].cascades_cmds {
            cmd.end();
        }
        self.last_render_data_index = 0;
    }
}

unsafe impl Send for SunShadowMakerKernelData {}

#[cfg_attr(debug_mode, derive(Debug))]
struct SunFrameData {
    shadow_mappers_primary_commands: Vec<CmdBuffer>,
    shadow_mappers_semaphore: Arc<Semaphore>,
    shadow_accumulator_secondary_command: CmdBuffer,
    shadow_accumulator_primary_command: CmdBuffer,
    shadow_accumulator_semaphore: Arc<Semaphore>,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Sun {
    obj_base: ObjectBase,
    zero_located_view: math::Matrix4<Real>,
    cascade_cameras: Vec<SunCascadeCamera>,
    kernels_data: Vec<Arc<Mutex<SunShadowMakerKernelData>>>,
    direction: math::Vector3<Real>,
    color: math::Vector3<Real>,
    strength: Real,
    frames_data: Vec<SunFrameData>,
    shadow_accumulator_uniform: ShadowAccumulatorDirectionalUniform,
    shadow_accumulator_uniform_buffer: DynamicBuffer,
}

impl Sun {
    fn update_with_kernels_data(&mut self) {
        let kernels_count = self.kernels_data.len();
        for ki in 0..kernels_count {
            let smd = vxresult!(self.kernels_data[ki].lock());
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

impl Light for Sun {
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
        return Some(self);
    }

    fn to_mut_shadow_maker(&mut self) -> Option<&mut ShadowMaker> {
        return Some(self);
    }

    fn update(&mut self) {
        self.update_with_kernels_data();
        for ccd in &mut self.cascade_cameras {
            let change_max_x = ccd.max_seen_x < ccd.max_x && ccd.min_x + EPSILON < ccd.max_seen_x;
            let change_max_y = ccd.max_seen_y < ccd.max_y && ccd.min_y + EPSILON < ccd.max_seen_y;
            let change_max_z = ccd.max_seen_z < ccd.max_z && ccd.min_z + EPSILON < ccd.max_seen_z;
            let change_min_x = ccd.min_seen_x > ccd.min_x && ccd.max_x > ccd.min_seen_x + EPSILON;
            let change_min_y = ccd.min_seen_y > ccd.min_y && ccd.max_y > ccd.min_seen_y + EPSILON;
            let change_min_z = ccd.min_seen_z > ccd.min_z && ccd.max_z > ccd.min_seen_z + EPSILON;
            if change_max_x {
                ccd.max_x = ccd.max_seen_x;
            }
            if change_max_y {
                ccd.max_y = ccd.max_seen_y;
            }
            if change_max_z {
                ccd.max_z = ccd.max_seen_z;
            } else if ccd.max_seen_z > ccd.max_z {
                ccd.max_z = ccd.max_seen_z;
            }
            if change_min_x {
                ccd.min_x = ccd.min_seen_x;
            }
            if change_min_y {
                ccd.min_y = ccd.min_seen_y;
            }
            if change_min_z {
                ccd.min_z = ccd.min_seen_z;
            }
        }
        for ccd in &mut self.cascade_cameras {
            let width = ((ccd.max_x - ccd.min_x) * 0.51).abs();
            let height = ((ccd.max_y - ccd.min_y) * 0.51).abs();
            let depth = (ccd.max_z - ccd.min_z).abs();
            let near = depth * 0.01;
            let far = depth * 1.03;
            let p = math::ortho(-width, width, -height, height, near, far);
            let t = -math::Vector3::new(
                (ccd.max_x + ccd.min_x) * 0.5,
                (ccd.max_y + ccd.min_y) * 0.5,
                ccd.max_z + near * 2.0,
            );
            let t = math::Matrix4::from_translation(t);
            ccd.vp = math::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
            ) * p
                * self.zero_located_view
                * t;
        }
    }
}

impl ShadowMaker for Sun {
    fn shadow(&self, m: &mut Model, mc: &Arc<RwLock<Model>>, kernel_index: usize) {
        vxresult!(self.kernels_data[kernel_index].lock()).shadow(m, mc);
    }

    fn begin_secondary_commands(
        &self,
        geng: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        sh: &Shadower,
        kernel_index: usize,
        frame_number: usize,
    ) {
        let mut kernel_data = vxresult!(self.kernels_data[kernel_index].lock());
        kernel_data.update_camera_limits(&self.cascade_cameras, self.zero_located_view);
        kernel_data.begin_secondary_commands(geng, cmd_pool, sh, frame_number);
    }

    fn render_shadow_mapper(&self, shadower: &Shadower, kernel_index: usize, frame_number: usize) {
        let mut kernel_data = vxresult!(self.kernels_data[kernel_index].lock());
        kernel_data.update_camera_view_projection_matrices(&self.cascade_cameras);
        kernel_data.render_shadow_mapper(shadower, frame_number);
    }

    fn submit_shadow_mapper(
        &mut self,
        sem: &Semaphore,
        geng: &GraphicApiEngine,
        shadower: &Shadower,
        frame_number: usize,
    ) -> Arc<Semaphore> {
        let cascades_count = self.cascade_cameras.len();
        let frame_data = &mut self.frames_data[frame_number];
        for i in 0..cascades_count {
            self.shadow_accumulator_uniform.view_projection_biases[i] = math::Matrix4::new(
                0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.0, 1.0,
            ) * self.cascade_cameras[i]
                .vp;
            let pricmd = &mut frame_data.shadow_mappers_primary_commands[i];
            pricmd.begin();
            shadower.begin_shadow_map_primary(pricmd, i);
            for kd in &self.kernels_data {
                let kd = vxresult!(kd.lock());
                let cmd = &kd.frames_data[frame_number].cascades_cmds[i];
                if cmd.get_has_render_record() {
                    pricmd.exe_cmd(cmd);
                }
            }
            pricmd.end_render_pass();
            pricmd.end();
        }
        let mut cmds = Vec::with_capacity(frame_data.shadow_mappers_primary_commands.len());
        for c in &frame_data.shadow_mappers_primary_commands {
            cmds.push(c);
        }
        geng.submit_multiple(&[sem], &cmds, &[&frame_data.shadow_mappers_semaphore]);
        self.shadow_accumulator_uniform.cascades_count = cascades_count as u32;
        self.shadow_accumulator_uniform.direction_strength =
            math::Vector4::new(0.0, 0.0, -1.0, 0.5);
        self.shadow_accumulator_uniform_buffer
            .update(&self.shadow_accumulator_uniform, frame_number);
        {
            let cmd = &mut frame_data.shadow_accumulator_secondary_command;
            cmd.begin_secondary(shadower.get_accumulator_framebuffer());
            cmd.bind_pipeline(shadower.get_shadow_accumulator_directional_pipeline());
            cmd.bind_shadow_accumulator_directional_descriptor(
                shadower.get_shadow_accumulator_directional_descriptor_set(),
                &*vxresult!(self
                    .shadow_accumulator_uniform_buffer
                    .get_buffer(frame_number)
                    .read()),
            );
            cmd.render_shadow_accumulator_directional();
            cmd.end();
        }
        {
            let cmd = &mut frame_data.shadow_accumulator_primary_command;
            cmd.begin();
            shadower.get_accumulator_framebuffer().begin(cmd);
            cmd.exe_cmd(&frame_data.shadow_accumulator_secondary_command);
            cmd.end_render_pass();
            cmd.end();
        }
        geng.submit(
            &frame_data.shadow_mappers_semaphore,
            &frame_data.shadow_accumulator_primary_command,
            &frame_data.shadow_accumulator_semaphore,
        );
        return frame_data.shadow_accumulator_semaphore.clone();
    }
}

impl Directional for Sun {
    fn update_cascaded_shadow_map_cameras(
        &mut self,
        walls: &Vec<[math::Vector3<Real>; 4]>,
        index: usize,
    ) {
        self.shadow_accumulator_uniform.light_index = index as u32;
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
                if max.z < p.z {
                    max.z = p.z;
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
            if walls_bnds[i].0.z < walls_bnds[ii].0.z {
                ccd.max_z = walls_bnds[ii].0.z;
            } else {
                ccd.max_z = walls_bnds[i].0.z;
            }
            ccd.max_seen_x = ccd.min_x;
            ccd.max_seen_y = ccd.min_y;
            ccd.max_seen_z = ccd.min_z;
            ccd.min_seen_x = ccd.max_x;
            ccd.min_seen_y = ccd.max_y;
            ccd.min_seen_z = ccd.max_z;
        }
    }

    fn update_uniform(&self, u: &mut DirectionalUniform) {
        u.color = self.color.extend(1.0);
        u.direction = self.direction.extend(self.strength);
    }
}

impl DefaultLighting for Sun {
    fn default(eng: &Engine) -> Self {
        let csc = eng.get_config().get_cascaded_shadows_count() as usize;
        let num_cpus = num_cpus::get();
        let max_render_data_count =
            eng.get_config()
                .get_max_shadow_maker_kernel_render_data_count() as usize;
        let mut cascade_cameras = Vec::with_capacity(csc);
        let geng = vxresult!(eng.get_gapi_engine().read());
        let frames_count = geng.get_frames_count();
        let mut frames_data = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            let mut shadow_mappers_primary_commands = Vec::with_capacity(num_cpus);
            for _ in 0..csc {
                shadow_mappers_primary_commands
                    .push(geng.create_primary_command_buffer_from_main_graphic_pool());
            }
            frames_data.push(SunFrameData {
                shadow_mappers_primary_commands,
                shadow_mappers_semaphore: Arc::new(geng.create_semaphore()),
                shadow_accumulator_secondary_command: geng
                    .create_secondary_command_buffer_from_main_graphic_pool(),
                shadow_accumulator_primary_command: geng
                    .create_primary_command_buffer_from_main_graphic_pool(),
                shadow_accumulator_semaphore: Arc::new(geng.create_semaphore()),
            });
        }
        for _ in 0..csc {
            cascade_cameras.push(SunCascadeCamera::new());
        }
        let zero_located_view = math::Matrix4::look_at(
            math::Point3::new(0.0, 0.0, 0.0),
            math::Point3::new(0.0, 0.0, -1.0),
            math::Vector3::new(0.0, 1.0, 0.0),
        );
        let mut kernels_data = Vec::with_capacity(num_cpus);
        let mut buffer_manager = vxresult!(geng.get_buffer_manager().write());
        for _ in 0..num_cpus {
            let kernel_data = Arc::new(Mutex::new(SunShadowMakerKernelData::new(
                zero_located_view,
                csc,
                frames_count,
                max_render_data_count,
                &mut *buffer_manager,
            )));
            kernels_data.push(kernel_data);
        }
        let shadow_accumulator_uniform = ShadowAccumulatorDirectionalUniform::new();
        let shadow_accumulator_uniform_buffer = buffer_manager
            .create_dynamic_buffer(size_of::<ShadowAccumulatorDirectionalUniform>() as isize);
        Sun {
            obj_base: ObjectBase::new(),
            zero_located_view,
            cascade_cameras,
            kernels_data,
            direction: math::Vector3::new(0.0, 0.0, -1.0),
            color: math::Vector3::new(1.0, 1.0, 1.0),
            strength: 0.5,
            frames_data,
            shadow_accumulator_uniform,
            shadow_accumulator_uniform_buffer,
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
        myself.color = math::Vector3::new(reader.read(), reader.read(), reader.read());
        myself.strength = reader.read();
        vxtodo!(); // ccr is not correct
        vxtodo!(); // ccds is not correct
        vxtodo!(); // direction is not correct
                   // let geng = vxresult!(engine.get_gapi_engine().read());
        return myself;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    engine: Option<Weak<RwLock<Engine>>>,
    lights: BTreeMap<Id, Weak<RwLock<Light>>>,
    name_to_id: BTreeMap<String, Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            engine: None,
            lights: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
    }

    pub fn create<L>(&mut self) -> Arc<RwLock<L>>
    where
        L: 'static + Light + DefaultLighting,
    {
        let eng = vxunwrap!(vxunwrap!(&self.engine).upgrade());
        let eng = vxresult!(eng.read());
        let result = L::default(&*eng);
        let id = result.get_id();
        let result = Arc::new(RwLock::new(result));
        let light: Arc<RwLock<Light>> = result.clone();
        self.lights.insert(id, Arc::downgrade(&light));
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
        let reader: &mut Gx3DReader = table.get_mut_reader();
        let type_id = reader.read_type_id();
        let result: Arc<RwLock<Light>> = if type_id == TypeId::Sun as u8 {
            Arc::new(RwLock::new(Sun::new_with_gx3d(eng, reader, id)))
        } else {
            vxunexpected!();
        };
        self.lights.insert(id, Arc::downgrade(&result));
        return result;
    }

    pub(crate) fn set_engine(&mut self, e: Weak<RwLock<Engine>>) {
        self.engine = Some(e);
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
}

impl DirectionalUniform {
    pub fn new() -> Self {
        Self {
            color: math::Vector4::new(0.0, 0.0, 0.0, 0.0),
            direction: math::Vector4::new(0.0, 0.0, -1.0, 0.0),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct ShadowAccumulatorDirectionalUniform {
    view_projection_biases: [math::Matrix4<Real>; MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
    direction_strength: math::Vector4<Real>,
    cascades_count: u32,
    light_index: u32,
}

impl ShadowAccumulatorDirectionalUniform {
    fn new() -> Self {
        Self {
            view_projection_biases: [math::Matrix4::new(
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ); MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
            direction_strength: math::Vector4::new(0.0, 0.0, 0.0, 0.0),
            cascades_count: 0,
            light_index: 0,
        }
    }
}
