use std::sync::{Arc, RwLock};
// use std::mem::transmute;
// use super::super::core::application::ApplicationTrait;
// use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
// use super::super::system::vulkan as vk;
// use super::super::util::cell::DebugCell;
// use super::buffer::Manager as BufferManager;
// use super::command::buffer::Buffer as CmdBuffer;
use super::command::pool::{Pool as CmdPool, Type as CmdPoolType};
use super::device::logical::Logical as LogicalDevice;
use super::device::physical::Physical as PhysicalDevice;
// use super::fence::Fence;
// use super::framebuffer::Framebuffer;
// use super::image::view::View as ImageView;
use super::instance::Instance;
use super::pipeline::Manager as PipelineManager;
// use super::render_pass::RenderPass;
use super::surface::Surface;
use super::swapchain::Swapchain;
// use super::synchronizer::semaphore::Semaphore;

pub struct Engine {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub physical_device: Arc<PhysicalDevice>,
    pub logical_device: Arc<LogicalDevice>,
    pub swapchain: Arc<Swapchain>,
    // pub depth_stencil_image_view: Option<Arc<ImageView>>,
    // pub render_pass: Option<Arc<RenderPass>>,
    // pub framebuffers: Vec<Arc<Framebuffer>>,
    pub graphic_cmd_pool: Arc<CmdPool>,
    pub compute_cmd_pool: Arc<CmdPool>,
    // pub transfer_cmd_pool: Option<Arc<CmdPool>>,
    // pub draw_commands: Vec<CmdBuffer>,
    // pub present_complete_semaphore: Option<Semaphore>,
    // pub render_complete_semaphore: Option<Semaphore>,
    // pub wait_fences: Vec<Fence>,
    // pub buffer_manager: Option<Arc<DebugCell<BufferManager>>>,
    pub pipeline_manager: Arc<RwLock<PipelineManager>>,
    // pub basic_engine: Option<BasicEngine>,
}

impl Engine {
    pub fn new(os_app: &Arc<RwLock<OsApp>>) -> Self {
        let instance = Arc::new(Instance::new());
        let surface = Arc::new(Surface::new(&instance, os_app));
        let physical_device = Arc::new(PhysicalDevice::new(&surface));
        let logical_device = Arc::new(LogicalDevice::new(&physical_device));
        let swapchain = Arc::new(Swapchain::new(&logical_device));
        let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(&logical_device)));
        let graphic_cmd_pool = Arc::new(CmdPool::new(&logical_device, CmdPoolType::Graphic, 0));
        let compute_cmd_pool = Arc::new(CmdPool::new(&logical_device, CmdPoolType::Compute, 0));

        Engine {
            //     core_app: unsafe { transmute(0usize) },
            //     os_app: unsafe { transmute(0usize) },
            instance,
            surface,
            physical_device,
            logical_device,
            swapchain,
            //     depth_stencil_image_view: None,
            //     render_pass: None,
            //     framebuffers: Vec::new(),
            graphic_cmd_pool,
            compute_cmd_pool,
            //     transfer_cmd_pool: None,
            //     draw_commands: Vec::new(),
            //     present_complete_semaphore: None,
            //     render_complete_semaphore: None,
            //     wait_fences: Vec::new(),
            //     buffer_manager: None,
            pipeline_manager,
            //     basic_engine: None,
        }
    }

    // fn set_core_app(&mut self, c: &'static mut CoreApp) {
    //     // self.core_app = c;
    // }

    // fn set_os_app(&mut self, o: &'static mut OsApplication<CoreApp>) {
    //     self.os_app = o;
    // }

    // fn initialize(&mut self) {
    //     let surface = Arc::new(Surface::new(self.instance.clone(), self.os_app));
    //     self.surface = Some(surface.clone());
    //     let physical_device = Arc::new(PhysicalDevice::new(surface.clone()));
    //     self.physical_device = Some(physical_device.clone());
    //     let logical_device = Arc::new(LogicalDevice::new(physical_device.clone()));
    //     self.logical_device = Some(logical_device.clone()); // Beacause of shader stage
    //     let swapchain = Arc::new(Swapchain::new(logical_device.clone()));
    //     self.swapchain = Some(swapchain.clone());
    //     let depth_stencil = Arc::new(ImageView::new_depth_stencil(logical_device.clone()));
    //     self.depth_stencil_image_view = Some(depth_stencil.clone());
    //     let render_pass = Arc::new(RenderPass::new(swapchain.clone()));
    //     self.render_pass = Some(render_pass.clone());
    //     for v in swapchain.image_views.clone() {
    //         self.framebuffers.push(Arc::new(Framebuffer::new(
    //             v.clone(),
    //             depth_stencil.clone(),
    //             render_pass.clone(),
    //         )));
    //     }
    //     let graphic_cmd_pool = Arc::new(CmdPool::new(logical_device.clone()));
    //     self.graphic_cmd_pool = Some(graphic_cmd_pool.clone());
    //     self.transfer_cmd_pool = Some(graphic_cmd_pool);
    //     self.present_complete_semaphore = Some(Semaphore::new(logical_device.clone()));
    //     self.render_complete_semaphore = Some(Semaphore::new(logical_device.clone()));
    //     for _ in 0..self.framebuffers.len() {
    //         self.wait_fences.push(Fence::new_signaled(
    //             self.logical_device.as_ref().unwrap().clone(),
    //         ));
    //     }
    //     // TODO
    //     self.buffer_manager = Some(Arc::new(DebugCell::new(BufferManager::new(
    //         self.logical_device.as_ref().unwrap().clone(),
    //         1024 * 1024,
    //         1024 * 1024,
    //         self.framebuffers.len()
    //     ))));
    //     let pipeline_manager = Arc::new(DebugCell::new(PipelineManager::new(self)));
    //     self.pipeline_manager = Some(pipeline_manager);
    //     self.basic_engine = Some(BasicEngine::new(self.os_app));
    //     self.record();
    // }

    // fn on_event(&mut self, e: Event) {
    //     match e {
    //         Event::WindowSize { w, h } => {
    //             self.window_resized(w, h);
    //         }
    //         _ => {}
    //     }
    // }

    // fn update(&mut self) {
    //     // let vk_device = self.logical_device.as_ref().unwrap().vk_data;
    //     // let present_complete_semaphore = self.present_complete_semaphore.as_ref().unwrap();
    //     // let current_buffer = match self.swapchain
    //     //     .as_ref()
    //     //     .unwrap()
    //     //     .get_next_image_index(present_complete_semaphore)
    //     // {
    //     //     NextImageResult::Next(c) => c,
    //     //     NextImageResult::NeedsRefresh => {
    //     //         unsafe {
    //     //             (*(*self.os_app).render_engine).reinitialize();
    //     //         }
    //     //         return;
    //     //     }
    //     // } as usize;
    //     // let uniform_data = {
    //     //     let current_scene = self.basic_engine.as_ref().unwrap().current_scene.borrow();
    //     //     let current_camera = current_scene.get_current_camera().borrow();
    //     //     UniformData {
    //     //         projection: current_camera.get_view_projection().clone(),
    //     //         view: Mat4x4::ident(),
    //     //         model: Mat4x4::ident(),
    //     //     }
    //     // };
    //     // self.uniform
    //     //     .as_ref()
    //     //     .unwrap()
    //     //     .update(unsafe { transmute(&uniform_data) });
    //     // vulkan_check!(vk::vkWaitForFences(
    //     //     vk_device,
    //     //     1,
    //     //     &(self.wait_fences[current_buffer].vk_data),
    //     //     1u32,
    //     //     u64::max_value(),
    //     // ));
    //     // vulkan_check!(vk::vkResetFences(
    //     //     vk_device,
    //     //     1,
    //     //     &(self.wait_fences[current_buffer].vk_data),
    //     // ));
    //     // let wait_stage_mask =
    //     //     vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
    //     // let mut submit_info = vk::VkSubmitInfo::default();
    //     // submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
    //     // submit_info.pWaitDstStageMask = &wait_stage_mask;
    //     // submit_info.pWaitSemaphores = &(self.present_complete_semaphore.as_ref().unwrap().vk_data);
    //     // submit_info.waitSemaphoreCount = 1;
    //     // submit_info.pSignalSemaphores = &(self.render_complete_semaphore.as_ref().unwrap().vk_data);
    //     // submit_info.signalSemaphoreCount = 1;
    //     // submit_info.pCommandBuffers = &(self.draw_commands[current_buffer].vk_data);
    //     // submit_info.commandBufferCount = 1;
    //     // vulkan_check!(vk::vkQueueSubmit(
    //     //     self.logical_device.as_ref().unwrap().vk_graphic_queue,
    //     //     1,
    //     //     &submit_info,
    //     //     self.wait_fences[current_buffer].vk_data,
    //     // ));
    //     // let image_index = current_buffer as u32;
    //     // let mut present_info = vk::VkPresentInfoKHR::default();
    //     // present_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
    //     // present_info.swapchainCount = 1;
    //     // present_info.pSwapchains = &(self.swapchain.as_ref().unwrap().vk_data);
    //     // present_info.pImageIndices = &image_index;
    //     // present_info.pWaitSemaphores = &(self.render_complete_semaphore.as_ref().unwrap().vk_data);
    //     // present_info.waitSemaphoreCount = 1;
    //     // vulkan_check!(vk::vkQueuePresentKHR(
    //     //     self.logical_device.as_ref().unwrap().vk_graphic_queue,
    //     //     &present_info,
    //     // ));
    // }

    // fn terminate(&mut self) {
    //     self.clean();
    // }

    // fn get_basic(&self) -> &BasicEngine {
    //     self.basic_engine.as_ref().unwrap()
    // }

    // fn get_mut_basic(&mut self) -> &mut BasicEngine {
    //     self.basic_engine.as_mut().unwrap()
    // }

    // fn record(&mut self) {
    //     let mut clear_values = [vk::VkClearValue::default(); 2];
    //     clear_values[0].data = [0.4, 0.4, 0.4, 1.0];
    //     clear_values[1].data = [1.0, 0.0, 0.0, 0.0];
    //     let surface_caps = self.physical_device
    //         .as_ref()
    //         .unwrap()
    //         .get_surface_capabilities();
    //     let mut render_pass_begin_info = vk::VkRenderPassBeginInfo::default();
    //     render_pass_begin_info.sType =
    //         vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    //     render_pass_begin_info.renderPass = self.render_pass.as_ref().unwrap().vk_data;
    //     render_pass_begin_info.renderArea.offset.x = 0;
    //     render_pass_begin_info.renderArea.offset.y = 0;
    //     render_pass_begin_info.renderArea.extent.width = surface_caps.currentExtent.width;
    //     render_pass_begin_info.renderArea.extent.height = surface_caps.currentExtent.height;
    //     render_pass_begin_info.clearValueCount = clear_values.len() as u32;
    //     render_pass_begin_info.pClearValues = clear_values.as_ptr();
    //     let images_count = self.framebuffers.len();
    //     for i in 0..images_count {
    //         render_pass_begin_info.framebuffer = self.framebuffers[i].vk_data;
    //         let mut draw_command = CmdBuffer::new(self.graphic_cmd_pool.as_ref().unwrap().clone());
    //         draw_command.begin_render_pass_with_info(render_pass_begin_info);
    //         let mut viewport = vk::VkViewport::default();
    //         viewport.x = 0.0;
    //         viewport.y = 0.0;
    //         viewport.height = surface_caps.currentExtent.height as f32;
    //         viewport.width = surface_caps.currentExtent.width as f32;
    //         viewport.minDepth = 0.0;
    //         viewport.maxDepth = 1.0;
    //         draw_command.set_viewport(viewport);
    //         let mut scissor = vk::VkRect2D::default();
    //         scissor.extent.width = surface_caps.currentExtent.width;
    //         scissor.extent.height = surface_caps.currentExtent.height;
    //         scissor.offset.x = 0;
    //         scissor.offset.y = 0;
    //         draw_command.set_scissor(scissor);
    //         self.basic_engine.as_mut().unwrap().record(&mut draw_command, i);
    //         // unsafe {
    //             // vk::vkCmdBindDescriptorSets(
    //             //     draw_command.vk_data,
    //             //     vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
    //             //     self.pipeline_layout.as_ref().unwrap().vk_data,
    //             //     0,
    //             //     1,
    //             //     &(self.descriptor_set.as_ref().unwrap().vk_data),
    //             //     0,
    //             //     null(),
    //             // );
    //             // vk::vkCmdBindPipeline(
    //             //     draw_command.vk_data,
    //             //     vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
    //             //     self.pipeline.as_ref().unwrap().vk_data,
    //             // );
    //             // let offsets = 0 as vk::VkDeviceSize;
    //             // vk::vkCmdBindVertexBuffers(
    //             //     draw_command.vk_data,
    //             //     0,
    //             //     1,
    //             //     &(self.mesh_buff.as_ref().unwrap().vertices_buffer),
    //             //     &offsets,
    //             // );
    //             // vk::vkCmdBindIndexBuffer(
    //             //     draw_command.vk_data,
    //             //     self.mesh_buff.as_ref().unwrap().indices_buffer,
    //             //     0,
    //             //     vk::VkIndexType::VK_INDEX_TYPE_UINT32,
    //             // );
    //             // vk::vkCmdDrawIndexed(
    //             //     draw_command.vk_data,
    //             //     self.mesh_buff.as_ref().unwrap().indices_count,
    //             //     1,
    //             //     0,
    //             //     0,
    //             //     1,
    //             // );
    //         // }
    //         draw_command.end_render_pass();
    //         draw_command.end();
    //         self.draw_commands.push(draw_command);
    //     }
    // }

    // fn clean(&mut self) {
    //     self.logical_device.as_ref().unwrap().wait_idle();
    //     self.basic_engine = None;
    //     // TODO
    //     self.pipeline_manager = None;
    //     self.buffer_manager = None;
    //     self.wait_fences.clear();
    //     self.render_complete_semaphore = None;
    //     self.present_complete_semaphore = None;
    //     self.draw_commands.clear();
    //     self.transfer_cmd_pool = None;
    //     self.graphic_cmd_pool = None;
    //     self.framebuffers.clear();
    //     self.render_pass = None;
    //     self.depth_stencil_image_view = None;
    //     self.swapchain = None;
    //     self.logical_device = None;
    //     self.physical_device = None;
    //     self.surface = None;
    // }

    // fn reinitialize(&mut self) {
    //     self.clean();
    //     self.initialize();
    // }

    // fn window_resized(&mut self, w: f64, h: f64) {
    //     {
    //         let current_scene = self.basic_engine.as_mut().unwrap().current_scene.borrow();
    //         let mut current_camera = current_scene.get_current_camera().borrow_mut();
    //         current_camera.set_viewport(w as f32, h as f32);
    //     }
    //     self.reinitialize();
    // }
}
