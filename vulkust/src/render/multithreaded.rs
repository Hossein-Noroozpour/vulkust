use super::super::core::types::Id;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::config::Configurations;
use super::deferred::Deferred;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::model::Model;
use super::object::Object;
use super::resolver::Resolver;
use super::scene::{Manager as SceneManager, Scene};
use super::shadower::Shadower;
use super::sync::Semaphore;
use num_cpus;
use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock, Weak};
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
            let mut renderer = Renderer::new(
                index,
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
        let scenes = vxresult!(scenes.read());
        let g_buffer_filler = vxresult!(self.g_buffer_filler.read());
        let shadower = vxresult!(self.shadower.read());
        for (_, scene) in &*scenes {
            if let Some(scene) = scene.upgrade() {
                vxresult!(scene.read()).render_gbuffer_shadow_maps(&*geng, &self.cmd_pool, &*g_buffer_filler, &*shadower, self.index);
            }
        }
    }

    pub fn shadow(&mut self) {
        let frame_number = vxresult!(self.g_engine.read()).get_frame_number();
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let scenes = vxresult!(scenes.read());
        for (_, scene) in &*scenes {
            if let Some(scene) = scene.upgrade() {
                let scene = vxresult!(scene.read());
                scene.render_shadow_maps(self.index, frame_number);
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
    deferred: Arc<Mutex<Deferred>>,
    resolver: Arc<Mutex<Resolver>>,
    shadower: Arc<RwLock<Shadower>>,
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
        let scnmgr = vxresult!(self.scene_manager.read());
        let scenes = scnmgr.get_scenes();
        let scenes = vxresult!(scenes.read());
        let deferred = vxresult!(self.deferred.lock());
        let resolver = vxresult!(self.resolver.lock());
        let mut last_semaphore = engine.get_starting_semaphore().clone();
        let framebuffer = engine.get_current_framebuffer();
        // for (scene_id, scene) in &*scenes {
        //     let scene = scene.upgrade();
        //     if scene.is_none() {
        //         cmdss.remove_scene(scene_id);
        //         continue;
        //     }
        //     let scene = vxunwrap!(scene);
        //     let scene = vxresult!(scene.read());
        //     if !scene.is_rendarable() {
        //         cmdss.remove_scene(scene_id);
        //         continue;
        //     }
        //     if !cmdss.has_scene(scene_id) {
        //         cmdss.add_scene(
        //             *scene_id,
        //             PrimaryPassesCommands::new(&*engine, self.cmd_pool.clone()),
        //         );
        //     }
        //     let cmds = vxunwrap!(cmdss.get_mut_scene(scene_id));
        //     {
        //         let cmd = &mut cmds.gbuff;
        //         cmd.begin();
        //         vxresult!(self.g_buffer_filler.read()).begin_primary(cmd);
        //     }
        //     resolver.begin_primary(&mut cmds.resolver);
        //     resolver.begin_secondary(&mut cmds.resolver_secondary, frame_number);
        //     {
        //         let cmd = &mut cmds.resolver;
        //         cmd.exe_cmd(&cmds.resolver_secondary);
        //         cmd.end_render_pass();
        //         cmd.end();
        //     }
        //     {
        //         let cmd = &mut cmds.deferred;
        //         cmd.begin();
        //         framebuffer.begin(cmd);
        //     }
        //     {
        //         let cmd = &mut cmds.deferred_secondary;
        //         cmd.begin_secondary(&*framebuffer);
        //         deferred.render(cmd, frame_number);
        //     }
        // }
        for k in &self.kernels {
            k.wait_rendering();
        }
        // for (scene_id, scene) in &*scenes {
        //     let scene = scene.upgrade();
        //     if scene.is_none() {
        //         cmdss.remove_scene(scene_id);
        //         continue;
        //     }
        //     let scene = vxunwrap!(scene);
        //     let scene = vxresult!(scene.read());
        //     if !scene.is_rendarable() {
        //         cmdss.remove_scene(scene_id);
        //         continue;
        //     }
        //     for k in &self.kernels {
        //         let frame_datas = vxresult!(k.frame_datas.lock());
        //         let frame_data = &frame_datas[frame_number];
        //         let scene_frame_data = frame_data.get_scene(scene_id);
        //         if scene_frame_data.is_none() {
        //             cmdss.remove_scene(scene_id);
        //             continue;
        //         }
        //         let scene_frame_data = vxunwrap!(scene_frame_data);
        //         scene.update_shadow_makers_with_data(&scene_frame_data.shadow_makers_data);
        //     }
        //     scene.update_shadow_makers();
        // }
        for k in &self.kernels {
            k.start_shadowing();
        }
        for k in &self.kernels {
            k.wait_shadowing();
        }
        // for (scene_id, scene) in &*scenes {
        //     let scene = scene.upgrade();
        //     if scene.is_none() {
        //         cmdss.remove_scene(scene_id);
        //         continue;
        //     }
        //     let scene = vxunwrap!(scene);
        //     let scene = vxresult!(scene.read());
        //     if !scene.is_rendarable() {
        //         cmdss.remove_scene(scene_id);
        //         continue;
        //     }
        //     let mut kcmdsgbuffdatas = Vec::new();
        //     for k in &self.kernels {
        //         let frame_datas = vxresult!(k.frame_datas.lock());
        //         let frame_data = &frame_datas[frame_number];
        //         let scene_frame_data = frame_data.get_scene(scene_id);
        //         if scene_frame_data.is_none() {
        //             cmdss.remove_scene(scene_id);
        //             continue;
        //         }
        //         let scene_frame_data = vxunwrap!(scene_frame_data);
        //         kcmdsgbuffdatas.push(scene_frame_data.cmds.gbuff.get_data());
        //     }
        //     let cmds = cmdss.get_mut_scene(scene_id);
        //     if cmds.is_none() {
        //         continue;
        //     }
        //     let cmds = vxunwrap!(cmds);
        //     {
        //         let cmd = &mut cmds.gbuff;
        //         cmd.exe_cmds_with_data(&kcmdsgbuffdatas);
        //         cmd.end_render_pass();
        //         cmd.end();
        //     }
        //     {
        //         let cmd = &mut cmds.deferred_secondary;
        //         scene.render_deferred(cmd, frame_number);
        //         cmd.render_deferred();
        //         cmd.end();
        //     }
        //     {
        //         let cmd = &mut cmds.deferred;
        //         cmd.exe_cmds_with_data(&[cmds.deferred_secondary.get_data()]);
        //         cmd.end_render_pass();
        //         cmd.end();
        //     }
        //     engine.submit(&last_semaphore, &cmds.gbuff, &cmds.gbuff_semaphore);
        //     engine.submit(
        //         &cmds.gbuff_semaphore,
        //         &cmds.resolver,
        //         &cmds.resolver_semaphore,
        //     );
        //     engine.submit(
        //         &cmds.resolver_semaphore,
        //         &cmds.deferred,
        //         &cmds.deferred_semaphore,
        //     );
        //     last_semaphore = cmds.deferred_semaphore.clone();
        // }
        engine.end(&last_semaphore);
    }

    fn update_scenes(&self, frame_number: usize) {
        let scnmgr = vxresult!(self.scene_manager.read());
        let mut ids = Vec::new();
        for (id, scene) in &*vxresult!(scnmgr.get_scenes().read()) {
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
        let mut scenes = vxresult!(scnmgr.get_scenes().write());
        for id in ids {
            scenes.remove(&id);
        }
    }
}
