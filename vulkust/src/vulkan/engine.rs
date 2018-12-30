use super::super::render::config::Configurations;
use super::super::render::sampler::Filter as SamplerFilter;
use super::super::system::os::application::Application as OsApp;
use super::buffer::Manager as BufferManager;
use super::command::{Buffer as CmdBuffer, Pool as CmdPool, Type as CmdPoolType};
use super::descriptor::Manager as DescriptorManager;
use super::device::Logical as LogicalDevice;
use super::device::Physical as PhysicalDevice;
use super::framebuffer::Framebuffer;
use super::image::View as ImageView;
use super::instance::Instance;
use super::memory::Manager as MemoryManager;
use super::pipeline::Manager as PipelineManager;
use super::render_pass::RenderPass;
use super::sampler::Sampler;
use super::surface::Surface;
use super::swapchain::{NextImageResult, Swapchain};
use super::sync::Fence;
use super::sync::Semaphore;
use super::vulkan as vk;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {
    os_app: Arc<RwLock<OsApp>>,
    instance: Arc<Instance>,
    surface: Arc<Surface>,
    physical_device: Arc<PhysicalDevice>,
    logical_device: Arc<LogicalDevice>,
    swapchain: Arc<Swapchain>,
    present_semaphore: Arc<Semaphore>,
    data_semaphore: Arc<Semaphore>,
    render_semaphore: Arc<Semaphore>,
    graphic_cmd_pool: Arc<CmdPool>,
    data_primary_cmds: Vec<Arc<Mutex<CmdBuffer>>>,
    memory_manager: Arc<RwLock<MemoryManager>>,
    buffer_manager: Arc<RwLock<BufferManager>>,
    descriptor_manager: Arc<RwLock<DescriptorManager>>,
    pipeline_manager: Arc<RwLock<PipelineManager>>,
    wait_fences: Vec<Arc<Fence>>,
    linear_repeat_sampler: Arc<Sampler>,
    nearest_repeat_sampler: Arc<Sampler>,
    //---------------------------------------
    clear_render_pass: Arc<RenderPass>,
    clear_framebuffers: Vec<Arc<Framebuffer>>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    //---------------------------------------
    current_frame_number: u32,
}

impl Engine {
    pub(crate) fn new(os_app: &Arc<RwLock<OsApp>>, conf: &Configurations) -> Self {
        let instance = Arc::new(Instance::new());
        let surface = Arc::new(Surface::new(&instance, os_app));
        let physical_device = Arc::new(PhysicalDevice::new(&surface));
        let logical_device = Arc::new(LogicalDevice::new(&physical_device, conf));
        let swapchain = Arc::new(Swapchain::new(&logical_device));
        let present_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let data_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let render_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        let graphic_cmd_pool = Arc::new(CmdPool::new(
            logical_device.clone(),
            CmdPoolType::Graphic,
            0,
        ));
        let mut data_primary_cmds = Vec::with_capacity(swapchain.image_views.len());
        let mut wait_fences = Vec::with_capacity(swapchain.image_views.len());
        for _ in 0..swapchain.image_views.len() {
            data_primary_cmds.push(Arc::new(Mutex::new(CmdBuffer::new_primary(
                graphic_cmd_pool.clone(),
            ))));
            wait_fences.push(Arc::new(Fence::new_signaled(logical_device.clone())));
        }
        let memory_manager = MemoryManager::new(&logical_device);
        let render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), false));
        let clear_render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), true));
        let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());
        let mut clear_framebuffers = Vec::with_capacity(swapchain.image_views.len());
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
        let linear_repeat_sampler = Arc::new(Sampler::new(logical_device.clone()));
        let nearest_repeat_sampler = Arc::new(Sampler::new_with_filter(
            logical_device.clone(),
            SamplerFilter::Nearest,
        ));
        let buffer_manager = Arc::new(RwLock::new(BufferManager::new(
            &memory_manager,
            &graphic_cmd_pool,
            32 * 1024 * 1024,
            32 * 1024 * 1024,
            32 * 1024 * 1024,
            swapchain.image_views.len() as isize,
        )));
        let descriptor_manager =
            Arc::new(RwLock::new(DescriptorManager::new(&logical_device, conf)));
        let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(
            logical_device.clone(),
            descriptor_manager.clone(),
        )));
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
            memory_manager,
            render_pass,
            clear_render_pass,
            descriptor_manager,
            pipeline_manager,
            framebuffers,
            clear_framebuffers,
            current_frame_number: 0,
            buffer_manager,
            wait_fences,
            linear_repeat_sampler,
            nearest_repeat_sampler,
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
            self.logical_device.get_data(),
            1,
            &self.wait_fences[current_buffer].vk_data,
            1,
            u64::max_value(),
        ));
        vulkan_check!(vk::vkResetFences(
            self.logical_device.get_data(),
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
        self.submit_with_fence(
            &[wait.get_data()],
            &[cmd.get_data()],
            &[signal.get_data()],
            None,
        );
    }

    pub(crate) fn submit_with_fence(
        &self,
        waits: &[vk::VkSemaphore],
        cmds: &[vk::VkCommandBuffer],
        signals: &[vk::VkSemaphore],
        fence: Option<&Fence>,
    ) {
        let wait_stage_mask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        let mut submit_info = vk::VkSubmitInfo::default();
        submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit_info.pWaitDstStageMask = &wait_stage_mask;
        submit_info.pWaitSemaphores = waits.as_ptr();
        submit_info.waitSemaphoreCount = waits.len() as u32;
        submit_info.pSignalSemaphores = signals.as_ptr();
        submit_info.signalSemaphoreCount = signals.len() as u32;
        submit_info.pCommandBuffers = cmds.as_ptr();
        submit_info.commandBufferCount = cmds.len() as u32;
        let fence = if let Some(fence) = fence {
            fence.vk_data
        } else {
            null_mut()
        };
        vulkan_check!(vk::vkQueueSubmit(
            self.logical_device.get_vk_graphic_queue(),
            1,
            &submit_info,
            fence,
        ));
    }

    pub(crate) fn submit_multiple(
        &self,
        waits: &[&Semaphore],
        cmds: &[&CmdBuffer],
        signals: &[&Semaphore],
    ) {
        let mut waits_data = Vec::with_capacity(waits.len());
        let mut signals_data = Vec::with_capacity(signals.len());
        let mut cmds_data = Vec::with_capacity(cmds.len());
        for w in waits {
            waits_data.push(w.get_data());
        }
        for s in signals {
            signals_data.push(s.get_data());
        }
        for c in cmds {
            cmds_data.push(c.get_data());
        }
        self.submit_with_fence(&waits_data, &cmds_data, &signals_data, None);
    }

    pub(crate) fn end(&self, wait: &Semaphore) {
        self.submit_with_fence(
            &[wait.get_data()],
            &[],
            &[self.render_semaphore.get_data()],
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
            self.logical_device.get_vk_graphic_queue(),
            &present_info,
        ));
    }

    // pub(crate) fn terminate(&mut self) {
    //     self.logical_device.wait_idle();
    // }

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

    pub(crate) fn create_primary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
        return CmdBuffer::new_primary(self.graphic_cmd_pool.clone());
    }

    pub(crate) fn create_secondary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
        return CmdBuffer::new_secondary(self.graphic_cmd_pool.clone());
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

    pub(crate) fn get_current_framebuffer(&self) -> &Arc<Framebuffer> {
        return &self.framebuffers[self.current_frame_number as usize];
    }

    pub(crate) fn get_starting_semaphore(&self) -> &Arc<Semaphore> {
        return &self.data_semaphore;
    }

    pub(crate) fn get_device(&self) -> &Arc<LogicalDevice> {
        return &self.logical_device;
    }

    pub(crate) fn get_memory_manager(&self) -> &Arc<RwLock<MemoryManager>> {
        return &self.memory_manager;
    }

    pub(crate) fn get_buffer_manager(&self) -> &Arc<RwLock<BufferManager>> {
        return &self.buffer_manager;
    }

    pub(crate) fn get_descriptor_manager(&self) -> &Arc<RwLock<DescriptorManager>> {
        return &self.descriptor_manager;
    }

    pub(crate) fn get_pipeline_manager(&self) -> &Arc<RwLock<PipelineManager>> {
        return &self.pipeline_manager;
    }

    pub(crate) fn get_linear_repeat_sampler(&self) -> &Arc<Sampler> {
        return &self.linear_repeat_sampler;
    }

    pub(crate) fn get_nearest_repeat_sampler(&self) -> &Arc<Sampler> {
        return &self.nearest_repeat_sampler;
    }

    pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
        return &self.render_pass;
    }

    pub(crate) fn get_framebuffers(&self) -> &Vec<Arc<Framebuffer>> {
        return &self.framebuffers;
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.logical_device.wait_idle();
    }
}

unsafe impl Send for Engine {}

unsafe impl Sync for Engine {}
