use super::super::core::types::Id;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::gapi::GraphicApiEngine;
use super::model::Model;
use super::object::Object;
use super::deferred::Deferred;
use super::scene::Manager as SceneManager;
use super::sync::Semaphore;
use num_cpus;
use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelPassesCommands {
    shadow: CmdBuffer,
    gbuff: CmdBuffer,
    is_filled: bool,
}

impl KernelPassesCommands {
    fn new(engine: &GraphicApiEngine, cmd_pool: Arc<CmdPool>) -> Self {
        let shadow = engine.create_secondary_command_buffer(cmd_pool.clone());
        let gbuff = engine.create_secondary_command_buffer(cmd_pool.clone());
        let is_filled = false;
        Self {
            shadow,
            gbuff,
            is_filled,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct KernelFrameData {
    scenes_commands: BTreeMap<Id, KernelPassesCommands>,
}

impl KernelFrameData {
    fn new() -> Self {
        Self {
            scenes_commands: BTreeMap::new()
        }
    }

    fn remove_scene(&mut self, id: &Id) {
        self.scenes_commands.remove(id);
    }

    fn has_scene(&self, id: &Id) -> bool {
        return self.scenes_commands.contains_key(id);
    }

    fn add_scene(&mut self, id: Id, kernel_passes_commands: KernelPassesCommands) {
        self.scenes_commands.insert(id, kernel_passes_commands);
    }

    fn get_mut_scene(&mut self, id: &Id) -> Option<&mut KernelPassesCommands> {
        return self.scenes_commands.get_mut(id);
    }

    fn get_scene(&self, id: &Id) -> Option<&KernelPassesCommands> {
        return self.scenes_commands.get(id);
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct Kernel {
    loop_signaler: Sender<bool>,
    ready_notifier: Receiver<()>,
    handle: JoinHandle<()>,
    cmd_buffers: Arc<Mutex<Vec<KernelFrameData>>>,
}

impl Kernel {
    pub fn new(
        index: usize,
        kernels_count: usize,
        engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
    ) -> Self {
        let (loop_signaler, rcv) = channel();
        let (ready_sig, ready_notifier) = channel();
        let cmd_buffers = Arc::new(Mutex::new(Vec::new()));
        let cmdbuffs = cmd_buffers.clone();
        let handle = spawn(move || {
            let mut renderer = Renderer::new(index, kernels_count, cmdbuffs, engine, scene_manager);
            while vxresult!(rcv.recv()) {
                renderer.render();
                vxresult!(ready_sig.send(()));
            }
            vxresult!(ready_sig.send(()));
        });
        Self {
            loop_signaler,
            ready_notifier,
            handle,
            cmd_buffers,
        }
    }

    fn start_rendering(&self) {
        vxresult!(self.loop_signaler.send(true));
    }

    fn wait_rendering(&self) {
        vxresult!(self.ready_notifier.recv());
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        vxresult!(self.loop_signaler.send(false));
        vxresult!(self.ready_notifier.recv());
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
}

impl Renderer {
    pub fn new(
        index: usize,
        kernels_count: usize,
        cmd_buffers: Arc<Mutex<Vec<KernelFrameData>>>,
        g_engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
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
        }
    }

    pub fn render(&mut self) {
        let g_engine = vxresult!(self.g_engine.read());
        let gbuff_framebuffer = g_engine.get_gbuff_framebuffer();
        let gbuff_pipeline = g_engine.get_gbuff_pipeline();
        let frame_number = g_engine.get_frame_number();
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
                cmdss.add_scene(*scene_id, KernelPassesCommands::new(&*g_engine, self.cmd_pool.clone()));
            }
            let cmds = vxunwrap!(cmdss.get_mut_scene(scene_id));
            let models = scene.get_all_models();
            cmds.gbuff.begin_secondary(&gbuff_framebuffer);
            // cmds[SECONDARY_SHADOW_PASS_INDEX].begin_secondary();
            cmds.gbuff.set_viewport(&gbuff_framebuffer.viewport);
            cmds.gbuff.set_scissor(&gbuff_framebuffer.scissor);
            cmds.gbuff.bind_pipeline(&gbuff_pipeline);
            scene.render(&mut cmds.gbuff, frame_number);
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
                Object::update(&mut *model);
                Model::update(&mut *model, &*scene);
                Object::render(
                    &mut *model,
                    &mut cmds.gbuff,
                    frame_number,
                );
                cmds.is_filled = true;
            }
            cmds.gbuff.end();
            // cmds[SECONDARY_SHADOW_PASS_INDEX].end();
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct PrimaryPassesCommands {
    shadow: CmdBuffer,
    shadow_semaphore: Arc<Semaphore>,
    gbuff: CmdBuffer,
    gbuff_semaphore: Arc<Semaphore>,
    deferred: CmdBuffer,
    deferred_secondary: CmdBuffer,
    deferred_semaphore: Arc<Semaphore>,
}

impl PrimaryPassesCommands {
    fn new(engine: &GraphicApiEngine, cmd_pool: Arc<CmdPool>) -> Self {
        let shadow = engine.create_primary_command_buffer(cmd_pool.clone());
        let gbuff = engine.create_primary_command_buffer(cmd_pool.clone());
        let deferred = engine.create_primary_command_buffer(cmd_pool.clone());
        let deferred_secondary = engine.create_secondary_command_buffer(cmd_pool.clone());
        let shadow_semaphore = Arc::new(engine.create_semaphore());
        let gbuff_semaphore = Arc::new(engine.create_semaphore());
        let deferred_semaphore = Arc::new(engine.create_semaphore());
        Self {
            shadow,
            shadow_semaphore,
            gbuff,
            gbuff_semaphore,
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
    fn new(engine: &GraphicApiEngine) -> Self {
        Self {
            scenes_commands: BTreeMap::new()
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

    fn get_scene(&mut self, id: &Id) -> Option<&PrimaryPassesCommands> {
        return self.scenes_commands.get(id);
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Engine {
    kernels: Vec<Kernel>,
    engine: Arc<RwLock<GraphicApiEngine>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    cmd_pool: Arc<CmdPool>,
    deferred: Mutex<Deferred>,
    cmdsss: Mutex<Vec<FrameData>>,
}

impl Engine {
    pub(crate) fn new(
        engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
    ) -> Self {
        let kernels_count = num_cpus::get();
        let mut kernels = Vec::new();
        for ki in 0..kernels_count {
            kernels.push(Kernel::new(
                ki,
                kernels_count,
                engine.clone(),
                scene_manager.clone(),
            ));
        }
        kernels.shrink_to_fit();
        let eng = engine.clone();
        let eng = vxresult!(eng.read());
        let cmd_pool = eng.create_command_pool();
        let frames_count = eng.get_frames_count();
        let mut cmdsss = Vec::new();
        for _ in 0..frames_count {
            cmdsss.push(FrameData::new(&*eng));
        }
        cmdsss.shrink_to_fit();
        let cmdsss = Mutex::new(cmdsss);
        let deferred = Mutex::new(Deferred::new(&eng, &*vxresult!(scene_manager.read())));
        Engine {
            kernels,
            engine,
            scene_manager,
            cmd_pool,
            cmdsss,
            deferred,
        }
    }

    pub(crate) fn render(&self) {
        vxresult!(self.engine.write()).start_rendering();
        let deferred = vxresult!(self.deferred.lock());
        self.update_scenes();
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let scenes = vxresult!(scenes.read());
        for k in &self.kernels {
            k.start_rendering();
        }
        let engine = vxresult!(self.engine.read());
        let last_semaphore = engine.get_starting_semaphore().clone();
        let gbuff_framebuffer = engine.get_gbuff_framebuffer();
        let gbuff_pipeline = engine.get_gbuff_pipeline();
        let deferred_framebuffer = engine.get_deferred_framebuffer();
        let deferred_pipeline = engine.get_deferred_pipeline();
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
                cmdss.add_scene(*scene_id, PrimaryPassesCommands::new(&*engine, self.cmd_pool.clone()));
            }
            let cmds = vxunwrap!(cmdss.get_mut_scene(scene_id));
            {
                let cmd = &mut cmds.gbuff;
                cmd.begin();
                gbuff_framebuffer.begin(cmd);
            }
            {
                let cmd = &mut cmds.deferred;
                cmd.begin();
                deferred_framebuffer.begin(cmd);
            }
            {
                let cmd = &mut cmds.deferred_secondary;
                cmd.begin_secondary(&deferred_framebuffer);
                cmd.set_viewport(&deferred_framebuffer.viewport);
                cmd.set_scissor(&deferred_framebuffer.scissor);
                cmd.bind_pipeline(&deferred_pipeline);
                deferred.render(cmd, frame_number);
            }
        }
        for k in &self.kernels {
            k.wait_rendering();
        }
        'scenes: for (scene_id, scene) in &*scenes {
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
                let kcmdsss = vxresult!(k.cmd_buffers.lock());
                let kcmdss = &kcmdsss[frame_number];
                let kcmds = kcmdss.get_scene(scene_id);
                if kcmds.is_none() {
                    cmdss.remove_scene(scene_id);
                    continue 'scenes;
                }
                let kcmds = vxunwrap!(kcmds);
                if !kcmds.is_filled {
                    continue;
                }
                kcmdsgbuffdatas.push(kcmds.gbuff.get_data());
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
                if scene.is_rendarable() {
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
