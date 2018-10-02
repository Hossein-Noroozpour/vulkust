use super::super::core::types::Id;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::gapi::GraphicApiEngine;
use super::model::Model;
use super::object::Object;
use super::scene::Manager as SceneManager;
use num_cpus;
use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};

const SECONDARY_PASSES_COUNT: usize = 2;
const SECONDARY_SHADOW_PASS_INDEX: usize = 0;
const SECONDARY_GBUFF_PASS_INDEX: usize = 1;

const PRIMARY_PASSES_COUNT: usize = 3; // add transparent and soft shadow in future
const PRIMARY_SHADOW_PASS_INDEX: usize = 0;
const PRIMARY_GBUFF_PASS_INDEX: usize = 1;
// soft shadow places in here
const PRIMARY_DEFERRED_PASS_INDEX: usize = 2;
// forward trnasparent places in here

#[cfg_attr(debug_mode, derive(Debug))]
struct Kernel {
    loop_signaler: Sender<bool>,
    ready_notifier: Receiver<()>,
    handle: JoinHandle<()>,
    cmd_buffers: Arc<Mutex<Vec<BTreeMap<Id, Vec<CmdBuffer>>>>>, // frame->scenes->pass
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
    cmd_buffers: Arc<Mutex<Vec<BTreeMap<Id, Vec<CmdBuffer>>>>>, // frame->scene->pass
}

impl Renderer {
    pub fn new(
        index: usize,
        kernels_count: usize,
        cmd_buffers: Arc<Mutex<Vec<BTreeMap<Id, Vec<CmdBuffer>>>>>,
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
            cmdsss.push(BTreeMap::new());
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
                cmdss.remove(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove(scene_id);
                continue;
            }
            if !cmdss.contains_key(scene_id) {
                let mut cmds = Vec::new();
                for _ in 0..SECONDARY_PASSES_COUNT {
                    cmds.push(g_engine.create_secondary_command_buffer(self.cmd_pool.clone()));
                }
                cmds.shrink_to_fit();
                cmdss.insert(*scene_id, cmds);
            }
            let cmds = vxunwrap!(cmdss.get_mut(scene_id));
            let models = scene.get_all_models();
            cmds[SECONDARY_GBUFF_PASS_INDEX].begin_secondary(&gbuff_framebuffer);
            // cmds[SECONDARY_SHADOW_PASS_INDEX].begin_secondary();
            cmds[SECONDARY_GBUFF_PASS_INDEX].set_viewport(&gbuff_framebuffer.viewport);
            cmds[SECONDARY_GBUFF_PASS_INDEX].set_scissor(&gbuff_framebuffer.scissor);
            cmds[SECONDARY_GBUFF_PASS_INDEX].bind_pipeline(&gbuff_pipeline);
            scene.render(&mut cmds[SECONDARY_GBUFF_PASS_INDEX], frame_number);
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
                    &mut cmds[SECONDARY_GBUFF_PASS_INDEX],
                    frame_number,
                );
            }
            cmds[SECONDARY_GBUFF_PASS_INDEX].end();
            // cmds[SECONDARY_SHADOW_PASS_INDEX].end();
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Engine {
    kernels: Vec<Kernel>,
    engine: Arc<RwLock<GraphicApiEngine>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    cmd_pool: Arc<CmdPool>,
    cmdsss: Mutex<Vec<BTreeMap<Id, Vec<CmdBuffer>>>>, // frame->scene->pass
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
            cmdsss.push(BTreeMap::new());
        }
        cmdsss.shrink_to_fit();
        let cmdsss = Mutex::new(cmdsss);
        Engine {
            kernels,
            engine,
            scene_manager,
            cmd_pool,
            cmdsss,
        }
    }

    pub(crate) fn render(&self) {
        self.update_scenes();
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let scenes = vxresult!(scenes.read());
        for k in &self.kernels {
            k.start_rendering();
        }
        let engine = vxresult!(self.engine.read());
        let gbuff_framebuffer = engine.get_gbuff_framebuffer();
        let gbuff_pipeline = engine.get_gbuff_pipeline();
        let deferred_framebuffer = engine.get_deferred_framebuffer();
        let deferred_pipeline = engine.get_deferred_pipeline();
        let frame_number = engine.get_frame_number();
        let cmdss = &mut vxresult!(self.cmdsss.lock())[frame_number];
        for (scene_id, scene) in &*scenes {
            let scene = scene.upgrade();
            if scene.is_none() {
                cmdss.remove(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove(scene_id);
                continue;
            }
            if !cmdss.contains_key(scene_id) {
                let mut cmds = Vec::new();
                for _ in 0..PRIMARY_PASSES_COUNT {
                    cmds.push(engine.create_primary_command_buffer(self.cmd_pool.clone()));
                }
                cmds.shrink_to_fit();
                cmdss.insert(*scene_id, cmds);
            }
            let cmds = vxunwrap!(cmdss.get_mut(scene_id));
            {
                let cmd = &mut cmds[PRIMARY_GBUFF_PASS_INDEX];
                cmd.begin();
                gbuff_framebuffer.begin(cmd);
            }
            {
                let cmd = &mut cmds[PRIMARY_DEFERRED_PASS_INDEX];
                cmd.begin();
                deferred_framebuffer.begin(cmd);
            }
        }
        for k in &self.kernels {
            k.wait_rendering();
        }
        'scenes: for (scene_id, scene) in &*scenes {
            let scene = scene.upgrade();
            if scene.is_none() {
                cmdss.remove(scene_id);
                continue;
            }
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            if !scene.is_rendarable() {
                cmdss.remove(scene_id);
                continue;
            }
            let mut kcmdsdatas = vec![Vec::new(), Vec::new()];
            for k in &self.kernels {
                let kcmdsss = vxresult!(k.cmd_buffers.lock());
                let kcmdss = &kcmdsss[frame_number];
                let kcmds = kcmdss.get(scene_id);
                if kcmds.is_none() {
                    cmdss.remove(scene_id);
                    continue 'scenes;
                }
                let kcmds = vxunwrap!(kcmds);
                kcmdsdatas[SECONDARY_GBUFF_PASS_INDEX]
                    .push(kcmds[SECONDARY_GBUFF_PASS_INDEX].get_data());
            }
            let cmds = cmdss.get_mut(scene_id);
            if cmds.is_none() {
                continue;
            }
            let cmds = vxunwrap!(cmds);
            vxlogi!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");
            {
                let cmd = &mut cmds[PRIMARY_GBUFF_PASS_INDEX];
                cmd.exe_cmds_with_data(&kcmdsdatas[SECONDARY_GBUFF_PASS_INDEX]);
                cmd.end_render_pass();
                cmd.end();
            }
            vxlogi!("rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr");
            {
                let cmd = &mut cmds[PRIMARY_DEFERRED_PASS_INDEX];
                cmd.end_render_pass();
                cmd.end();
            }
            vxlogi!("????????????????????????????????????????????????????????????????????????");
        }
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
