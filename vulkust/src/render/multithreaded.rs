use super::super::core::asset::Manager as AssetManager;
use super::command::Pool as CmdPool;
use super::config::Configurations;
use super::deferred::Deferred;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::resolver::Resolver;
use super::scene::Manager as SceneManager;
use super::shadower::Shadower;
use num_cpus;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};

#[cfg_attr(debug_mode, derive(Debug))]
struct Kernel {
    render_signal: Sender<bool>,
    render_wait: Receiver<()>,
    shadow_signal: Sender<()>,
    shadow_wait: Receiver<()>,
    handle: JoinHandle<()>,
}

impl Kernel {
    pub fn new(
        index: usize,
        engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
        g_buffer_filler: Arc<RwLock<GBufferFiller>>,
        shadower: Arc<RwLock<Shadower>>,
    ) -> Self {
        let (render_signal, render_receiver) = channel();
        let (render_ready, render_wait) = channel();
        let (shadow_signal, shadow_receiver) = channel();
        let (shadow_ready, shadow_wait) = channel();
        let handle = spawn(move || {
            let mut renderer =
                Renderer::new(index, engine, scene_manager, g_buffer_filler, shadower);
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
    g_engine: Arc<RwLock<GraphicApiEngine>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    cmd_pool: Arc<CmdPool>,
    g_buffer_filler: Arc<RwLock<GBufferFiller>>,
    shadower: Arc<RwLock<Shadower>>,
}

impl Renderer {
    pub fn new(
        index: usize,
        g_engine: Arc<RwLock<GraphicApiEngine>>,
        scene_manager: Arc<RwLock<SceneManager>>,
        g_buffer_filler: Arc<RwLock<GBufferFiller>>,
        shadower: Arc<RwLock<Shadower>>,
    ) -> Self {
        let eng = g_engine.clone();
        let eng = vxresult!(eng.read());
        let cmd_pool = eng.create_command_pool();
        Renderer {
            index,
            g_engine,
            scene_manager,
            cmd_pool,
            g_buffer_filler,
            shadower,
        }
    }

    pub fn render(&mut self) {
        let geng = vxresult!(self.g_engine.read());
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let g_buffer_filler = vxresult!(self.g_buffer_filler.read());
        let shadower = vxresult!(self.shadower.read());
        for (_, scene) in &*scenes {
            if let Some(scene) = scene.upgrade() {
                vxresult!(scene.read()).render_gbuffer_shadow_maps(
                    &*geng,
                    &self.cmd_pool,
                    &*g_buffer_filler,
                    &*shadower,
                    self.index,
                );
            }
        }
    }

    pub fn shadow(&mut self) {
        let frame_number = vxresult!(self.g_engine.read()).get_frame_number();
        let shadower = vxresult!(self.shadower.read());
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        for (_, scene) in &*scenes {
            if let Some(scene) = scene.upgrade() {
                let scene = vxresult!(scene.read());
                scene.render_shadow_maps(&*shadower, self.index, frame_number);
            }
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Engine {
    kernels: Vec<Kernel>,
    engine: Arc<RwLock<GraphicApiEngine>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    cmd_pool: Arc<CmdPool>,
    g_buffer_filler: Arc<RwLock<GBufferFiller>>,
    deferred: Arc<RwLock<Deferred>>,
    resolver: Arc<RwLock<Resolver>>,
    shadower: Arc<RwLock<Shadower>>,
}

impl Engine {
    pub(crate) fn new(
        engine: Arc<RwLock<GraphicApiEngine>>,
        asset_manager: &AssetManager,
        config: &Configurations,
    ) -> Self {
        let eng = engine.clone();
        let eng = vxresult!(eng.read());
        let scene_manager = asset_manager.get_scene_manager().clone();
        let mut texmgr = vxresult!(asset_manager.get_texture_manager().write());
        let g_buffer_filler = GBufferFiller::new(&eng, config);
        let resolver = Resolver::new(&eng, &g_buffer_filler, &mut *texmgr, config);
        let shadower = Shadower::new(&eng, &resolver, config, &mut *texmgr);
        let deferred = Arc::new(RwLock::new(Deferred::new(
            &eng, &resolver, &shadower, config,
        )));
        let resolver = Arc::new(RwLock::new(resolver));
        let g_buffer_filler = Arc::new(RwLock::new(g_buffer_filler));
        let shadower = Arc::new(RwLock::new(shadower));
        let kernels_count = num_cpus::get();
        let mut kernels = Vec::with_capacity(kernels_count);
        for ki in 0..kernels_count {
            kernels.push(Kernel::new(
                ki,
                engine.clone(),
                scene_manager.clone(),
                g_buffer_filler.clone(),
                shadower.clone(),
            ));
        }
        let cmd_pool = eng.create_command_pool();
        Engine {
            kernels,
            engine,
            scene_manager,
            cmd_pool,
            g_buffer_filler,
            deferred,
            resolver,
            shadower,
        }
    }

    pub(crate) fn render(&self) {
        vxresult!(self.engine.write()).start_rendering();
        let engine = vxresult!(self.engine.read());
        let frame_number = engine.get_frame_number();
        self.update_scenes(frame_number);
        for k in &self.kernels {
            k.start_rendering();
        }
        for k in &self.kernels {
            k.wait_rendering();
        }
        self.update_shadow_makers();
        for k in &self.kernels {
            k.start_shadowing();
        }
        for k in &self.kernels {
            k.wait_shadowing();
        }
        self.submit(&*engine);
    }

    fn update_scenes(&self, frame_number: usize) {
        let mut ids = Vec::new();
        for (id, scene) in vxresult!(self.scene_manager.read()).get_scenes() {
            if let Some(scene) = scene.upgrade() {
                let mut scene = vxresult!(scene.write());
                if !scene.is_rendarable() {
                    continue;
                }
                scene.update(frame_number);
                scene.clean();
            } else {
                ids.push(*id);
            }
        }
        let mut scnmgr = vxresult!(self.scene_manager.write());
        for id in ids {
            scnmgr.remove_with_id(&id);
        }
    }

    fn update_shadow_makers(&self) {
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        for (_, scene) in &*scenes {
            if let Some(scene) = scene.upgrade() {
                let mut scene = vxresult!(scene.write());
                if !scene.is_rendarable() {
                    continue;
                }
                scene.update_shadow_makers();
            }
        }
    }

    fn submit(&self, engine: &GraphicApiEngine) {
        let mut last_semaphore = engine.get_starting_semaphore().clone();
        let frame_number = engine.get_frame_number();
        vxresult!(self.resolver.write()).update(frame_number);
        vxresult!(self.deferred.write()).update(frame_number);
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let g_buffer_filler = vxresult!(self.g_buffer_filler.read());
        let resolver = vxresult!(self.resolver.read());
        let mut shadower = vxresult!(self.shadower.write());
        let deferred = vxresult!(self.deferred.read());
        for (_, scene) in &*scenes {
            if let Some(scene) = scene.upgrade() {
                let mut scene = vxresult!(scene.write());
                if !scene.is_rendarable() {
                    continue;
                }
                last_semaphore = scene
                    .submit(
                        engine,
                        &last_semaphore,
                        &self.cmd_pool,
                        &*g_buffer_filler,
                        &*resolver,
                        &mut *shadower,
                        &*deferred,
                    )
                    .clone();
            }
        }
        engine.end(&last_semaphore);
    }
}
