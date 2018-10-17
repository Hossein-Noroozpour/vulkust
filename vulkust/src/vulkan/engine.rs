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
use super::pipeline::{Manager as PipelineManager, Pipeline};
use super::render_pass::RenderPass;
use super::sampler::Sampler;
use super::surface::Surface;
use super::swapchain::{NextImageResult, Swapchain};
use super::sync::Fence;
use super::sync::Semaphore;
use super::vulkan as vk;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, RwLock};

const GBUFF_COLOR_FMT: vk::VkFormat = vk::VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT;
const GBUFF_DEPTH_FMT: vk::VkFormat = vk::VkFormat::VK_FORMAT_D32_SFLOAT;
const SHADOW_MAP_FMT: vk::VkFormat = vk::VkFormat::VK_FORMAT_D32_SFLOAT;
const SHADOW_ACCUMULATOR_FMT: vk::VkFormat = vk::VkFormat::VK_FORMAT_R32_SFLOAT;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {
    pub(crate) os_app: Arc<RwLock<OsApp>>,
    pub(crate) instance: Arc<Instance>,
    pub(crate) surface: Arc<Surface>,
    pub(crate) physical_device: Arc<PhysicalDevice>,
    pub(crate) logical_device: Arc<LogicalDevice>,
    pub(crate) swapchain: Arc<Swapchain>,
    present_semaphore: Arc<Semaphore>,
    data_semaphore: Arc<Semaphore>,
    render_semaphore: Arc<Semaphore>,
    graphic_cmd_pool: Arc<CmdPool>,
    data_primary_cmds: Vec<Arc<Mutex<CmdBuffer>>>,
    pub(crate) memory_mgr: Arc<RwLock<MemoryManager>>,
    pub(crate) buffer_manager: Arc<RwLock<BufferManager>>,
    pub(crate) descriptor_manager: Arc<RwLock<DescriptorManager>>,
    pub(crate) pipeline_manager: Arc<RwLock<PipelineManager>>,
    pub(crate) wait_fences: Vec<Arc<Fence>>,
    pub(crate) sampler: Arc<Sampler>,
    pub(crate) samples_count: vk::VkSampleCountFlagBits,
    //---------------------------------------
    pub(crate) g_render_pass: Arc<RenderPass>,
    pub(crate) g_framebuffer: Arc<Framebuffer>,
    //---------------------------------------
    clear_render_pass: Arc<RenderPass>,
    pub(crate) clear_framebuffers: Vec<Arc<Framebuffer>>,
    pub(crate) render_pass: Arc<RenderPass>,
    pub(crate) framebuffers: Vec<Arc<Framebuffer>>,
    //---------------------------------------
    shadow_map_buffers: Vec<Arc<ImageView>>,
    shadow_map_render_pass: Arc<RenderPass>,
    shadow_map_framebuffers: Vec<Arc<Framebuffer>>,
    //---------------------------------------
    black_accumulator_buffer: Arc<ImageView>,
    black_accumulator_render_pass: Arc<RenderPass>,
    black_accumulator_framebuffer: Arc<Framebuffer>,
    clear_black_accumulator_render_pass: Arc<RenderPass>,
    clear_black_accumulator_framebuffer: Arc<Framebuffer>,
    //---------------------------------------
    resolver_buffers: Vec<Arc<ImageView>>,
    resolver_render_pass: Arc<RenderPass>,
    resolver_framebuffer: Arc<Framebuffer>,
    //---------------------------------------
    pub(crate) current_frame_number: u32,
}

impl Engine {
    pub(crate) fn new(os_app: &Arc<RwLock<OsApp>>, conf: &Configurations) -> Self {
        let instance = Arc::new(Instance::new());
        let surface = Arc::new(Surface::new(&instance, os_app));
        let physical_device = Arc::new(PhysicalDevice::new(&surface));
        let logical_device = Arc::new(LogicalDevice::new(&physical_device));
        let swapchain = Arc::new(Swapchain::new(&logical_device));
        let present_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let data_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let render_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let graphic_cmd_pool = Arc::new(CmdPool::new(
            logical_device.clone(),
            CmdPoolType::Graphic,
            0,
        ));
        let mut data_primary_cmds = Vec::new();
        let mut wait_fences = Vec::new();
        for _ in 0..swapchain.image_views.len() {
            data_primary_cmds.push(Arc::new(Mutex::new(CmdBuffer::new_primary(
                graphic_cmd_pool.clone(),
            ))));
            wait_fences.push(Arc::new(Fence::new_signaled(logical_device.clone())));
        }
        wait_fences.shrink_to_fit();
        data_primary_cmds.shrink_to_fit();
        let memory_mgr = Arc::new(RwLock::new(MemoryManager::new(&logical_device)));
        let memory_mgr_w = Arc::downgrade(&memory_mgr);
        vxresult!(memory_mgr.write()).set_itself(memory_mgr_w);
        let samples_count = Self::get_max_sample_count(&physical_device);
        let render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), false));
        let clear_render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), true));
        let (g_render_pass, g_framebuffer) =
            Self::create_gbuffer_filler(&logical_device, &memory_mgr, samples_count);
        let mut framebuffers = Vec::new();
        let mut clear_framebuffers = Vec::new();
        for v in &swapchain.image_views {
            framebuffers.push(Arc::new(Framebuffer::new(
                vec![v.clone()],
                render_pass.clone(),
            )));
            clear_framebuffers.push(Arc::new(Framebuffer::new(
                vec![v.clone()],
                clear_render_pass.clone(),
            )));
        }
        framebuffers.shrink_to_fit();
        clear_framebuffers.shrink_to_fit();
        let sampler = Arc::new(Sampler::new(logical_device.clone()));
        let buffer_manager = Arc::new(RwLock::new(BufferManager::new(
            &memory_mgr,
            &graphic_cmd_pool,
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
        let mut shadow_map_buffers = Vec::new();
        for _ in 0..conf.max_shadow_maps_count {
            shadow_map_buffers.push(Arc::new(ImageView::new_attachment(
                logical_device.clone(),
                &memory_mgr,
                SHADOW_MAP_FMT,
                vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                AttachmentType::DepthShadowBuffer,
                conf.shadow_map_aspect,
                conf.shadow_map_aspect,
            )));
        }
        shadow_map_buffers.shrink_to_fit();
        let black_accumulator_buffer = Arc::new(ImageView::new_surface_attachment(
            logical_device.clone(),
            &memory_mgr,
            SHADOW_ACCUMULATOR_FMT,
            vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
            AttachmentType::ColorDisplay,
        ));
        let clear_black_accumulator_render_pass = Arc::new(RenderPass::new(
            vec![black_accumulator_buffer.clone()],
            true,
            false,
        ));
        let black_accumulator_render_pass = Arc::new(RenderPass::new(
            vec![black_accumulator_buffer.clone()],
            false,
            true,
        ));
        let shadow_map_render_pass = Arc::new(RenderPass::new(
            vec![shadow_map_buffers[0].clone()],
            true,
            true,
        ));
        let clear_black_accumulator_framebuffer = Arc::new(Framebuffer::new(
            vec![black_accumulator_buffer.clone()],
            black_accumulator_render_pass.clone(),
        ));
        let black_accumulator_framebuffer = Arc::new(Framebuffer::new(
            vec![black_accumulator_buffer.clone()],
            clear_black_accumulator_render_pass.clone(),
        ));
        let mut shadow_map_framebuffers = Vec::new();
        for v in &shadow_map_buffers {
            shadow_map_framebuffers.push(Arc::new(Framebuffer::new(
                vec![v.clone()],
                shadow_map_render_pass.clone(),
            )));
        }
        shadow_map_buffers.shrink_to_fit();
        let resolver_buffers = vec![
            Arc::new(ImageView::new_surface_attachment(
                logical_device.clone(),
                &memory_mgr,
                GBUFF_COLOR_FMT,
                vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                AttachmentType::ResolverBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                logical_device.clone(),
                &memory_mgr,
                GBUFF_COLOR_FMT,
                vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                AttachmentType::ResolverBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                logical_device.clone(),
                &memory_mgr,
                GBUFF_COLOR_FMT,
                vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                AttachmentType::ResolverBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                logical_device.clone(),
                &memory_mgr,
                SHADOW_ACCUMULATOR_FMT,
                vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                AttachmentType::ResolverBuffer,
            )),
        ];
        let resolver_render_pass = Arc::new(RenderPass::new(resolver_buffers.clone(), true, true));
        let resolver_framebuffers = Arc::new(Framebuffer::new(resolver_buffers.clone(), resolver_render_pass.clone()));
        let os_app = os_app.clone();
        Engine {
            os_app,
            instance,
            surface,
            physical_device,
            logical_device,
            swapchain,
            present_semaphore,
            data_semaphore,
            render_semaphore,
            graphic_cmd_pool,
            data_primary_cmds,
            memory_mgr,
            samples_count,
            render_pass,
            clear_render_pass,
            g_render_pass,
            g_framebuffer,
            descriptor_manager,
            pipeline_manager,
            framebuffers,
            clear_framebuffers,
            current_frame_number: 0,
            buffer_manager,
            wait_fences,
            sampler,
            shadow_map_buffers,
            black_accumulator_buffer,
            clear_black_accumulator_render_pass,
            black_accumulator_render_pass,
            shadow_map_render_pass,
            clear_black_accumulator_framebuffer,
            black_accumulator_framebuffer,
            shadow_map_framebuffers,
            resolver_buffers,
            resolver_render_pass,
            resolver_framebuffers,
        }
    }

    pub(crate) fn start_rendering(&mut self) {
        let current_buffer = match self.swapchain.get_next_image_index(&self.present_semaphore) {
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
        self.current_frame_number = current_buffer as u32;

        let mut pcmd = vxresult!(self.data_primary_cmds[self.current_frame_number as usize].lock());
        pcmd.begin();
        vxresult!(self.buffer_manager.write())
            .update(&mut *pcmd, self.current_frame_number as usize);
        self.clear_framebuffers[self.current_frame_number as usize].begin(&mut *pcmd);
        pcmd.end_render_pass();
        pcmd.end();
        self.submit(&self.present_semaphore, &pcmd, &self.data_semaphore);
    }

    pub(crate) fn submit(&self, wait: &Semaphore, cmd: &CmdBuffer, signal: &Semaphore) {
        self.submit_with_fence(wait, Some(cmd), signal, None);
    }

    pub(crate) fn submit_with_fence(
        &self,
        wait: &Semaphore,
        cmd: Option<&CmdBuffer>,
        signal: &Semaphore,
        fence: Option<&Fence>,
    ) {
        let wait_stage_mask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        let mut submit_info = vk::VkSubmitInfo::default();
        submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit_info.pWaitDstStageMask = &wait_stage_mask;
        submit_info.pWaitSemaphores = &wait.vk_data;
        submit_info.waitSemaphoreCount = 1;
        submit_info.pSignalSemaphores = &signal.vk_data;
        submit_info.signalSemaphoreCount = 1;
        if cmd.is_some() {
            let cmd = vxunwrap!(cmd);
            cmd.fill_submit_info(&mut submit_info);
        }
        let fence = if fence.is_some() {
            vxunwrap!(fence).vk_data
        } else {
            null_mut()
        };
        vulkan_check!(vk::vkQueueSubmit(
            self.logical_device.vk_graphic_queue,
            1,
            &submit_info,
            fence,
        ));
    }

    pub(crate) fn end(&self, wait: &Semaphore) {
        self.submit_with_fence(
            wait,
            None,
            &self.render_semaphore,
            Some(&self.wait_fences[self.current_frame_number as usize]),
        );

        let mut present_info = vk::VkPresentInfoKHR::default();
        present_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
        present_info.swapchainCount = 1;
        present_info.pSwapchains = &self.swapchain.vk_data;
        present_info.pImageIndices = &self.current_frame_number;
        present_info.pWaitSemaphores = &self.render_semaphore.vk_data;
        present_info.waitSemaphoreCount = 1;
        vulkan_check!(vk::vkQueuePresentKHR(
            self.logical_device.vk_graphic_queue,
            &present_info,
        ));
    }

    pub(crate) fn terminate(&mut self) {
        self.logical_device.wait_idle();
    }

    pub(crate) fn create_texture_2d_with_pixels(
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

    fn create_gbuffer_filler(
        logical_device: &Arc<LogicalDevice>,
        memory_manager: &Arc<RwLock<MemoryManager>>,
        sample_count: vk::VkSampleCountFlagBits,
    ) -> (Arc<RenderPass>, Arc<Framebuffer>) {
        let g_pos = Arc::new(ImageView::new_surface_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_COLOR_FMT,
            sample_count,
            AttachmentType::ColorGBuffer,
        ));
        let g_nrm = Arc::new(ImageView::new_surface_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_COLOR_FMT,
            sample_count,
            AttachmentType::ColorGBuffer,
        ));
        let g_alb = Arc::new(ImageView::new_surface_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_COLOR_FMT,
            sample_count,
            AttachmentType::ColorGBuffer,
        ));
        let g_dpt = Arc::new(ImageView::new_surface_attachment(
            logical_device.clone(),
            memory_manager,
            GBUFF_DEPTH_FMT,
            sample_count,
            AttachmentType::DepthGBuffer,
        ));
        let views = vec![g_pos.clone(), g_nrm.clone(), g_alb.clone(), g_dpt.clone()];
        let g_render_pass = Arc::new(RenderPass::new(views.clone(), true, true));
        let g_framebuffer = Arc::new(Framebuffer::new(views, g_render_pass.clone()));
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

    pub(crate) fn create_semaphore(&self) -> Semaphore {
        return Semaphore::new(self.logical_device.clone());
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

    pub(crate) fn get_gbuff_pipeline(&self) -> Arc<Pipeline> {
        let pipmgr = vxresult!(self.pipeline_manager.read());
        return pipmgr.gbuff_pipeline.clone();
    }

    pub(crate) fn get_deferred_framebuffer(&self) -> &Arc<Framebuffer> {
        return &self.framebuffers[self.current_frame_number as usize];
    }

    pub(crate) fn get_deferred_pipeline(&self) -> Arc<Pipeline> {
        let pipmgr = vxresult!(self.pipeline_manager.read());
        return pipmgr.deferred_pipeline.clone();
    }

    pub(crate) fn get_starting_semaphore(&self) -> &Arc<Semaphore> {
        return &self.data_semaphore;
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
