#[cfg(feature = "d3d12")]
use super::super::d3d12::engine::Engine;
#[cfg(any(feature = "metal", target_os = "macos"))]
use super::super::metal::engine::Engine;
#[cfg(all(not(feature = "metal"), not(feature = "d3d12"), not(target_os = "macos")))]
use super::super::vulkan::engine::Engine;

use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;

pub type RenderEngine<CoreApp> = Engine<CoreApp>;

pub trait EngineTrait<CoreApp> where CoreApp: ApplicationTrait {
    fn new() -> Self;
    fn set_core_app(&mut self, c: *mut CoreApp);
    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>);
    fn initialize(&mut self);
    fn update(&mut self);
    fn terminate(&mut self);
}
