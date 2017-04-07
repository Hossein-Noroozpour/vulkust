use std::ptr::null_mut;
use std::sync::Arc;

use super::super::render::engine::EngineTrait;
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::instance::Instance;
use super::surface::Surface;
use super::device::physical::Physical as PhysicalDevice;
use super::device::logical::Logical as LogicalDevice;
use super::swapchain::Swapchain;
use super::image::view::View as ImageView;
use super::render_pass::RenderPass;
use super::framebuffer::Framebuffer;
use super::command::pool::Pool as CmdPool;
// for the triangle
use super::buffer::Buffer;
use super::buffer::uniform::Uniform;
use std::mem::transmute;


pub struct Engine<CoreApp> where CoreApp: ApplicationTrait {
    pub core_app: *mut CoreApp,
    pub os_app: *mut OsApplication<CoreApp>,
    pub instance: Option<Arc<Instance>>,
    pub surface: Option<Arc<Surface>>,
    pub physical_device: Option<Arc<PhysicalDevice>>,
    pub logical_device: Option<Arc<LogicalDevice>>,
    pub swapchain: Option<Arc<Swapchain>>,
    pub depth_stencil_image_view: Option<Arc<ImageView>>,
    pub render_pass: Option<Arc<RenderPass>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub graphic_cmd_pool: Option<Arc<CmdPool>>,
    // for triangle
    pub mesh_buff: Option<Arc<Buffer>>,
    pub uniform: Option<Arc<Uniform>>,
}

impl<CoreApp> EngineTrait<CoreApp> for Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn new() -> Self {
        Engine {
            core_app: null_mut(),
            os_app: null_mut(),
            instance: None,
            surface: None,
            physical_device: None,
            logical_device: None,
            swapchain: None,
            depth_stencil_image_view: None,
            render_pass: None,
            framebuffers: Vec::new(),
            graphic_cmd_pool: None,
            mesh_buff: None,
            uniform: None,
        }
    }

    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }

    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>) {
        self.os_app = o;
    }

    fn initialize(&mut self) {
        let instance = Arc::new(Instance::new());
        #[cfg(target_os = "linux")]
        let surface = Arc::new(Surface::new(
            instance.clone(),
            unsafe { (*self.os_app).connection },
            unsafe { (*self.os_app).window }));
        let physical_device = Arc::new(PhysicalDevice::new(surface.clone()));
        let logical_device = Arc::new(LogicalDevice::new(physical_device.clone()));
        let swapchain = Arc::new(Swapchain::new(logical_device.clone()));
        let depth_stencil = Arc::new(ImageView::new_depth_stencil(logical_device.clone()));
        let render_pass = Arc::new(RenderPass::new(swapchain.clone()));
        for v in swapchain.image_views.clone() {
            self.framebuffers.push(Arc::new(Framebuffer::new(
                v.clone(), depth_stencil.clone(), render_pass.clone())));
        }
        let graphic_cmd_pool = Arc::new(CmdPool::new(logical_device.clone()));
        let vertices = [
             1.0f32,  1.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32,
			-1.0f32,  1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32,
			 0.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
        ];
        let indices = [
            0u32, 1u32, 2u32,
        ];
        let mesh_buff = Arc::new(Buffer::new(
            logical_device.clone(), graphic_cmd_pool.clone(),
            unsafe {transmute(vertices.as_ptr())}, vertices.len() as u32 * 4,
            unsafe {transmute(indices.as_ptr())}, indices.len() as u32 * 4
        ));
        let uniform_data = [
            1.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 1.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 1.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 1.0f32,

            1.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 1.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 1.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 1.0f32,

            1.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 1.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 1.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 1.0f32,
        ];
        let uniform = Arc::new(Uniform::new(
            logical_device.clone(), graphic_cmd_pool.clone(), uniform_data.len() as u32));
        uniform.update(unsafe { transmute(uniform_data.as_ptr()) });
        self.instance = Some(instance);
        self.surface = Some(surface);
        self.physical_device = Some(physical_device);
        self.logical_device = Some(logical_device);
        self.swapchain = Some(swapchain);
        self.depth_stencil_image_view = Some(depth_stencil);
        self.render_pass = Some(render_pass);
        self.graphic_cmd_pool = Some(graphic_cmd_pool);
        self.mesh_buff = Some(mesh_buff);
        self.uniform = Some(uniform);
    }

    fn update(&mut self) {
        // TODO
    }

    fn terminate(&mut self) {
        self.uniform = None;
        self.mesh_buff = None;
        self.graphic_cmd_pool = None;
        self.framebuffers.clear();
        self.render_pass = None;
        self.depth_stencil_image_view = None;
        self.swapchain = None;
        self.logical_device = None;
        self.physical_device = None;
        self.surface = None;
        self.instance = None;
    }
}
