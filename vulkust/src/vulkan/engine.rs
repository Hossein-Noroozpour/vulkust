use std::mem::transmute;
use std::sync::{Arc, RwLock};
// use super::super::core::application::ApplicationTrait;
// use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
// use super::super::system::vulkan as vk;
// use super::super::util::cell::DebugCell;
use super::buffer::{DynamicBuffer, Manager as BufferManager, StaticBuffer};
use super::command::buffer::Buffer as CmdBuffer;
use super::command::pool::{Pool as CmdPool, Type as CmdPoolType};
use super::device::logical::Logical as LogicalDevice;
use super::device::physical::Physical as PhysicalDevice;
// use super::fence::Fence;
use super::framebuffer::Framebuffer;
use super::image::view::View as ImageView;
use super::instance::Instance;
use super::memory::Manager as MemoryManager;
use super::pipeline::Manager as PipelineManager;
use super::render_pass::RenderPass;
use super::surface::Surface;
use super::swapchain::Swapchain;
use super::synchronizer::semaphore::Semaphore;

const UNIFORM: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

pub struct Engine {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub physical_device: Arc<PhysicalDevice>,
    pub logical_device: Arc<LogicalDevice>,
    pub swapchain: Arc<Swapchain>,
    pub present_complete_semaphore: Arc<Semaphore>,
    pub render_complete_semaphore: Arc<Semaphore>,
    pub graphic_cmd_pool: Arc<CmdPool>,
    pub draw_commands: Vec<CmdBuffer>,
    pub memory_mgr: Arc<RwLock<MemoryManager>>,
    pub depth_stencil_image_view: Arc<ImageView>,
    pub render_pass: Arc<RenderPass>,
    pub pipeline_manager: Arc<RwLock<PipelineManager>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub frame_number: Arc<RwLock<u32>>,
    pub buffer_manager: Arc<RwLock<BufferManager>>,
    // pub transfer_cmd_pool: Option<Arc<CmdPool>>,
    // pub wait_fences: Vec<Fence>,
    // pub basic_engine: Option<BasicEngine>,
    //----------------------------------------------------------------------------------------------
    pub vertex_buffer: StaticBuffer,
    pub index_buffer: StaticBuffer,
    pub uniform_buffer: DynamicBuffer,
    //----------------------------------------------------------------------------------------------
}

impl Engine {
    pub fn new(os_app: &Arc<RwLock<OsApp>>) -> Self {
        let instance = Arc::new(Instance::new());
        let surface = Arc::new(Surface::new(&instance, os_app));
        let physical_device = Arc::new(PhysicalDevice::new(&surface));
        let logical_device = Arc::new(LogicalDevice::new(&physical_device));
        let swapchain = Arc::new(Swapchain::new(&logical_device));
        let present_complete_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let render_complete_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let graphic_cmd_pool = Arc::new(CmdPool::new(&logical_device, CmdPoolType::Graphic, 0));
        let mut draw_commands = Vec::new();
        for _ in 0..swapchain.image_views.len() {
            let draw_command = CmdBuffer::new(graphic_cmd_pool.clone());
            draw_commands.push(draw_command);
        }
        draw_commands.shrink_to_fit();
        let memory_mgr = Arc::new(RwLock::new(MemoryManager::new(&logical_device)));
        let memory_mgr_w = Arc::downgrade(&memory_mgr);
        vxresult!(memory_mgr.write()).set_itself(memory_mgr_w);
        let depth_stencil_image_view = Arc::new(ImageView::new_depth_stencil(
            logical_device.clone(),
            &memory_mgr,
        ));
        let render_pass = Arc::new(RenderPass::new(&swapchain));
        let mut framebuffers = Vec::new();
        for v in &swapchain.image_views {
            framebuffers.push(Arc::new(Framebuffer::new(
                v.clone(),
                depth_stencil_image_view.clone(),
                render_pass.clone(),
            )));
        }
        framebuffers.shrink_to_fit();
        let frame_number = Arc::new(RwLock::new(0));
        let mut buffer_manager = BufferManager::new(
            &memory_mgr,
            &graphic_cmd_pool,
            &frame_number,
            1028,
            1028,
            1028,
            swapchain.image_views.len() as isize,
        );
        // -----------------------------------------------------------------------------------------
        let vertices = [
            1.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32,
            1.0f32, 0.0f32, 0.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
        ];
        let vertices_size = vertices.len() * 4;
        let vertex_buffer = buffer_manager.create_static_buffer(vertices_size as isize, unsafe {
            transmute(vertices.as_ptr())
        });
        let indices = [0u32, 1, 2];
        let indices_size = indices.len() * 4;
        let index_buffer = buffer_manager.create_static_buffer(indices_size as isize, unsafe {
            transmute(indices.as_ptr())
        });
        let uniform_size = (UNIFORM.len() * 4) as isize;
        let uniform_buffer = buffer_manager.create_dynamic_buffer(uniform_size);
        //------------------------------------------------------------------------------------------
        let buffer_manager = Arc::new(RwLock::new(buffer_manager));
        let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(
            &logical_device,
            &buffer_manager,
            &render_pass,
        )));
        Engine {
            //     core_app: unsafe { transmute(0usize) },
            //     os_app: unsafe { transmute(0usize) },
            instance,
            surface,
            physical_device,
            logical_device,
            swapchain,
            present_complete_semaphore,
            render_complete_semaphore,
            graphic_cmd_pool,
            draw_commands,
            memory_mgr,
            depth_stencil_image_view,
            render_pass,
            pipeline_manager,
            framebuffers,
            frame_number,
            buffer_manager,
            //     transfer_cmd_pool: None,
            //     wait_fences: Vec::new(),
            //     basic_engine: None,
            //--------------------------------------------------------------------------------------
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            //--------------------------------------------------------------------------------------
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

    pub fn update(&mut self) {
        {
            let mut frame_number = vxresult!(self.frame_number.write());
            *frame_number += 1;
            *frame_number %= self.framebuffers.len() as u32;
        }
        self.uniform_buffer
            .update(unsafe { transmute(UNIFORM.as_ptr()) });
        vxresult!(self.buffer_manager.write()).update();
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
    }

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
