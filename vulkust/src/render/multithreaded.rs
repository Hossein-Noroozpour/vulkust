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

const PASSES_COUNT: usize = 2;
const SHADOW_PASS_INDEX: usize = 0;
const GBUFF_PASS_INDEX: usize = 1;

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
        });
        Self {
            loop_signaler,
            ready_notifier,
            handle,
            cmd_buffers,
        }
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
            if !cmdss.contains_key(scene_id) {
                let mut cmds = Vec::new();
                for _ in 0..PASSES_COUNT {
                    cmds.push(g_engine.create_command_buffer(self.cmd_pool.clone()));
                }
                cmds.shrink_to_fit();
                cmdss.insert(*scene_id, cmds);
            }
            let cmds = vxunwrap!(cmdss.get_mut(scene_id));
            let scene = vxunwrap!(scene);
            let scene = vxresult!(scene.read());
            let models = scene.get_models();
            for (_, model) in &*models {
                task_index += 1;
                if task_index % self.kernels_count != self.index {
                    continue;
                }
                let mut model = vxresult!(model.write());
                Object::update(&mut *model);
                Model::update(&mut *model, &*scene);
                // todo add command fillers in here
            }
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Engine {
    kernels: Vec<Kernel>,
}

impl Engine {
    pub fn new(
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
        Engine { kernels }
    }
}
