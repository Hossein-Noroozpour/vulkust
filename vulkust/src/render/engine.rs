#[cfg(feature = "d3d12")]
use super::super::d3d12::engine::Engine;
#[cfg(any(feature = "metal", target_os = "macos"))]
use super::super::metal::engine::Engine;
#[cfg(all(not(feature = "metal"), not(feature = "d3d12"), not(target_os = "macos")))]
use super::super::vulkan::engine::Engine;

use super::super::core::application::ApplicationTrait;
use super::super::core::event::Event;
use super::super::system::os::OsApplication;
use super::scene::Scene;

pub type RenderEngine<CoreApp> = Engine<CoreApp>;

pub trait EngineTrait<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn new() -> Self;
    fn set_core_app(&mut self, c: *mut CoreApp);
    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>);
    fn initialize(&mut self);
    fn on_event(&mut self, e: Event);
    fn update(&mut self);
    fn terminate(&mut self);
    fn get_basic(&self) -> &Basic;
    fn get_mut_basic(&mut self) -> &mut Basic;
}

pub struct Basic {
    pub current_scene: Scene,
}

impl Basic {
    pub fn new() -> Self {
        Basic {
            current_scene: Scene::new(),
        }
    }

    pub fn get_mut_current_scene(&mut self) -> &mut Scene {
        &mut self.current_scene
    }

    pub fn get_current_scene(&self) -> &Scene {
        &self.current_scene
    }
}
