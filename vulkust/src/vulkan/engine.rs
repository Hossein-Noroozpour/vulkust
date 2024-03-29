// use super::super::render::sampler::Filter as SamplerFilter;
// use super::super::system::os::application::Application as OsApp;
// use super::buffer::Manager as BufferManager;
// use super::command::{Buffer as CmdBuffer, Pool as CmdPool, Type as CmdPoolType};
// use super::descriptor::Manager as DescriptorManager;
// use super::device::Logical as LogicalDevice;
// use super::device::Physical as PhysicalDevice;
// use super::framebuffer::Framebuffer;
// use super::image::View as ImageView;
use super::{super::platform::os::application::Application as OsApp, instance::Instance};
// use super::memory::Manager as MemoryManager;
// use super::pipeline::Manager as PipelineManager;
// use super::render_pass::RenderPass;
// use super::sampler::Sampler;
// use super::surface::Surface;
// use super::swapchain::{NextImageResult, Swapchain};
// use super::sync::Fence;
// use super::sync::Semaphore;
// use ash::version::DeviceV1_0;
// use ash::vk;
// use std::sync::{Arc, Mutex, RwLock};

pub struct Engine {
    pub instance: Instance,
    // pub surface: Arc<Surface>,
    // pub physical_device: Arc<PhysicalDevice>,
    // pub logical_device: Arc<LogicalDevice>,
    // pub swapchain: Arc<Swapchain>,
    // pub present_semaphore: Arc<Semaphore>,
    // pub data_semaphore: Arc<Semaphore>,
    // pub second_data_semaphore: Arc<Semaphore>,
    // pub render_semaphore: Arc<Semaphore>,
    // pub graphic_cmd_pool: Arc<CmdPool>,
    // pub data_primary_cmds: Vec<Arc<Mutex<CmdBuffer>>>,
    // pub second_data_primary_cmds: Vec<Arc<Mutex<CmdBuffer>>>,
    // pub memory_manager: Arc<RwLock<MemoryManager>>,
    // pub buffer_manager: Arc<RwLock<BufferManager>>,
    // pub descriptor_manager: Arc<RwLock<DescriptorManager>>,
    // pub pipeline_manager: Arc<RwLock<PipelineManager>>,
    // pub wait_fences: Vec<Arc<Fence>>,
    // pub linear_repeat_sampler: Arc<Sampler>,
    // pub nearest_repeat_sampler: Arc<Sampler>,
    // //---------------------------------------
    // pub clear_render_pass: Arc<RenderPass>,
    // pub clear_framebuffers: Vec<Arc<Framebuffer>>,
    // pub render_pass: Arc<RenderPass>,
    // pub framebuffers: Vec<Arc<Framebuffer>>,
    // //---------------------------------------
    // pub current_frame_number: u32,
}

impl Engine {
    pub fn new(os_app: &mut OsApp) -> Self {
        let instance = Instance::new(os_app);
        // let surface = Arc::new(Surface::new(&instance, os_app));
        // let physical_device = Arc::new(PhysicalDevice::new(&surface));
        // let logical_device = Arc::new(LogicalDevice::new(&physical_device, conf.get_render()));
        // let swapchain = Arc::new(Swapchain::new(&logical_device));
        // let present_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        // let data_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        // let second_data_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        // let render_semaphore = Arc::new(Semaphore::new(logical_device.clone()));
        // let graphic_cmd_pool = Arc::new(CmdPool::new(
        //     logical_device.clone(),
        //     CmdPoolType::Graphic,
        //     vk::CommandPoolCreateFlags::empty(),
        // ));
        // let mut data_primary_cmds = Vec::with_capacity(swapchain.get_image_views().len());
        // let mut second_data_primary_cmds = Vec::with_capacity(swapchain.get_image_views().len());
        // let mut wait_fences = Vec::with_capacity(swapchain.get_image_views().len());
        // for _ in 0..swapchain.get_image_views().len() {
        //     data_primary_cmds.push(Arc::new(Mutex::new(CmdBuffer::new_primary(
        //         graphic_cmd_pool.clone(),
        //     ))));
        //     second_data_primary_cmds.push(Arc::new(Mutex::new(CmdBuffer::new_primary(
        //         graphic_cmd_pool.clone(),
        //     ))));
        //     wait_fences.push(Arc::new(Fence::new_signaled(logical_device.clone())));
        // }
        // let memory_manager = MemoryManager::new(&logical_device);
        // let render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), false));
        // let clear_render_pass = Arc::new(RenderPass::new_with_swapchain(swapchain.clone(), true));
        // let mut framebuffers = Vec::with_capacity(swapchain.get_image_views().len());
        // let mut clear_framebuffers = Vec::with_capacity(swapchain.get_image_views().len());
        // for v in swapchain.get_image_views() {
        //     framebuffers.push(Arc::new(Framebuffer::new(
        //         vec![v.clone()],
        //         render_pass.clone(),
        //     )));
        //     clear_framebuffers.push(Arc::new(Framebuffer::new(
        //         vec![v.clone()],
        //         clear_render_pass.clone(),
        //     )));
        // }
        // let linear_repeat_sampler = Arc::new(Sampler::new(logical_device.clone()));
        // let nearest_repeat_sampler = Arc::new(Sampler::new_with_filter(
        //     logical_device.clone(),
        //     SamplerFilter::Nearest,
        // ));
        // let buffer_manager = Arc::new(RwLock::new(BufferManager::new(
        //     &memory_manager,
        //     &graphic_cmd_pool,
        //     8 * 1024 * 1024,
        //     8 * 1024 * 1024,
        //     8 * 1024 * 1024,
        //     swapchain.get_image_views().len() as isize,
        // )));
        // let descriptor_manager = Arc::new(RwLock::new(DescriptorManager::new(
        //     &logical_device,
        //     conf.get_render(),
        // )));
        // let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(
        //     logical_device.clone(),
        //     descriptor_manager.clone(),
        // )));
        // let os_app = os_app.clone();
        Self {
            instance,
            // surface,
            // physical_device,
            // logical_device,
            // swapchain,
            // present_semaphore,
            // data_semaphore,
            // second_data_semaphore,
            // render_semaphore,
            // graphic_cmd_pool,
            // data_primary_cmds,
            // second_data_primary_cmds,
            // memory_manager,
            // render_pass,
            // clear_render_pass,
            // descriptor_manager,
            // pipeline_manager,
            // framebuffers,
            // clear_framebuffers,
            // current_frame_number: 0,
            // buffer_manager,
            // wait_fences,
            // linear_repeat_sampler,
            // nearest_repeat_sampler,
        }
    }

    // pub(crate) fn start_rendering(&mut self) {
    //     let current_buffer = match self.swapchain.get_next_image_index(&self.present_semaphore) {
    //         NextImageResult::Next(c) => c,
    //         NextImageResult::NeedsRefresh => {
    //             vx_log_f!("Problem with rereshing screen, engine needs refreshing.");
    //         }
    //     } as usize;
    //     self.wait_fences[current_buffer].wait();
    //     self.wait_fences[current_buffer].reset();
    //     self.current_frame_number = current_buffer as u32;

    //     self.clear_copy_data();

    //     self.secondary_data_preparing();
    // }

    // fn clear_copy_data(&self) {
    //     let mut pcmd =
    //         vx_result!(self.data_primary_cmds[self.current_frame_number as usize].lock());
    //     pcmd.begin();
    //     vx_result!(self.buffer_manager.write())
    //         .update(&mut *pcmd, self.current_frame_number as usize);
    //     self.clear_framebuffers[self.current_frame_number as usize].begin(&mut *pcmd);
    //     pcmd.end_render_pass();
    //     pcmd.end();
    //     self.submit(&self.present_semaphore, &pcmd, &self.data_semaphore);
    // }

    // fn secondary_data_preparing(&self) {
    //     let mut pcmd =
    //         vx_result!(self.second_data_primary_cmds[self.current_frame_number as usize].lock());
    //     pcmd.begin();
    //     vx_result!(self.buffer_manager.write())
    //         .secondary_update(&mut *pcmd, self.current_frame_number as usize);
    //     pcmd.end();
    //     self.submit(&self.data_semaphore, &pcmd, &self.second_data_semaphore);
    // }

    // pub(crate) fn submit(&self, wait: &Semaphore, cmd: &CmdBuffer, signal: &Semaphore) {
    //     self.submit_with_fence(
    //         &[*wait.get_data()],
    //         &[*cmd.get_data()],
    //         &[*signal.get_data()],
    //         None,
    //     );
    // }

    // pub(crate) fn submit_with_fence(
    //     &self,
    //     waits: &[vk::Semaphore],
    //     cmds: &[vk::CommandBuffer],
    //     signals: &[vk::Semaphore],
    //     fence: Option<&Fence>,
    // ) {
    //     let wait_stage_mask = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
    //     let mut submit_info = vk::SubmitInfo::default();
    //     submit_info.p_wait_dst_stage_mask = &wait_stage_mask;
    //     submit_info.p_wait_semaphores = waits.as_ptr();
    //     submit_info.wait_semaphore_count = waits.len() as u32;
    //     submit_info.p_signal_semaphores = signals.as_ptr();
    //     submit_info.signal_semaphore_count = signals.len() as u32;
    //     submit_info.p_command_buffers = cmds.as_ptr();
    //     submit_info.command_buffer_count = cmds.len() as u32;
    //     let fence = if let Some(fence) = fence {
    //         *fence.get_data()
    //     } else {
    //         vk::Fence::null()
    //     };
    //     let vk_dev = self.logical_device.get_data();
    //     vx_result!(unsafe {
    //         vk_dev.queue_submit(
    //             self.logical_device.get_vk_graphic_queue(),
    //             &[submit_info],
    //             fence,
    //         )
    //     });
    // }

    // pub(crate) fn submit_multiple(
    //     &self,
    //     waits: &[&Semaphore],
    //     cmds: &[&CmdBuffer],
    //     signals: &[&Semaphore],
    // ) {
    //     let mut waits_data = Vec::with_capacity(waits.len());
    //     let mut signals_data = Vec::with_capacity(signals.len());
    //     let mut cmds_data = Vec::with_capacity(cmds.len());
    //     for w in waits {
    //         waits_data.push(*w.get_data());
    //     }
    //     for s in signals {
    //         signals_data.push(*s.get_data());
    //     }
    //     for c in cmds {
    //         cmds_data.push(*c.get_data());
    //     }
    //     self.submit_with_fence(&waits_data, &cmds_data, &signals_data, None);
    // }

    // pub(crate) fn end(&self, wait: &Semaphore) {
    //     self.submit_with_fence(
    //         &[*wait.get_data()],
    //         &[],
    //         &[*self.render_semaphore.get_data()],
    //         Some(&self.wait_fences[self.current_frame_number as usize]),
    //     );

    //     let mut present_info = vk::PresentInfoKHR::default();
    //     present_info.swapchain_count = 1;
    //     present_info.p_swapchains = self.swapchain.get_data();
    //     present_info.p_image_indices = &self.current_frame_number;
    //     present_info.p_wait_semaphores = self.render_semaphore.get_data();
    //     present_info.wait_semaphore_count = 1;
    //     vx_result!(unsafe {
    //         self.swapchain
    //             .get_loader()
    //             .queue_present(self.logical_device.get_vk_graphic_queue(), &present_info)
    //     });
    // }

    // // pub(crate) fn terminate(&mut self) {
    // //     self.logical_device.wait_idle();
    // // }

    // pub(crate) fn create_texture_2d_with_pixels(
    //     &self,
    //     width: u32,
    //     height: u32,
    //     data: &[u8],
    // ) -> Arc<ImageView> {
    //     Arc::new(ImageView::new_texture_2d_with_pixels(
    //         width,
    //         height,
    //         data,
    //         &self.buffer_manager,
    //     ))
    // }

    // pub(crate) fn create_texture_cube_with_pixels(
    //     &self,
    //     width: u32,
    //     height: u32,
    //     data: &[&[u8]; 6],
    // ) -> Arc<ImageView> {
    //     Arc::new(ImageView::new_texture_cube_with_pixels(
    //         width,
    //         height,
    //         data,
    //         &self.buffer_manager,
    //     ))
    // }

    // pub(crate) fn create_command_pool(&self) -> Arc<CmdPool> {
    //     return Arc::new(CmdPool::new(
    //         self.logical_device.clone(),
    //         CmdPoolType::Graphic,
    //         vk::CommandPoolCreateFlags::empty(),
    //     ));
    // }

    // pub(crate) fn create_secondary_command_buffer(&self, cmd_pool: Arc<CmdPool>) -> CmdBuffer {
    //     return CmdBuffer::new_secondary(cmd_pool);
    // }

    // pub(crate) fn create_primary_command_buffer(&self, cmd_pool: Arc<CmdPool>) -> CmdBuffer {
    //     return CmdBuffer::new_primary(cmd_pool);
    // }

    // pub(crate) fn create_primary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
    //     return CmdBuffer::new_primary(self.graphic_cmd_pool.clone());
    // }

    // pub(crate) fn create_secondary_command_buffer_from_main_graphic_pool(&self) -> CmdBuffer {
    //     return CmdBuffer::new_secondary(self.graphic_cmd_pool.clone());
    // }

    // pub(crate) fn create_semaphore(&self) -> Semaphore {
    //     return Semaphore::new(self.logical_device.clone());
    // }

    // pub(crate) fn get_frames_count(&self) -> usize {
    //     return self.framebuffers.len();
    // }

    // pub(crate) fn get_frame_number(&self) -> usize {
    //     return self.current_frame_number as usize;
    // }

    // pub(crate) fn get_current_framebuffer(&self) -> &Arc<Framebuffer> {
    //     return &self.framebuffers[self.current_frame_number as usize];
    // }

    // pub(crate) fn get_starting_semaphore(&self) -> &Arc<Semaphore> {
    //     return &self.second_data_semaphore;
    // }

    // pub(crate) fn get_device(&self) -> &Arc<LogicalDevice> {
    //     return &self.logical_device;
    // }

    // pub(crate) fn get_memory_manager(&self) -> &Arc<RwLock<MemoryManager>> {
    //     return &self.memory_manager;
    // }

    // pub(crate) fn get_buffer_manager(&self) -> &Arc<RwLock<BufferManager>> {
    //     return &self.buffer_manager;
    // }

    // pub(crate) fn get_descriptor_manager(&self) -> &Arc<RwLock<DescriptorManager>> {
    //     return &self.descriptor_manager;
    // }

    // pub(crate) fn get_pipeline_manager(&self) -> &Arc<RwLock<PipelineManager>> {
    //     return &self.pipeline_manager;
    // }

    // pub(crate) fn get_linear_repeat_sampler(&self) -> &Arc<Sampler> {
    //     return &self.linear_repeat_sampler;
    // }

    // pub(crate) fn get_nearest_repeat_sampler(&self) -> &Arc<Sampler> {
    //     return &self.nearest_repeat_sampler;
    // }

    // pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
    //     return &self.render_pass;
    // }

    // pub(crate) fn get_framebuffers(&self) -> &Vec<Arc<Framebuffer>> {
    //     return &self.framebuffers;
    // }

    pub fn update(&mut self) {}
}
