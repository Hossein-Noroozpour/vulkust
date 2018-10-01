use super::super::render::config::Configurations;
use super::super::render::image::AttachmentType;
use super::super::system::os::application::Application as OsApp;
use super::buffer::Manager as BufferManager;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool, Type as CmdPoolType};
use super::descriptor::Manager as DescriptorManager;
use super::device::logical::Logical as LogicalDevice;
use super::device::physical::Physical as PhysicalDevice;
use super::framebuffer::Framebuffer;
use super::image::View as ImageView;
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
use std::sync::{Arc, RwLock};

const GBUFF_COLOR_FMT: vk::VkFormat = vk::VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT;
const GBUFF_DEPTH_FMT: vk::VkFormat = vk::VkFormat::VK_FORMAT_D32_SFLOAT;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {
    pub(crate) os_app: Arc<RwLock<OsApp>>,
    pub(crate) instance: Arc<Instance>,
    pub(crate) surface: Arc<Surface>,
    pub(crate) physical_device: Arc<PhysicalDevice>,
    pub(crate) logical_device: Arc<LogicalDevice>,
    pub(crate) swapchain: Arc<Swapchain>,
    pub(crate) present_complete_semaphore: Arc<Semaphore>,
    pub(crate) gbuff_complete_semaphore: Arc<Semaphore>,
    pub(crate) render_complete_semaphore: Arc<Semaphore>,
    pub(crate) graphic_cmd_pool: Arc<CmdPool>,
    // pub(crate) draw_commands: Vec<Arc<RwLock<(CmdBuffer, CmdBuffer)>>>, // gbuff, deferred
    pub(crate) memory_mgr: Arc<RwLock<MemoryManager>>,
    pub(crate) samples_count: vk::VkSampleCountFlagBits,
    pub(crate) depth_stencil_image_view: Arc<ImageView>,
    pub(crate) render_pass: Arc<RenderPass>,
    pub(crate) g_render_pass: Arc<RenderPass>,
    pub(crate) g_framebuffer: Arc<Framebuffer>,
    pub(crate) descriptor_manager: Arc<RwLock<DescriptorManager>>,
    pub(crate) pipeline_manager: Arc<RwLock<PipelineManager>>,
    pub(crate) framebuffers: Vec<Arc<Framebuffer>>,
    pub(crate) frame_number: Arc<RwLock<u32>>,
    pub(crate) current_frame_number: u32,
    pub(crate) buffer_manager: Arc<RwLock<BufferManager>>,
    pub(crate) wait_fences: Vec<Arc<Fence>>,
    pub(crate) sampler: Arc<Sampler>,
    pub(crate) bound_gbuff_descriptor_sets: [vk::VkDescriptorSet; 3],
    pub(crate) bound_gbuff_dynamic_offsets: [u32; 3],
    pub(crate) bound_deferred_descriptor_sets: [vk::VkDescriptorSet; 2],
    pub(crate) bound_deferred_dynamic_offsets: [u32; 2],
}

impl Engine {
    pub fn new(os_app: &Arc<RwLock<OsApp>>, conf: &Configurations) -> Self {
        let instance = Arc::new(Instance::new());
        let surface = Arc::new(Surface::new(&instance, os_app));
        let physical_device = Arc::new(PhysicalDevice::new(&surface));
        let logical_device = Arc::new(LogicalDevice::new(&physical_device));
        let swapchain = Arc::new(Swapchain::new(&logical_device));
        let present_complete_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let gbuff_complete_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let render_complete_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let graphic_cmd_pool = Arc::new(CmdPool::new(
            logical_device.clone(),
            CmdPoolType::Graphic,
            0,
        ));
        // let mut draw_commands = Vec::new();
        let mut wait_fences = Vec::new();
        for _ in 0..swapchain.image_views.len() {
            // let draw_command = Arc::new(RwLock::new((
            //     CmdBuffer::new(graphic_cmd_pool.clone()),
            //     CmdBuffer::new(graphic_cmd_pool.clone()),
            // )));
            // draw_commands.push(draw_command);
            wait_fences.push(Arc::new(Fence::new_signaled(logical_device.clone())));
        }
        // draw_commands.shrink_to_fit();
        wait_fences.shrink_to_fit();
        let memory_mgr = Arc::new(RwLock::new(MemoryManager::new(&logical_device)));
        let memory_mgr_w = Arc::downgrade(&memory_mgr);
        vxresult!(memory_mgr.write()).set_itself(memory_mgr_w);
        let depth_stencil_image_view = Arc::new(ImageView::new_depth_stencil(
            logical_device.clone(),
            &memory_mgr,
        ));
        let samples_count = Self::get_max_sample_count(&physical_device);
        let render_pass = Arc::new(RenderPass::new(&swapchain));
        let (g_render_pass, g_framebuffer) =
            Self::create_gbuffer_filler(&logical_device, &memory_mgr, samples_count);
        let mut framebuffers = Vec::new();
        for v in &swapchain.image_views {
            framebuffers.push(Arc::new(Framebuffer::new(
                vec![v.clone()],
                depth_stencil_image_view.clone(),
                render_pass.clone(),
            )));
        }
        framebuffers.shrink_to_fit();
        let frame_number = Arc::new(RwLock::new(0));
        let sampler = Arc::new(Sampler::new(logical_device.clone()));
        let buffer_manager = Arc::new(RwLock::new(BufferManager::new(
            &memory_mgr,
            &graphic_cmd_pool,
            &frame_number,
            32 * 1024 * 1024,
            32 * 1024 * 1024,
            16 * 1024 * 1024,
            swapchain.image_views.len() as isize,
        )));
        let descriptor_manager = Arc::new(RwLock::new(DescriptorManager::new(
            &buffer_manager,
            &logical_device,
            conf,
        )));
        let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(
            &logical_device,
            descriptor_manager.clone(),
            render_pass.clone(),
            g_render_pass.clone(),
            samples_count,
        )));
        let os_app = os_app.clone();
        Engine {
            os_app,
            instance,
            surface,
            physical_device,
            logical_device,
            swapchain,
            present_complete_semaphore,
            gbuff_complete_semaphore,
            render_complete_semaphore,
            graphic_cmd_pool,
            // draw_commands,
            memory_mgr,
            depth_stencil_image_view,
            samples_count,
            render_pass,
            g_render_pass,
            g_framebuffer,
            descriptor_manager,
            pipeline_manager,
            framebuffers,
            frame_number,
            current_frame_number: 0,
            buffer_manager,
            wait_fences,
            sampler,
            bound_gbuff_descriptor_sets: [0 as vk::VkDescriptorSet; 3],
            bound_gbuff_dynamic_offsets: [0; 3],
            bound_deferred_descriptor_sets: [0 as vk::VkDescriptorSet; 2],
            bound_deferred_dynamic_offsets: [0; 2],
        }
    }

    fn wait_for_preset_frame(&mut self) -> usize {
        let current_buffer = match self
            .swapchain
            .get_next_image_index(&self.present_complete_semaphore)
        {
            NextImageResult::Next(c) => c,
            NextImageResult::NeedsRefresh => {
                vxlogf!("Problem with rereshing screen, engine needs refreshing.");
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
        self.current_frame_number = current_buffer as u32;
        return current_buffer;
    }

    // pub fn start_recording(&mut self) {
    //     vxresult!(self.buffer_manager.write()).update();
    //     let frame_number = self.wait_for_preset_frame();

    //     let mut cmd_buffers = vxresult!(self.draw_commands[frame_number].write());
    //     cmd_buffers.0.begin();

    //     self.g_framebuffer.begin_render(&mut cmd_buffers.0);
    // }

    // pub fn start_deferred(&self) {
    //     let mut cmd_buffers =
    //         vxresult!(self.draw_commands[self.current_frame_number as usize].write());

    //     self.g_framebuffer.end_render(&mut cmd_buffers.0);

    //     cmd_buffers.0.end();

    //     let wait_stage_mask =
    //         vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
    //     let mut submit_info = vk::VkSubmitInfo::default();
    //     submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
    //     submit_info.pWaitDstStageMask = &wait_stage_mask;
    //     submit_info.pWaitSemaphores = &self.present_complete_semaphore.vk_data;
    //     submit_info.waitSemaphoreCount = 1;
    //     submit_info.pSignalSemaphores = &self.gbuff_complete_semaphore.vk_data;
    //     submit_info.signalSemaphoreCount = 1;
    //     cmd_buffers.0.fill_submit_info(&mut submit_info);
    //     vulkan_check!(vk::vkQueueSubmit(
    //         self.logical_device.vk_graphic_queue,
    //         1,
    //         &submit_info,
    //         0 as vk::VkFence,
    //     ));

    //     cmd_buffers.1.begin();
    //     self.framebuffers[self.current_frame_number as usize].begin_render(&mut cmd_buffers.1);
    // }

    // pub fn end_recording(&mut self) {
    //     let frame_number = self.current_frame_number as usize;

    //     let mut cmd_buffers = vxresult!(self.draw_commands[frame_number].write());

    //     let pipemgr = vxresult!(self.pipeline_manager.read());
    //     // cmd_buffers.1.bind_descriptor_sets(
    //     //     &pipemgr.deferred_pipeline.layout,
    //     //     &self.bound_deferred_descriptor_sets,
    //     //     &self.bound_deferred_dynamic_offsets,
    //     // );
    //     cmd_buffers.1.bind_pipeline(&pipemgr.deferred_pipeline);
    //     cmd_buffers.1.draw(3);
    //     self.framebuffers[frame_number].end_render(&mut cmd_buffers.1);
    //     cmd_buffers.1.end();

    //     let wait_stage_mask =
    //         vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
    //     let mut submit_info = vk::VkSubmitInfo::default();
    //     submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
    //     submit_info.pWaitDstStageMask = &wait_stage_mask;
    //     submit_info.pWaitSemaphores = &self.gbuff_complete_semaphore.vk_data;
    //     submit_info.waitSemaphoreCount = 1;
    //     submit_info.pSignalSemaphores = &self.render_complete_semaphore.vk_data;
    //     submit_info.signalSemaphoreCount = 1;
    //     cmd_buffers.1.fill_submit_info(&mut submit_info);
    //     vulkan_check!(vk::vkQueueSubmit(
    //         self.logical_device.vk_graphic_queue,
    //         1,
    //         &submit_info,
    //         self.wait_fences[frame_number].vk_data,
    //     ));

    //     let image_index = frame_number as u32;
    //     let mut present_info = vk::VkPresentInfoKHR::default();
    //     present_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
    //     present_info.swapchainCount = 1;
    //     present_info.pSwapchains = &self.swapchain.vk_data;
    //     present_info.pImageIndices = &image_index;
    //     present_info.pWaitSemaphores = &self.render_complete_semaphore.vk_data;
    //     present_info.waitSemaphoreCount = 1;
    //     vulkan_check!(vk::vkQueuePresentKHR(
    //         self.logical_device.vk_graphic_queue,
    //         &present_info,
    //     ));
    // }

    pub fn terminate(&mut self) {
        self.logical_device.wait_idle();
    }

    // pub fn bind_gbuff_descriptor(
    //     &mut self,
    //     descriptor_set: &DescriptorSet,
    //     uniform_buffer: &DynamicBuffer,
    //     index: usize,
    // ) {
    //     let frame_number: usize = *vxresult!(self.frame_number.read()) as usize; // todo temporary
    //     self.bound_gbuff_descriptor_sets[index] = descriptor_set.vk_data;
    //     self.bound_gbuff_dynamic_offsets[index] =
    //         vxresult!(uniform_buffer.buffers[frame_number].0.read())
    //             .info
    //             .base
    //             .offset as u32; // todo move this to dynamicbuffer
    // }

    // pub fn bind_deferred_descriptor(
    //     &mut self,
    //     descriptor_set: &DescriptorSet,
    //     uniform_buffer: &DynamicBuffer,
    //     index: usize,
    // ) {
    //     let frame_number = self.current_frame_number as usize;
    //     self.bound_deferred_descriptor_sets[index] = descriptor_set.vk_data;
    //     self.bound_deferred_dynamic_offsets[index] =
    //         vxresult!(uniform_buffer.buffers[frame_number].0.read())
    //             .info
    //             .base
    //             .offset as u32; // todo move this to dynamicbuffer
    // }

    // pub fn bind_gbuff_pipeline(&self) {
    //     let frame_number: usize = *vxresult!(self.frame_number.read()) as usize;
    //     let draw_command = &self.draw_commands[frame_number];
    //     let mut draw_command = vxresult!(draw_command.write());
    //     let pipemgr = vxresult!(self.pipeline_manager.read());
    //     draw_command.0.bind_pipeline(&pipemgr.gbuff_pipeline);
    // }

    // pub fn render_gbuff(
    //     &self,
    //     vertex_buffer: &StaticBuffer,
    //     index_buffer: &StaticBuffer,
    //     indices_count: u32,
    // ) {
    //     let frame_number: usize = *vxresult!(self.frame_number.read()) as usize;
    //     let draw_command = &self.draw_commands[frame_number];
    //     let mut draw_command = vxresult!(draw_command.write());
    //     let pipemgr = vxresult!(self.pipeline_manager.read());
    //     draw_command.0.bind_descriptor_sets(
    //         &pipemgr.gbuff_pipeline.layout,
    //         &self.bound_gbuff_descriptor_sets,
    //         &self.bound_gbuff_dynamic_offsets,
    //     );
    //     draw_command.0.bind_vertex_buffer(&vertex_buffer.buffer);
    //     draw_command.0.bind_index_buffer(&index_buffer.buffer);
    //     draw_command.0.draw_index(indices_count);
    // }

    // pub fn create_texture(&self, file_name: &str) -> Arc<ImageView> {
    //     Arc::new(ImageView::new_texture_with_file(
    //         file_name,
    //         &self.buffer_manager,
    //     ))
    // }

    // pub fn create_texture_with_bytes(&self, data: &[u8]) -> Arc<ImageView> {
    //     Arc::new(ImageView::new_texture_with_bytes(
    //         data,
    //         &self.buffer_manager,
    //     ))
    // }

    pub fn create_texture_2d_with_pixels(
        &self,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Arc<ImageView> {
        Arc::new(ImageView::new_texture_2d_with_pixels(
            width,
            height,
            data,
            &self.buffer_manager,
        ))
    }

    pub fn reinitialize(&mut self, conf: &Configurations) {
        self.logical_device.wait_idle();
        let new = Self::new(&self.os_app, conf);
        self.instance = new.instance.clone();
        self.surface = new.surface.clone();
        self.physical_device = new.physical_device.clone();
        self.logical_device = new.logical_device.clone();
        self.swapchain = new.swapchain.clone();
        self.present_complete_semaphore = new.present_complete_semaphore.clone();
        self.render_complete_semaphore = new.render_complete_semaphore.clone();
        self.graphic_cmd_pool = new.graphic_cmd_pool.clone();
        // self.draw_commands = new.draw_commands.clone();
        self.memory_mgr = new.memory_mgr.clone();
        self.depth_stencil_image_view = new.depth_stencil_image_view.clone();
        self.render_pass = new.render_pass.clone();
        self.pipeline_manager = new.pipeline_manager.clone();
        self.framebuffers = new.framebuffers.clone();
        self.frame_number = new.frame_number.clone();
        self.buffer_manager = new.buffer_manager.clone();
        self.wait_fences = new.wait_fences.clone();
    }

    fn create_gbuffer_filler(
        logical_device: &Arc<LogicalDevice>,
        memory_manager: &Arc<RwLock<MemoryManager>>,
        sample_count: vk::VkSampleCountFlagBits,
    ) -> (Arc<RenderPass>, Arc<Framebuffer>) {
        let g_pos = Arc::new(ImageView::new_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_COLOR_FMT,
            sample_count,
            AttachmentType::ColorGBuffer,
        ));
        let g_nrm = Arc::new(ImageView::new_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_COLOR_FMT,
            sample_count,
            AttachmentType::ColorGBuffer,
        ));
        let g_alb = Arc::new(ImageView::new_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_COLOR_FMT,
            sample_count,
            AttachmentType::ColorGBuffer,
        ));
        let g_dpt = Arc::new(ImageView::new_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_DEPTH_FMT,
            sample_count,
            AttachmentType::DepthGBuffer,
        ));
        let views = vec![g_pos.clone(), g_nrm.clone(), g_alb.clone(), g_dpt.clone()];
        let g_render_pass = Arc::new(RenderPass::new_with_views(views));
        let views = vec![g_pos, g_nrm, g_alb];
        let g_framebuffer = Arc::new(Framebuffer::new(views, g_dpt, g_render_pass.clone()));
        return (g_render_pass, g_framebuffer);
    }

    pub(crate) fn create_command_pool(&self) -> Arc<CmdPool> {
        return Arc::new(CmdPool::new(
            self.logical_device.clone(),
            CmdPoolType::Graphic,
            0,
        ));
    }

    pub(crate) fn create_secondary_command_buffer(&self, cmd_pool: Arc<CmdPool>) -> CmdBuffer {
        return CmdBuffer::new_secondary(cmd_pool);
    }

    pub(crate) fn create_primary_command_buffer(&self, cmd_pool: Arc<CmdPool>) -> CmdBuffer {
        return CmdBuffer::new_primary(cmd_pool);
    }

    pub(crate) fn get_frames_count(&self) -> usize {
        return self.framebuffers.len();
    }

    pub(crate) fn get_frame_number(&self) -> usize {
        return self.current_frame_number as usize;
    }

    pub(crate) fn get_gbuff_framebuffer(&self) -> &Arc<Framebuffer> {
        return &self.g_framebuffer;
    }

    fn get_max_sample_count(phdev: &Arc<PhysicalDevice>) -> vk::VkSampleCountFlagBits {
        let mut sample_count = phdev.get_max_sample_bit(
            GBUFF_COLOR_FMT,
            vk::VkImageType::VK_IMAGE_TYPE_2D,
            vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
            vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as vk::VkImageUsageFlags
                | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as vk::VkImageUsageFlags,
            0,
        );
        sample_count &= phdev.get_max_sample_bit(
            GBUFF_DEPTH_FMT,
            vk::VkImageType::VK_IMAGE_TYPE_2D,
            vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
            vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT
                as vk::VkImageUsageFlags
                | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as vk::VkImageUsageFlags,
            0,
        );
        let result = phdev.get_max_sample_bit_with_mask(sample_count);
        vxlogi!("Sample count is: {:?}", result);
        return result;
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.logical_device.wait_idle();
    }
}

unsafe impl Send for Engine {}

unsafe impl Sync for Engine {}
