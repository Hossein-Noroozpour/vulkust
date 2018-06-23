use std::mem::transmute;
use std::sync::{Arc, RwLock};
// use super::super::core::application::ApplicationTrait;
// use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
use super::buffer::{DynamicBuffer, Manager as BufferManager, StaticBuffer};
use super::command::buffer::Buffer as CmdBuffer;
use super::command::pool::{Pool as CmdPool, Type as CmdPoolType};
use super::descriptor::{Manager as DescriptorManager, Set as DescriptorSet};
use super::device::logical::Logical as LogicalDevice;
use super::device::physical::Physical as PhysicalDevice;
use super::framebuffer::Framebuffer;
use super::image::view::View as ImageView;
use super::instance::Instance;
use super::memory::Manager as MemoryManager;
use super::pipeline::Manager as PipelineManager;
use super::render_pass::RenderPass;
use super::sampler::Sampler;
use super::surface::Surface;
use super::swapchain::{NextImageResult, Swapchain};
use super::synchronizer::fence::Fence;
use super::synchronizer::semaphore::Semaphore;
use super::vulkan as vk;

use math;
use math::prelude::*;

const INDICES: [u32; 3] = [0, 1, 2];

const UNIFORM: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

pub struct Engine {
    pub os_app: Arc<RwLock<OsApp>>,
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub physical_device: Arc<PhysicalDevice>,
    pub logical_device: Arc<LogicalDevice>,
    pub swapchain: Arc<Swapchain>,
    pub present_complete_semaphore: Arc<Semaphore>,
    pub render_complete_semaphore: Arc<Semaphore>,
    pub graphic_cmd_pool: Arc<CmdPool>,
    pub draw_commands: Vec<Arc<RwLock<CmdBuffer>>>,
    pub memory_mgr: Arc<RwLock<MemoryManager>>,
    pub depth_stencil_image_view: Arc<ImageView>,
    pub render_pass: Arc<RenderPass>,
    pub descriptor_manager: Arc<RwLock<DescriptorManager>>,
    pub pipeline_manager: Arc<RwLock<PipelineManager>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub frame_number: Arc<RwLock<u32>>,
    pub buffer_manager: Arc<RwLock<BufferManager>>,
    pub wait_fences: Vec<Arc<Fence>>,
    pub sampler: Arc<Sampler>,
    //----------------------------------------------------------------------------------------------
    pub vertex_buffer: StaticBuffer,
    pub index_buffer: StaticBuffer,
    pub uniform_buffer: DynamicBuffer,
    pub texture_view: Arc<ImageView>,
    pub main_desc: Arc<DescriptorSet>,
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
        let mut wait_fences = Vec::new();
        for _ in 0..swapchain.image_views.len() {
            let draw_command = Arc::new(RwLock::new(CmdBuffer::new(graphic_cmd_pool.clone())));
            draw_commands.push(draw_command);
            wait_fences.push(Arc::new(Fence::new_signaled(logical_device.clone())));
        }
        draw_commands.shrink_to_fit();
        wait_fences.shrink_to_fit();
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
            4 * 1028,
            8 * 1028 * 1028,
            4 * 1028,
            swapchain.image_views.len() as isize,
        );
        let sampler = Arc::new(Sampler::new(logical_device.clone()));
        let vertices = vec![
            1.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32, -1.0f32, 1.0f32,
            0.0f32, 0.0f32, 1.0f32, 1.0f32, -1.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, 0.0f32,
            0.0f32, 1.0f32, 0.0f32, -1.0f32,
        ];
        let vertex_buffer = buffer_manager.create_static_buffer_with_vec(&vertices);
        let index_buffer = buffer_manager.create_static_buffer_with_vec(&INDICES.to_vec());
        let uniform_size = (UNIFORM.len() * 4) as isize;
        let uniform_buffer = buffer_manager.create_dynamic_buffer(uniform_size);
        let buffer_manager = Arc::new(RwLock::new(buffer_manager));
        let descriptor_manager = Arc::new(RwLock::new(DescriptorManager::new(
            &buffer_manager,
            &logical_device,
        )));
        let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(
            &logical_device,
            &descriptor_manager,
            &render_pass,
        )));
        let texture_view = Arc::new(ImageView::new_texture_with_file("1.png", &buffer_manager));
        let main_desc = Arc::new(
            vxresult!(descriptor_manager.write()).create_main_set(&texture_view, &sampler),
        );
        let os_app = os_app.clone();
        Engine {
            // core_app: unsafe { transmute(0usize) },
            os_app,
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
            descriptor_manager,
            pipeline_manager,
            framebuffers,
            frame_number,
            buffer_manager,
            wait_fences,
            sampler,
            //--------------------------------------------------------------------------------------
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            texture_view,
            main_desc,
            //--------------------------------------------------------------------------------------
        }
    }

    // fn on_event(&mut self, e: Event) {
    //     match e {
    //         Event::WindowSize { w, h } => {
    //             self.window_resized(w, h);
    //         }
    //         _ => {}
    //     }
    // }

    pub fn update(&mut self) {
        let current_buffer = match self
            .swapchain
            .get_next_image_index(&self.present_complete_semaphore)
        {
            NextImageResult::Next(c) => c,
            NextImageResult::NeedsRefresh => {
                self.reinitialize();
                return;
            }
        } as usize;
        vulkan_check!(vk::vkWaitForFences(
            self.logical_device.vk_data,
            1,
            &self.wait_fences[current_buffer].vk_data,
            1,
            u64::max_value(),
        ));
        vulkan_check!(vk::vkResetFences(
            self.logical_device.vk_data,
            1,
            &self.wait_fences[current_buffer].vk_data,
        ));
        *vxresult!(self.frame_number.write()) = current_buffer as u32;
        self.record();
        let proj = math::perspective(math::Rad(1.57f32), 1.43f32, 0.1f32, 2.0f32);
        let view = math::Matrix4::look_at(
            math::Point3::new(0.0f32, 0.0f32, 1.5f32),
            math::Point3::new(0.0f32, 0.0f32, 0.0f32),
            math::Vector3::new(0.0f32, 1.0f32, 0.0f32));
        let vp = proj * view;
        self.uniform_buffer
            .update(unsafe { transmute(vp.as_ptr()) });
        vxresult!(self.buffer_manager.write()).update();
        let wait_stage_mask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        let mut submit_info = vk::VkSubmitInfo::default();
        submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit_info.pWaitDstStageMask = &wait_stage_mask;
        submit_info.pWaitSemaphores = &self.present_complete_semaphore.vk_data;
        submit_info.waitSemaphoreCount = 1;
        submit_info.pSignalSemaphores = &self.render_complete_semaphore.vk_data;
        submit_info.signalSemaphoreCount = 1;
        submit_info.pCommandBuffers = &vxresult!(self.draw_commands[current_buffer].read()).vk_data;
        submit_info.commandBufferCount = 1;
        vulkan_check!(vk::vkQueueSubmit(
            self.logical_device.vk_graphic_queue,
            1,
            &submit_info,
            self.wait_fences[current_buffer].vk_data,
        ));
        let image_index = current_buffer as u32;
        let mut present_info = vk::VkPresentInfoKHR::default();
        present_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
        present_info.swapchainCount = 1;
        present_info.pSwapchains = &self.swapchain.vk_data;
        present_info.pImageIndices = &image_index;
        present_info.pWaitSemaphores = &self.render_complete_semaphore.vk_data;
        present_info.waitSemaphoreCount = 1;
        vulkan_check!(vk::vkQueuePresentKHR(
            self.logical_device.vk_graphic_queue,
            &present_info,
        ));
    }

    pub fn terminate(&mut self) {
        self.logical_device.wait_idle();
    }

    // fn get_basic(&self) -> &BasicEngine {
    //     self.basic_engine.as_ref().unwrap()
    // }

    // fn get_mut_basic(&mut self) -> &mut BasicEngine {
    //     self.basic_engine.as_mut().unwrap()
    // }

    pub fn record(&mut self) {
        let mut clear_values = [vk::VkClearValue::default(); 2];
        clear_values[0].data = [0.4, 0.4, 0.4, 1.0];
        clear_values[1].data = [1.0, 0.0, 0.0, 0.0];
        let surface_caps = &self.physical_device.surface_caps;
        let mut render_pass_begin_info = vk::VkRenderPassBeginInfo::default();
        render_pass_begin_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
        render_pass_begin_info.renderPass = self.render_pass.vk_data;
        render_pass_begin_info.renderArea.offset.x = 0;
        render_pass_begin_info.renderArea.offset.y = 0;
        render_pass_begin_info.renderArea.extent.width = surface_caps.currentExtent.width;
        render_pass_begin_info.renderArea.extent.height = surface_caps.currentExtent.height;
        render_pass_begin_info.clearValueCount = clear_values.len() as u32;
        render_pass_begin_info.pClearValues = clear_values.as_ptr();
        let frame_number = vxresult!(self.frame_number.read());
        let frame_number = *frame_number as usize;
        render_pass_begin_info.framebuffer = self.framebuffers[frame_number].vk_data;
        let mut viewport = vk::VkViewport::default();
        viewport.x = 0.0;
        viewport.y = 0.0;
        viewport.height = surface_caps.currentExtent.height as f32;
        viewport.width = surface_caps.currentExtent.width as f32;
        viewport.minDepth = 0.0;
        viewport.maxDepth = 1.0;
        let mut scissor = vk::VkRect2D::default();
        scissor.extent.width = surface_caps.currentExtent.width;
        scissor.extent.height = surface_caps.currentExtent.height;
        scissor.offset.x = 0;
        scissor.offset.y = 0;
        let draw_command = &mut self.draw_commands[frame_number];
        let mut draw_command = vxresult!(draw_command.write());
        draw_command.reset();
        draw_command.begin();
        draw_command.begin_render_pass_with_info(render_pass_begin_info);
        draw_command.set_viewport(viewport);
        draw_command.set_scissor(scissor);
        let pipemgr = vxresult!(self.pipeline_manager.read());
        draw_command.bind_descriptor_set(
            &pipemgr.main_pipeline.layout,
            &self.main_desc,
            vxresult!(self.uniform_buffer.buffers[frame_number].0.read())
                .info
                .offset as usize,
        );
        draw_command.bind_pipeline(&pipemgr.main_pipeline);
        draw_command.bind_vertex_buffer(&self.vertex_buffer.buffer);
        draw_command.bind_index_buffer(&self.index_buffer.buffer);
        draw_command.draw_index(INDICES.len() as u32);
        draw_command.end_render_pass();
        draw_command.end();
    }

    fn reinitialize(&mut self) {
        self.logical_device.wait_idle();
        let new = Self::new(&self.os_app);
        self.instance = new.instance.clone();
        self.surface = new.surface.clone();
        self.physical_device = new.physical_device.clone();
        self.logical_device = new.logical_device.clone();
        self.swapchain = new.swapchain.clone();
        self.present_complete_semaphore = new.present_complete_semaphore.clone();
        self.render_complete_semaphore = new.render_complete_semaphore.clone();
        self.graphic_cmd_pool = new.graphic_cmd_pool.clone();
        self.draw_commands = new.draw_commands.clone();
        self.memory_mgr = new.memory_mgr.clone();
        self.depth_stencil_image_view = new.depth_stencil_image_view.clone();
        self.render_pass = new.render_pass.clone();
        self.pipeline_manager = new.pipeline_manager.clone();
        self.framebuffers = new.framebuffers.clone();
        self.frame_number = new.frame_number.clone();
        self.buffer_manager = new.buffer_manager.clone();
        self.wait_fences = new.wait_fences.clone();
        self.vertex_buffer = new.vertex_buffer.clone();
        self.index_buffer = new.index_buffer.clone();
        self.uniform_buffer = new.uniform_buffer.clone();
    }

    // fn window_resized(&mut self, w: f64, h: f64) {
    //     {
    //         let current_scene = self.basic_engine.as_mut().unwrap().current_scene.borrow();
    //         let mut current_camera = current_scene.get_current_camera().borrow_mut();
    //         current_camera.set_viewport(w as f32, h as f32);
    //     }
    //     self.reinitialize();
    // }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.logical_device.wait_idle();
    }
}
