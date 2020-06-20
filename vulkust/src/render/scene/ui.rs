use super::super::super::core::gx3d::Gx3DReader;
use super::super::super::core::object::Object as CoreObject;
use super::super::super::core::types::Id;
use super::super::camera::Camera;
use super::super::command::Pool as CmdPool;
use super::super::deferred::Deferred;
use super::super::engine::Engine;
use super::super::g_buffer_filler::GBufferFiller;
use super::super::gapi::GraphicApiEngine;
use super::super::light::Light;
use super::super::model::Model;
use super::super::object::Object;
use super::super::shadower::Shadower;
use super::super::ssao::SSAO;
use super::super::sync::Semaphore;
use super::base_unlit::Base;
use super::{DefaultScene, Loadable, Scene};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Ui {
    base: Base,
}

impl Ui {}

impl CoreObject for Ui {
    fn get_id(&self) -> Id {
        self.base.get_id()
    }
}

impl Object for Ui {
    fn get_name(&self) -> Option<String> {
        self.base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
        vx_unimplemented!(); //it must update corresponding manager
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn is_renderable(&self) -> bool {
        return self.base.is_renderable();
    }
}

impl Scene for Ui {
    fn add_camera(&mut self, camera: Arc<RwLock<dyn Camera>>) {
        self.base.add_camera(camera)
    }

    fn add_model(&mut self, model: Arc<RwLock<dyn Model>>) {
        self.base.add_model(model);
    }

    fn add_light(&mut self, light: Arc<RwLock<dyn Light>>) {
        self.base.add_light(light);
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<dyn Camera>>> {
        return self.base.get_active_camera();
    }

    fn update(&mut self, frame_number: usize) {
        self.base.update(frame_number);
    }

    fn render_gbuffer_shadow_maps(
        &self,
        geng: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        shadower: &Shadower,
        kernel_index: usize,
    ) {
        self.base.render_gbuffer_shadow_maps(
            geng,
            cmd_pool,
            g_buffer_filler,
            shadower,
            kernel_index,
        );
    }

    fn update_shadow_makers(&self) {
        self.base.update_shadow_makers();
    }

    fn render_shadow_maps(&self, shadower: &Shadower, kernel_index: usize, frame_number: usize) {
        self.base
            .render_shadow_maps(shadower, kernel_index, frame_number);
    }

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<dyn Model>>> {
        return self.base.get_models();
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<dyn Model>>> {
        return self.base.get_all_models();
    }

    fn clean(&mut self) {
        self.base.clean();
    }

    fn submit(
        &mut self,
        geng: &GraphicApiEngine,
        sem: &Arc<Semaphore>,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        shadower: &mut Shadower,
        deferred: &Deferred,
        ssao: Option<&SSAO>,
    ) -> Arc<Semaphore> {
        return self.base.submit(
            geng,
            sem,
            cmd_pool,
            g_buffer_filler,
            shadower,
            deferred,
            ssao,
        );
    }
}

impl Loadable for Ui {
    fn new_with_gltf(engine: &Engine, scene: &gltf::Scene, data: &[u8]) -> Self {
        let base = Base::new_with_gltf(engine, scene, data);
        Ui { base }
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let base = Base::new_with_gx3d(engine, reader, my_id);
        Ui { base }
    }
}

impl DefaultScene for Ui {
    fn default(engine: &Engine) -> Self {
        let base = Base::default(engine);
        Ui { base }
    }
}
