use std::cell::{Ref, RefCell, RefMut};
use std::sync::Arc;

#[cfg(feature = "d3d12")]
use super::super::d3d12::engine::Engine;
#[cfg(any(feature = "metal", target_os = "macos"))]
use super::super::metal::engine::Engine;
#[cfg(all(not(feature = "metal"), not(feature = "d3d12"), not(target_os = "macos")))]
use super::super::vulkan::engine::Engine;

use super::super::core::application::ApplicationTrait;
use super::super::core::event::Event;
use super::super::system::os::OsApplication;
use super::super::system::os::ApplicationTrait as OsApp;
use super::scene::Scene;

pub type RenderEngine<CoreApp> = Engine<CoreApp>;

pub trait EngineTrait<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn new() -> Self;
    fn set_core_app(&mut self, c: &'static mut CoreApp);
    fn set_os_app(&mut self, o: &'static mut OsApplication<CoreApp>);
    fn initialize(&mut self);
    fn on_event(&mut self, e: Event);
    fn update(&mut self);
    fn terminate(&mut self);
    fn get_basic(&self) -> &Basic;
    fn get_mut_basic(&mut self) -> &mut Basic;
}

pub struct Basic {
    pub current_scene: Arc<RefCell<Scene>>,
}

impl Basic {
    pub fn new<CoreApp>(os_app: &mut OsApplication<CoreApp>) -> Self
    where CoreApp: ApplicationTrait {
        let screen_ratio = os_app.get_window_ratio() as f32;
        let transfer_cmd_pool = os_app.render_engine.transfer_cmd_pool.as_ref().unwrap().clone();
        Basic {
            current_scene: os_app.asset_manager.get_scene(0, screen_ratio, transfer_cmd_pool),
        }
    }

    pub fn get_mut_current_scene(&mut self) -> RefMut<Scene + 'static> {
        self.current_scene.borrow_mut()
    }

    pub fn get_current_scene(&self) -> Ref<Scene + 'static> {
        self.current_scene.borrow()
    }
}
