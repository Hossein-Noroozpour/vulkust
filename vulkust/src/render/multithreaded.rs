use super::super::core::types::Id;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::config::Configurations;
use super::deferred::Deferred;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::light::ShadowMakerData;
use super::model::Model;
use super::object::Object;
use super::resolver::Resolver;
use super::scene::{Manager as SceneManager, Scene};
use super::shadower::Shadower;
use super::sync::Semaphore;
use num_cpus;
use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelPassesCommands {
    gbuff: CmdBuffer,
    // light->(cascaded/camera index)->cmd
    lights: BTreeMap<Id, Vec<CmdBuffer>>, // todo move it to the lights
}

impl KernelPassesCommands {
    fn new(engine: &GraphicApiEngine, cmd_pool: Arc<CmdPool>) -> Self {
        let gbuff = engine.create_secondary_command_buffer(cmd_pool.clone());
        let lights = BTreeMap::new();
        Self { gbuff, lights }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelSceneData {
    cmds: KernelPassesCommands,
    shadow_makers_data: BTreeMap<Id, Box<ShadowMakerKernelData>>,
}

impl KernelSceneData {
    fn new(scene: &Scene, g_engine: &GraphicApiEngine, cmd_pool: Arc<CmdPool>) -> Self {
        let mut shadow_makers_data = BTreeMap::new();
        scene.update_shadow_makers_data(&mut shadow_makers_data);
        Self {
            cmds: KernelPassesCommands::new(g_engine, cmd_pool),
            shadow_makers_data,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelFrameData {
    scenes: BTreeMap<Id, KernelSceneData>,
}

impl KernelFrameData {
    fn new() -> Self {
        Self {
            scenes: BTreeMap::new(),
        }
    }

    fn remove_scene(&mut self, id: &Id) {
        self.scenes.remove(id);
    }

    fn has_scene(&self, id: &Id) -> bool {
        return self.scenes.contains_key(id);
    }

    fn add_scene(&mut self, id: Id, kernel_scene_data: KernelSceneData) {
        self.scenes.insert(id, kernel_scene_data);
    }

    fn get_mut_scene(&mut self, id: &Id) -> Option<&mut KernelSceneData> {
        return self.scenes.get_mut(id);
    }

    fn get_scene(&self, id: &Id) -> Option<&KernelSceneData> {
        return self.scenes.get(id);
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct Kernel {
    render_signal: Sender<bool>,
    render_wait: Receiver<()>,
    shadow_signal: Sender<()>,
    shadow_wait: Receiver<()>,
    handle: JoinHandle<()>,
    frame_datas: Arc<Mutex<Vec<KernelFrameData>>>,
}

impl Kernel {
    pub fn new(
        index: usize,
        kernels_count: usize,
        engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
        g_buffer_filler: Arc<RwLock<GBufferFiller>>,
        shadower: Arc<RwLock<Shadower>>,
    ) -> Self {
        let (render_signal, render_receiver) = channel();
        let (render_ready, render_wait) = channel();
        let (shadow_signal, shadow_receiver) = channel();
        let (shadow_ready, shadow_wait) = channel();
        let frame_datas = Arc::new(Mutex::new(Vec::new()));
        let cmdbuffs = frame_datas.clone();
        let handle = spawn(move || {
            let mut renderer = Renderer::new(
                index,
                kernels_count,
                cmdbuffs,
                engine,
                scene_manager,
                g_buffer_filler,
                shadower,
            );
            while vxresult!(render_receiver.recv()) {
                renderer.render();
                vxresult!(render_ready.send(()));
                vxresult!(shadow_receiver.recv());
                renderer.shadow();
                vxresult!(shadow_ready.send(()));
            }
            vxresult!(render_ready.send(()));
        });
        Self {
            render_signal,
            render_wait,
            shadow_signal,
            shadow_wait,
            handle,
            frame_datas,
        }
    }

    fn start_rendering(&self) {
        vxresult!(self.render_signal.send(true));
    }

    fn wait_rendering(&self) {
        vxresult!(self.render_wait.recv());
    }

    fn start_shadowing(&self) {
        vxresult!(self.shadow_signal.send(()));
    }

    fn wait_shadowing(&self) {
        vxresult!(self.shadow_wait.recv());
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        vxresult!(self.render_signal.send(false));
        vxresult!(self.render_wait.recv());
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct Renderer {
    index: usize,
    kernels_count: usize,
    g_engine: Arc<RwLock<GraphicApiEngine>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    cmd_pool: Arc<CmdPool>,
    cmd_buffers: Arc<Mutex<Vec<KernelFrameData>>>,
    g_buffer_filler: Arc<RwLock<GBufferFiller>>,
    shadower: Arc<RwLock<Shadower>>,
}

impl Renderer {
    pub fn new(
        index: usize,
        kernels_count: usize,
        cmd_buffers: Arc<Mutex<Vec<KernelFrameData>>>,
        g_engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
        g_buffer_filler: Arc<RwLock<GBufferFiller>>,
        shadower: Arc<RwLock<Shadower>>,
    ) -> Self {
        let eng = g_engine.clone();
        let eng = vxresult!(eng.read());
        let cmd_pool = eng.create_command_pool();
        let frames_count = eng.get_frames_count();
        let cmdsss = cmd_buffers.clone();
        let mut cmdsss = vxresult!(cmdsss.lock());
        for _ in 0..frames_count {
            cmdsss.push(KernelFrameData::new());
        }
        cmdsss.shrink_to_fit();
        Renderer {
            index,
            kernels_count,
            g_engine,
            scene_manager,
            cmd_pool,
            cmd_buffers,
            g_buffer_filler,
            shadower,
        }
    }

    pub fn render(&mut self) {
        let g_engine = vxresult!(self.g_engine.read());
        let frame_number = g_engine.get_frame_number();
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let scenes = vxresult!(scenes.read());
        let mut cmdsss = vxresult!(self.cmd_buffers.lock());
        let cmdss = &mut cmdsss[frame_number];
        let g_buffer_filler = vxresult!(self.g_buffer_filler.read());
        let mut task_index = 0usize;
        for (scene_id, scene) in &*scenes {
            let scene = scene.upgrade();
            if scene.is_none() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            let mut need_update_shadow_makers_data = true;
            if !cmdss.has_scene(scene_id) {
                need_update_shadow_makers_data = false;
                cmdss.add_scene(
                    *scene_id,
                    KernelSceneData::new(&*scene, &*g_engine, self.cmd_pool.clone()),
                );
            }
            let scene_data = vxunwrap!(cmdss.get_mut_scene(scene_id));
            if need_update_shadow_makers_data {
                scene.update_shadow_makers_data(&mut scene_data.shadow_makers_data);
            }
            let models = scene.get_all_models();
            g_buffer_filler.begin_secondary(&mut scene_data.cmds.gbuff);
            scene.render(&mut scene_data.cmds.gbuff, frame_number);
            for (_, model) in &*models {
                let camera = vxunwrap!(scene.get_active_camera()).upgrade();
                let camera = vxunwrap!(camera);
                let camera = vxresult!(camera.read());
                task_index += 1;
                if task_index % self.kernels_count != self.index {
                    continue;
                }
                let model = model.upgrade();
                if model.is_none() {
                    continue;
                }
                let model = vxunwrap!(model);
                let mut model = vxresult!(model.write());
                if !model.is_rendarable() {
                    continue;
                }
                Object::update(&mut *model);
                Model::update(&mut *model, &*scene, &*camera);
                Object::render(&mut *model, &mut scene_data.cmds.gbuff, frame_number);
                if model.has_shadow() {
                    model.clear_light_visibilities();
                    for (_, shm) in &mut scene_data.shadow_makers_data {
                        shm.check_shadowability(&mut *model);
                    }
                }
            }
            scene_data.cmds.gbuff.end();
        }
    }

    pub fn shadow(&mut self) {
        let geng = vxresult!(self.g_engine.read());
        let frame_number = geng.get_frame_number();
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let scenes = vxresult!(scenes.read());
        let mut cmdsss = vxresult!(self.cmd_buffers.lock());
        let cmdss = &mut cmdsss[frame_number];
        let mut task_index = 0usize;
        for (scene_id, scene) in &*scenes {
            let scene = scene.upgrade();
            if scene.is_none() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            if !cmdss.has_scene(scene_id) {
                cmdss.add_scene(
                    *scene_id,
                    KernelSceneData::new(&*scene, &*geng, self.cmd_pool.clone()),
                );
            }
            let scene_data = vxunwrap!(cmdss.get_mut_scene(scene_id));
            let models = scene.get_all_models();
            let shadow_makers = scene.get_shadow_makers();
            let shadower = vxresult!(self.shadower.read());
            for (id, l) in &*shadow_makers {
                {
                    if let Some(lights) = scene_data.cmds.lights.get_mut(id) { // todo 
                        shadower.begin_secondary_shadow_mappers(lights);
                        continue;
                    }
                }
                let l = vxresult!(l.read());
                let mut lights = l.create_shadow_mapper_commands(&geng, &self.cmd_pool);
                shadower.begin_secondary_shadow_mappers(&mut lights);
                scene_data.cmds.lights.insert(*id, lights);
            }
            for (_, model) in &*models {
                task_index += 1;
                if task_index % self.kernels_count != self.index {
                    continue;
                }
                let model = model.upgrade();
                if model.is_none() {
                    continue;
                }
                let model = vxunwrap!(model);
                let mut model = vxresult!(model.write());
                if !model.is_rendarable() {
                    continue;
                }
                if !model.has_shadow() {
                    continue;
                }
                model.update_light_visibilities();
                let light_visibilities = model.get_light_visibilities();
                for (_, l) in light_visibilities {
                    l.render();
                    model.render_shadow();
                } 
                ////////////////////////////////////////////////////
            }
            for (id, l) in &*shadow_makers {
                if let Some(lights) = scene_data.cmds.lights.get_mut(id) {
                    shadower.begin_secondary_shadow_mappers(lights);
                    continue;
                }
            }
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct PrimaryPassesCommands {
    shadow: CmdBuffer,
    shadow_semaphore: Arc<Semaphore>,
    gbuff: CmdBuffer,
    gbuff_semaphore: Arc<Semaphore>,
    resolver: CmdBuffer,
    resolver_secondary: CmdBuffer,
    resolver_semaphore: Arc<Semaphore>,
    deferred: CmdBuffer,
    deferred_secondary: CmdBuffer,
    deferred_semaphore: Arc<Semaphore>,
}

impl PrimaryPassesCommands {
    fn new(engine: &GraphicApiEngine, cmd_pool: Arc<CmdPool>) -> Self {
        let shadow = engine.create_primary_command_buffer(cmd_pool.clone());
        let shadow_semaphore = Arc::new(engine.create_semaphore());
        let gbuff = engine.create_primary_command_buffer(cmd_pool.clone());
        let gbuff_semaphore = Arc::new(engine.create_semaphore());
        let resolver = engine.create_primary_command_buffer(cmd_pool.clone());
        let resolver_secondary = engine.create_secondary_command_buffer(cmd_pool.clone());
        let resolver_semaphore = Arc::new(engine.create_semaphore());
        let deferred = engine.create_primary_command_buffer(cmd_pool.clone());
        let deferred_secondary = engine.create_secondary_command_buffer(cmd_pool.clone());
        let deferred_semaphore = Arc::new(engine.create_semaphore());
        Self {
            shadow,
            shadow_semaphore,
            gbuff,
            gbuff_semaphore,
            resolver,
            resolver_secondary,
            resolver_semaphore,
            deferred,
            deferred_secondary,
            deferred_semaphore,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct FrameData {
    scenes_commands: BTreeMap<Id, PrimaryPassesCommands>,
}

impl FrameData {
    fn new() -> Self {
        Self {
            scenes_commands: BTreeMap::new(),
        }
    }

    fn remove_scene(&mut self, id: &Id) {
        self.scenes_commands.remove(id);
    }

    fn has_scene(&self, id: &Id) -> bool {
        return self.scenes_commands.contains_key(id);
    }

    fn add_scene(&mut self, id: Id, primary_passes_commands: PrimaryPassesCommands) {
        self.scenes_commands.insert(id, primary_passes_commands);
    }

    fn get_mut_scene(&mut self, id: &Id) -> Option<&mut PrimaryPassesCommands> {
        return self.scenes_commands.get_mut(id);
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Engine {
    kernels: Vec<Kernel>,
    engine: Arc<RwLock<GraphicApiEngine>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    cmd_pool: Arc<CmdPool>,
    g_buffer_filler: Arc<RwLock<GBufferFiller>>,
    deferred: Arc<Mutex<Deferred>>,
    resolver: Arc<Mutex<Resolver>>,
    shadower: Arc<RwLock<Shadower>>,
    cmdsss: Mutex<Vec<FrameData>>,
    cascaded_count: usize,
}

impl Engine {
    pub(crate) fn new(
        engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
        config: &Configurations,
    ) -> Self {
        let eng = engine.clone();
        let eng = vxresult!(eng.read());
        let scnmgr = scene_manager.clone();
        let scnmgr = vxresult!(scnmgr.read());
        let g_buffer_filler = GBufferFiller::new(&eng);
        let resolver = Resolver::new(&eng, &g_buffer_filler, &*scnmgr);
        let deferred = Arc::new(Mutex::new(Deferred::new(&eng, &*scnmgr, &resolver)));
        let resolver = Arc::new(Mutex::new(resolver));
        let g_buffer_filler = Arc::new(RwLock::new(g_buffer_filler));
        let shadower = Arc::new(RwLock::new(Shadower::new(&eng, config)));
        let kernels_count = num_cpus::get();
        let mut kernels = Vec::new();
        let cascaded_count = config.cascaded_shadows_count as usize;
        for ki in 0..kernels_count {
            kernels.push(Kernel::new(
                ki,
                kernels_count,
                engine.clone(),
                scene_manager.clone(),
                g_buffer_filler.clone(),
                shadower.clone(),
            ));
        }
        kernels.shrink_to_fit();
        let cmd_pool = eng.create_command_pool();
        let frames_count = eng.get_frames_count();
        let mut cmdsss = Vec::new();
        for _ in 0..frames_count {
            cmdsss.push(FrameData::new());
        }
        cmdsss.shrink_to_fit();
        let cmdsss = Mutex::new(cmdsss);
        Engine {
            kernels,
            engine,
            scene_manager,
            cmd_pool,
            cmdsss,
            g_buffer_filler,
            deferred,
            cascaded_count,
            resolver,
            shadower,
        }
    }

    pub(crate) fn render(&self) {
        vxresult!(self.engine.write()).start_rendering();
        self.update_scenes();
        for k in &self.kernels {
            k.start_rendering();
        }
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let scenes = vxresult!(scenes.read());
        let deferred = vxresult!(self.deferred.lock());
        let resolver = vxresult!(self.resolver.lock());
        let engine = vxresult!(self.engine.read());
        let mut last_semaphore = engine.get_starting_semaphore().clone();
        let framebuffer = engine.get_current_framebuffer();
        let frame_number = engine.get_frame_number();
        let cmdss = &mut vxresult!(self.cmdsss.lock())[frame_number];
        for (scene_id, scene) in &*scenes {
            let scene = scene.upgrade();
            if scene.is_none() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            if !cmdss.has_scene(scene_id) {
                cmdss.add_scene(
                    *scene_id,
                    PrimaryPassesCommands::new(&*engine, self.cmd_pool.clone()),
                );
            }
            let cmds = vxunwrap!(cmdss.get_mut_scene(scene_id));
            {
                let cmd = &mut cmds.gbuff;
                cmd.begin();
                vxresult!(self.g_buffer_filler.read()).begin_primary(cmd);
            }
            resolver.begin_primary(&mut cmds.resolver);
            resolver.begin_secondary(&mut cmds.resolver_secondary, frame_number);
            {
                let cmd = &mut cmds.resolver;
                cmd.exe_cmd(&cmds.resolver_secondary);
                cmd.end_render_pass();
                cmd.end();
            }
            {
                let cmd = &mut cmds.deferred;
                cmd.begin();
                framebuffer.begin(cmd);
            }
            {
                let cmd = &mut cmds.deferred_secondary;
                cmd.begin_secondary(&*framebuffer);
                deferred.render(cmd, frame_number);
            }
        }
        for k in &self.kernels {
            k.wait_rendering();
        }
        for (scene_id, scene) in &*scenes {
            let scene = scene.upgrade();
            if scene.is_none() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            for k in &self.kernels {
                let frame_datas = vxresult!(k.frame_datas.lock());
                let frame_data = &frame_datas[frame_number];
                let scene_frame_data = frame_data.get_scene(scene_id);
                if scene_frame_data.is_none() {
                    cmdss.remove_scene(scene_id);
                    continue;
                }
                let scene_frame_data = vxunwrap!(scene_frame_data);
                scene.update_shadow_makers_with_data(&scene_frame_data.shadow_makers_data);
            }
            scene.update_shadow_makers();
        }
        for k in &self.kernels {
            k.start_shadowing();
        }
        for k in &self.kernels {
            k.wait_shadowing();
        }
        for (scene_id, scene) in &*scenes {
            let scene = scene.upgrade();
            if scene.is_none() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove_scene(scene_id);
                continue;
            }
            let mut kcmdsgbuffdatas = Vec::new();
            for k in &self.kernels {
                let frame_datas = vxresult!(k.frame_datas.lock());
                let frame_data = &frame_datas[frame_number];
                let scene_frame_data = frame_data.get_scene(scene_id);
                if scene_frame_data.is_none() {
                    cmdss.remove_scene(scene_id);
                    continue;
                }
                let scene_frame_data = vxunwrap!(scene_frame_data);
                kcmdsgbuffdatas.push(scene_frame_data.cmds.gbuff.get_data());
            }
            let cmds = cmdss.get_mut_scene(scene_id);
            if cmds.is_none() {
                continue;
            }
            let cmds = vxunwrap!(cmds);
            {
                let cmd = &mut cmds.gbuff;
                cmd.exe_cmds_with_data(&kcmdsgbuffdatas);
                cmd.end_render_pass();
                cmd.end();
            }
            {
                let cmd = &mut cmds.deferred_secondary;
                scene.render_deferred(cmd, frame_number);
                cmd.render_deferred();
                cmd.end();
            }
            {
                let cmd = &mut cmds.deferred;
                cmd.exe_cmds_with_data(&[cmds.deferred_secondary.get_data()]);
                cmd.end_render_pass();
                cmd.end();
            }
            engine.submit(&last_semaphore, &cmds.gbuff, &cmds.gbuff_semaphore);
            engine.submit(
                &cmds.gbuff_semaphore,
                &cmds.resolver,
                &cmds.resolver_semaphore,
            );
            engine.submit(
                &cmds.resolver_semaphore,
                &cmds.deferred,
                &cmds.deferred_semaphore,
            );
            last_semaphore = cmds.deferred_semaphore.clone();
        }
        engine.end(&last_semaphore);
    }

    fn update_scenes(&self) {
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let mut scenes = vxresult!(scenes.write());
        let mut ids = Vec::new();
        {
            for (id, scene) in &*scenes {
                let scene = scene.upgrade();
                if scene.is_none() {
                    ids.push(*id);
                    continue;
                }
                let scene = vxunwrap!(&scene);
                let mut scene = vxresult!(scene.write());
                if !scene.is_rendarable() {
                    continue;
                }
                scene.update();
                scene.clean();
            }
        }
        for id in ids {
            scenes.remove(&id);
        }
    }
}
