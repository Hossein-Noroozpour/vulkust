use std::ptr::{
    null_mut,
    null,
};
use std::sync::Arc;
use super::super::system::vulkan as vk;
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
use super::pipeline::layout::Layout;
use super::pipeline::pipeline::Pipeline;
use super::pipeline::cache::Cache as PipelineCache;
use super::descriptor::pool::Pool as DescriptorPool;
use super::descriptor::set::Set as DescriptorSet;
use super::command::buffer::Buffer as CmdBuffer;
use super::synchronizer::semaphore::Semaphore;
use super::fence::Fence;
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
    pub pipeline_layout: Option<Arc<Layout>>,
    pub pipeline_cache: Option<Arc<PipelineCache>>,
    pub pipeline: Option<Arc<Pipeline>>,
    pub descriptor_pool: Option<Arc<DescriptorPool>>,
    pub descriptor_set: Option<Arc<DescriptorSet>>,
    pub draw_commands: Vec<CmdBuffer>,
    pub present_complete_semaphore: Option<Semaphore>,
    pub render_complete_semaphore: Option<Semaphore>,
    pub wait_fences: Vec<Fence>,
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
            pipeline_layout: None,
            pipeline_cache: None,
            pipeline: None,
            descriptor_pool: None,
            descriptor_set: None,
            draw_commands: Vec::new(),
            present_complete_semaphore: None,
            render_complete_semaphore: None,
            wait_fences: Vec::new(),
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
            logical_device.clone(), graphic_cmd_pool.clone(), uniform_data.len() as u32 * 4));
        uniform.update(unsafe { transmute(uniform_data.as_ptr()) });
        let pipeline_layout = Arc::new(Layout::new(logical_device.clone()));
        let pipeline_cache = Arc::new(PipelineCache::new(logical_device.clone()));
        let pipeline = Arc::new(Pipeline::new(
            pipeline_layout.clone(), render_pass.clone(), pipeline_cache.clone()));
        let descriptor_pool = Arc::new(DescriptorPool::new(logical_device.clone()));
        let descriptor_set = Arc::new(DescriptorSet::new(
            descriptor_pool.clone(), pipeline_layout.clone(), uniform.clone()));
        let present_complete_semaphore = Semaphore::new(logical_device.clone());
        let render_complete_semaphore = Semaphore::new(logical_device.clone());
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
        self.pipeline_layout = Some(pipeline_layout);
        self.pipeline_cache = Some(pipeline_cache);
        self.pipeline = Some(pipeline);
        self.descriptor_pool = Some(descriptor_pool);
        self.descriptor_set = Some(descriptor_set);
        self.initialize_draw_commands();
        self.present_complete_semaphore = Some(present_complete_semaphore);
        self.render_complete_semaphore = Some(render_complete_semaphore);
        for _ in 0..self.framebuffers.len() {
            self.wait_fences.push(Fence::new_signaled(
                self.logical_device.as_ref().unwrap().clone()));
        }
    }

    fn update(&mut self) {
        let vk_device = self.logical_device.as_ref().unwrap().vk_data;
	    let current_buffer = self.swapchain.as_ref().unwrap().get_next_image_index(
            self.present_complete_semaphore.as_ref().unwrap()) as usize;
		vulkan_check!(vk::vkWaitForFences(
            vk_device, 1, &(self.wait_fences[current_buffer].vk_data), 1u32, u64::max_value()));
		vulkan_check!(vk::vkResetFences(
            vk_device, 1, &(self.wait_fences[current_buffer].vk_data)));
		let wait_stage_mask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
		let mut submit_info = vk::VkSubmitInfo::default();
		submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
		submit_info.pWaitDstStageMask = &wait_stage_mask;
		submit_info.pWaitSemaphores = &(self.present_complete_semaphore.as_ref().unwrap().vk_data);
		submit_info.waitSemaphoreCount = 1;
		submit_info.pSignalSemaphores =
            &(self.render_complete_semaphore.as_ref().unwrap().vk_data);
		submit_info.signalSemaphoreCount = 1;
		submit_info.pCommandBuffers = &(self.draw_commands[current_buffer].vk_data);
		submit_info.commandBufferCount = 1;
		vulkan_check!(vk::vkQueueSubmit(
            self.logical_device.as_ref().unwrap().vk_graphic_queue, 1, &submit_info,
            self.wait_fences[current_buffer].vk_data));
        let image_index = current_buffer as u32;
        let mut present_info = vk::VkPresentInfoKHR::default();
    	present_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
    	present_info.swapchainCount = 1;
    	present_info.pSwapchains = &(self.swapchain.as_ref().unwrap().vk_data);
    	present_info.pImageIndices = &image_index;
    	present_info.pWaitSemaphores = &(self.render_complete_semaphore.as_ref().unwrap().vk_data);
    	present_info.waitSemaphoreCount = 1;
    	vulkan_check!(vk::vkQueuePresentKHR(
            self.logical_device.as_ref().unwrap().vk_graphic_queue, &present_info));
    }

    fn terminate(&mut self) {
        self.logical_device.as_ref().unwrap().wait_idle();

        self.wait_fences.clear();
        self.render_complete_semaphore = None;
        self.present_complete_semaphore = None;
        self.draw_commands.clear();
        self.descriptor_set = None;
        self.descriptor_pool = None;
        self.pipeline = None;
        self.pipeline_cache = None;
        self.pipeline_layout = None;
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

impl<CoreApp> Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn initialize_draw_commands(&mut self) {
        let mut clear_values = [vk::VkClearValue::default(); 2];
		clear_values[0].data = [0.4, 0.4, 0.4, 1.0];
		clear_values[1].data = [1.0, 0.0, 0.0, 0.0];
        let surface_caps = self.physical_device.as_ref().unwrap().get_surface_capabilities();
		let mut render_pass_begin_info = vk::VkRenderPassBeginInfo::default();
		render_pass_begin_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
		render_pass_begin_info.renderPass = self.render_pass.as_ref().unwrap().vk_data;
		render_pass_begin_info.renderArea.offset.x = 0;
		render_pass_begin_info.renderArea.offset.y = 0;
		render_pass_begin_info.renderArea.extent.width = surface_caps.currentExtent.width;
		render_pass_begin_info.renderArea.extent.height = surface_caps.currentExtent.height;
		render_pass_begin_info.clearValueCount = 2;
		render_pass_begin_info.pClearValues = clear_values.as_ptr();
        let mut draw_commands = Vec::new();
        let images_count = self.framebuffers.len();
		for i in 0..images_count {
			render_pass_begin_info.framebuffer = self.framebuffers[i].vk_data;
            let draw_command = CmdBuffer::new(self.graphic_cmd_pool.as_ref().unwrap().clone());
			draw_command.begin_render_pass_with_info(render_pass_begin_info);
			let mut viewport = vk::VkViewport::default();
            viewport.x = 0.0;
            viewport.y = 0.0;
			viewport.height = surface_caps.currentExtent.height as f32;
			viewport.width = surface_caps.currentExtent.width as f32;
			viewport.minDepth = 0.0;
			viewport.maxDepth = 1.0;
			draw_command.set_viewport(viewport);
            let mut scissor = vk::VkRect2D::default();
			scissor.extent.width = surface_caps.currentExtent.width;
			scissor.extent.height = surface_caps.currentExtent.height;
			scissor.offset.x = 0;
			scissor.offset.y = 0;
			draw_command.set_scissor(scissor);
			unsafe {
                vk::vkCmdBindDescriptorSets(
                    draw_command.vk_data, vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                    self.pipeline_layout.as_ref().unwrap().vk_data, 0, 1,
                    &(self.descriptor_set.as_ref().unwrap().vk_data), 0, null());
                vk::vkCmdBindPipeline(
                    draw_command.vk_data,
                    vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                    self.pipeline.as_ref().unwrap().vk_data);
			    let offsets = 0 as vk::VkDeviceSize;
			    vk::vkCmdBindVertexBuffers(
                    draw_command.vk_data, 0, 1, &(self.mesh_buff.as_ref().unwrap().vertices_buffer),
                    &offsets);
			    vk::vkCmdBindIndexBuffer(
                    draw_command.vk_data, self.mesh_buff.as_ref().unwrap().indices_buffer, 0,
                    vk::VkIndexType::VK_INDEX_TYPE_UINT32);
			    vk::vkCmdDrawIndexed(
                    draw_command.vk_data, self.mesh_buff.as_ref().unwrap().indices_count, 1, 0, 0, 1);

			    vk::vkCmdEndRenderPass(draw_command.vk_data);
            }
		    vulkan_check!(vk::vkEndCommandBuffer(draw_command.vk_data));
            draw_commands.push(draw_command);
		}
        self.draw_commands = draw_commands;
    }
}
