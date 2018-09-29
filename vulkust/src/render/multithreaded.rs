use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::gapi::GraphicApiEngine;
use num_cpus;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};

const PASSES_COUNT: usize = 3;
const GBUFF_PASS_INDEX: usize = 0;
const FORWARD_TRANSPARENT_PASS_INDEX: usize = 1;
const SHADOW_PASS_INDEX: usize = 2;

#[cfg_attr(debug_mode, derive(Debug))]
struct Kernel {
    loop_signaler: Sender<bool>,
    ready_notifier: Receiver<()>,
    handle: JoinHandle<()>,
    cmd_buffers: Arc<Mutex<Vec<Vec<CmdBuffer>>>>, // frame -> pass
}

impl Kernel {
    pub fn new(index: usize, kernels_count: usize, engine: Arc<RwLock<GraphicApiEngine>>) -> Self {
        let (loop_signaler, rcv) = channel();
        let (ready_sig, ready_notifier) = channel();
        let cmd_buffers = Arc::new(Mutex::new(Vec::new()));
        let cmdbuffs = cmd_buffers.clone();
        let handle = spawn(move || {
            let mut renderer = Renderer::new(index, kernels_count, cmdbuffs, engine);
            while vxresult!(rcv.recv()) {
                renderer.render();
                ready_sig.send(());
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
    g_engine: Arc<RwLock<GraphicApiEngine>>,
    cmd_pool: Arc<CmdPool>,
    cmd_buffers: Arc<Mutex<Vec<Vec<CmdBuffer>>>>, // frame -> pass
}

impl Renderer {
    pub fn new(
        index: usize,
        kernels_count: usize,
        cmd_buffers: Arc<Mutex<Vec<Vec<CmdBuffer>>>>,
        g_engine: Arc<RwLock<GraphicApiEngine>>,
    ) -> Self {
        let eng = g_engine.clone();
        let eng = vxresult!(eng.read());
        let cmd_pool = eng.create_command_pool();
        let cmdss = cmd_buffers.clone();
        let mut cmdss = vxresult!(cmdss.lock());
        let frames_count = eng.get_frames_count();
        for _ in 0..frames_count {
            let mut cmds = Vec::new();
            for _ in 0..PASSES_COUNT {
                cmds.push(eng.create_command_buffer(cmd_pool.clone()));
            }
            cmds.shrink_to_fit();
            cmdss.push(cmds);
        }
        cmdss.shrink_to_fit();
        Renderer {
            index,
            g_engine,
            cmd_pool,
            cmd_buffers,
        }
    }

    pub fn render(&mut self) {}
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Engine {
    kernels: Vec<Kernel>,
}

impl Engine {
    pub fn new(engine: Arc<RwLock<GraphicApiEngine>>) -> Self {
        let kernels_count = num_cpus::get();
        let mut kernels = Vec::new();
        for ki in 0..kernels_count {
            kernels.push(Kernel::new(ki, kernels_count, engine.clone()));
        }
        kernels.shrink_to_fit();
        Engine { kernels }
    }
}
