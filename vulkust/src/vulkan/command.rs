use super::super::core::allocate::Object as CoreAllocObj;
use super::buffer::{Buffer as BufBuffer, Static as StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::device::Logical as LogicalDevice;
use super::framebuffer::Framebuffer;
use super::pipeline::Pipeline;
// use super::sync::Fence;
use ash::version::DeviceV1_0;
use ash::vk;
use std::sync::{Arc, RwLock};

pub struct Buffer {
    pool: Arc<Pool>,
    vk_device: ash::Device,
    vk_data: vk::CommandBuffer,
    has_render_record: bool,
    bound_pipeline_layout: vk::PipelineLayout,
    bound_descriptor_sets: [vk::DescriptorSet; 3],
    bound_dynamic_buffer_offsets: [u32; 3],
    #[cfg(debug_mode)]
    is_secondary: bool,
}

const GBUFF_SCENE_DESCRIPTOR_OFFSET: usize = 0;
const GBUFF_MODEL_DESCRIPTOR_OFFSET: usize = 1;
const GBUFF_MATERIAL_DESCRIPTOR_OFFSET: usize = 2;

const GBUFF_DESCRIPTOR_SETS_COUNT: usize = 3;
const GBUFF_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 3;

const UNLIT_MODEL_DESCRIPTOR_OFFSET: usize = 0;
const UNLIT_MATERIAL_DESCRIPTOR_OFFSET: usize = 1;

const UNLIT_DESCRIPTOR_SETS_COUNT: usize = 2;
const UNLIT_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 2;

const DEFERRED_SCENE_DESCRIPTOR_OFFSET: usize = 0;
const DEFERRED_DEFERRED_DESCRIPTOR_OFFSET: usize = 1;

const DEFERRED_DESCRIPTOR_SETS_COUNT: usize = 2;
const DEFERRED_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 2;

const SSAO_SCENE_DESCRIPTOR_OFFSET: usize = 0;
const SSAO_SSAO_DESCRIPTOR_OFFSET: usize = 1;

const SSAO_DESCRIPTOR_SETS_COUNT: usize = 2;
const SSAO_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 2;

const SHADOW_MAPPER_DESCRIPTOR_SETS_COUNT: usize = 2;
const SHADOW_MAPPER_LIGHT_DESCRIPTOR_OFFSET: usize = 0;
const SHADOW_MAPPER_MATERIAL_DESCRIPTOR_OFFSET: usize = 1;

const SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_SETS_COUNT: usize = 1;
const SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_OFFSET: usize = 0;

const MAX_DESCRIPTOR_SETS_COUNT: usize = 3;
const MAX_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 3;

impl Buffer {
    pub(crate) fn new_primary(pool: Arc<Pool>) -> Self {
        return Self::new(pool, false);
    }

    pub(crate) fn new_secondary(pool: Arc<Pool>) -> Self {
        return Self::new(pool, true);
    }

    fn new(pool: Arc<Pool>, is_secondary: bool) -> Self {
        let level = if is_secondary {
            vk::CommandBufferLevel::SECONDARY
        } else {
            vk::CommandBufferLevel::PRIMARY
        };
        let mut cmd_buf_allocate_info = vk::CommandBufferAllocateInfo::default();
        cmd_buf_allocate_info.command_pool = pool.vk_data;
        cmd_buf_allocate_info.level = level;
        cmd_buf_allocate_info.command_buffer_count = 1;
        let vk_device = pool.logical_device.get_data().clone();
        let vk_data =
            vxresult!(unsafe { vk_device.allocate_command_buffers(&cmd_buf_allocate_info) });
        let vk_data = vk_data[0];
        let pool = pool.clone();
        Self {
            pool,
            vk_data,
            vk_device,
            has_render_record: false,
            bound_pipeline_layout: vk::PipelineLayout::null(),
            bound_descriptor_sets: [vk::DescriptorSet::null(); MAX_DESCRIPTOR_SETS_COUNT],
            bound_dynamic_buffer_offsets: [0; MAX_DYNAMIC_BUFFER_OFFSETS_COUNT],
            #[cfg(debug_mode)]
            is_secondary,
        }
    }

    pub(crate) fn get_data(&self) -> &vk::CommandBuffer {
        return &self.vk_data;
    }

    pub(crate) fn get_has_render_record(&self) -> bool {
        return self.has_render_record;
    }

    pub(crate) fn exe_cmds_with_data(&mut self, data: &[vk::CommandBuffer]) {
        if data.len() < 1 {
            return;
        }
        self.has_render_record = true;
        unsafe {
            self.vk_device.cmd_execute_commands(self.vk_data, &data);
        }
    }

    pub(crate) fn exe_cmd(&mut self, other: &Self) {
        self.has_render_record = true;
        let data = [other.vk_data];
        self.exe_cmds_with_data(&data);
    }

    pub(crate) fn begin(&mut self) {
        #[cfg(debug_mode)]
        {
            if self.is_secondary {
                vxunexpected!();
            }
        }
        self.has_render_record = false;
        let cmd_buf_info = vk::CommandBufferBeginInfo::default();
        vxresult!(unsafe {
            self.vk_device
                .begin_command_buffer(self.vk_data, &cmd_buf_info)
        });
    }

    pub(crate) fn begin_secondary(&mut self, framebuffer: &Framebuffer) {
        #[cfg(debug_mode)]
        {
            if !self.is_secondary {
                vxunexpected!();
            }
        }
        self.has_render_record = false;

        let mut inheritance_info = vk::CommandBufferInheritanceInfo::default();
        inheritance_info.framebuffer = *framebuffer.get_data();
        inheritance_info.render_pass = *framebuffer.get_render_pass().get_data();

        let mut cmd_buf_info = vk::CommandBufferBeginInfo::default();
        cmd_buf_info.p_inheritance_info = &inheritance_info;
        cmd_buf_info.flags = vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE;
        vxresult!(unsafe {
            self.vk_device
                .begin_command_buffer(self.vk_data, &cmd_buf_info)
        });
        unsafe {
            self.vk_device
                .cmd_set_viewport(self.vk_data, 0, &[*framebuffer.get_vk_viewport()]);
            self.vk_device
                .cmd_set_scissor(self.vk_data, 0, &[*framebuffer.get_vk_scissor()]);
        }
    }

    pub(crate) fn begin_render_pass_with_info(
        &mut self,
        render_pass_begin_info: &vk::RenderPassBeginInfo,
    ) {
        unsafe {
            self.vk_device.cmd_begin_render_pass(
                self.vk_data,
                render_pass_begin_info,
                vk::SubpassContents::SECONDARY_COMMAND_BUFFERS,
            );
        }
    }

    pub(crate) fn copy_buffer(
        &mut self,
        src: vk::Buffer,
        dst: vk::Buffer,
        regions: &[vk::BufferCopy],
    ) {
        unsafe {
            self.vk_device
                .cmd_copy_buffer(self.vk_data, src, dst, regions);
        }
    }

    pub(crate) fn copy_buffer_to_image(
        &mut self,
        src: vk::Buffer,
        dst: vk::Image,
        region: &vk::BufferImageCopy,
    ) {
        unsafe {
            self.vk_device.cmd_copy_buffer_to_image(
                self.vk_data,
                src,
                dst,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[*region],
            );
        }
    }

    // pub(crate) fn reset(&mut self) {
    //     unsafe {
    //         vk::vkResetCommandBuffer(self.vk_data, 0);
    //     }
    // }

    // pub(crate) fn flush(&mut self) {
    //     let fence = Fence::new(self.pool.logical_device.clone());
    //     vxresult!(unsafe { self.vk_device.EndCommandBuffer(self.vk_data));
    //     let mut submit_info = vk::VkSubmitInfo::default();
    //     submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
    //     submit_info.commandBufferCount = 1;
    //     submit_info.pCommandBuffers = &self.vk_data;
    //     vxresult!(unsafe { self.vk_device.QueueSubmit(
    //         self.pool.logical_device.vk_graphic_queue,
    //         1,
    //         &submit_info,
    //         fence.vk_data,
    //     ));
    //     fence.wait();
    // }

    pub(crate) fn end_render_pass(&mut self) {
        unsafe {
            self.vk_device.cmd_end_render_pass(self.vk_data);
        }
    }

    pub(crate) fn end(&mut self) {
        vxresult!(unsafe { self.vk_device.end_command_buffer(self.vk_data) });
    }

    pub(crate) fn bind_pipeline(&mut self, p: &Pipeline) {
        let info = p.get_info_for_binding();
        self.bound_pipeline_layout = *p.get_layout().get_data();
        unsafe {
            self.vk_device
                .cmd_bind_pipeline(self.vk_data, info.0, info.1);
        }
    }

    pub(crate) fn bind_vertex_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let vkbuff = buffer.get_data();
        let offset = buffer.get_allocated_memory().get_offset() as vk::DeviceSize;
        unsafe {
            self.vk_device
                .cmd_bind_vertex_buffers(self.vk_data, 0, &[vkbuff], &[offset]);
        }
    }

    pub(crate) fn bind_index_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let vkbuff = buffer.get_data();
        let offset = buffer.get_allocated_memory().get_offset() as vk::DeviceSize;
        unsafe {
            self.vk_device.cmd_bind_index_buffer(
                self.vk_data,
                vkbuff,
                offset,
                vk::IndexType::UINT32,
            );
        }
    }

    pub(crate) fn draw_index(&mut self, indices_count: u32) {
        unsafe {
            self.vk_device
                .cmd_draw_indexed(self.vk_data, indices_count, 1, 0, 0, 1);
        }
    }

    pub(crate) fn draw(&mut self, vertices_count: u32) {
        unsafe {
            self.vk_device
                .cmd_draw(self.vk_data, vertices_count, 1, 0, 0);
        }
    }

    pub(crate) fn pipeline_image_barrier(
        &mut self,
        src_stage: vk::PipelineStageFlags,
        dst_stage: vk::PipelineStageFlags,
        dependancy: vk::DependencyFlags,
        info: &vk::ImageMemoryBarrier,
    ) {
        unsafe {
            self.vk_device.cmd_pipeline_barrier(
                self.vk_data,
                src_stage,
                dst_stage,
                dependancy,
                &[],
                &[],
                &[*info],
            );
        }
    }

    pub(crate) fn bind_gbuff_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_SCENE_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[GBUFF_SCENE_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_gbuff_model_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_MODEL_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[GBUFF_MODEL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_gbuff_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_MATERIAL_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[GBUFF_MATERIAL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn render_gbuff(
        &mut self,
        vertex_buffer: &StaticBuffer,
        index_buffer: &StaticBuffer,
        indices_count: u32,
    ) {
        self.has_render_record = true;
        unsafe {
            self.vk_device.cmd_bind_descriptor_sets(
                self.vk_data,
                vk::PipelineBindPoint::GRAPHICS,
                self.bound_pipeline_layout,
                0,
                &self.bound_descriptor_sets[..GBUFF_DESCRIPTOR_SETS_COUNT],
                &self.bound_dynamic_buffer_offsets[..GBUFF_DYNAMIC_BUFFER_OFFSETS_COUNT],
            );
        }
        self.bind_vertex_buffer(vertex_buffer.get_buffer());
        self.bind_index_buffer(index_buffer.get_buffer());
        self.draw_index(indices_count);
    }

    pub(crate) fn bind_unlit_model_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[UNLIT_MODEL_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[UNLIT_MODEL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_unlit_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[UNLIT_MATERIAL_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[UNLIT_MATERIAL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn render_unlit(
        &mut self,
        vertex_buffer: &StaticBuffer,
        index_buffer: &StaticBuffer,
        indices_count: u32,
    ) {
        self.has_render_record = true;
        unsafe {
            self.vk_device.cmd_bind_descriptor_sets(
                self.vk_data,
                vk::PipelineBindPoint::GRAPHICS,
                self.bound_pipeline_layout,
                0,
                &self.bound_descriptor_sets[..UNLIT_DESCRIPTOR_SETS_COUNT],
                &self.bound_dynamic_buffer_offsets[..UNLIT_DYNAMIC_BUFFER_OFFSETS_COUNT],
            );
        }
        self.bind_vertex_buffer(vertex_buffer.get_buffer());
        self.bind_index_buffer(index_buffer.get_buffer());
        self.draw_index(indices_count);
    }

    pub(crate) fn render_deferred(&mut self) {
        self.has_render_record = true;
        unsafe {
            self.vk_device.cmd_bind_descriptor_sets(
                self.vk_data,
                vk::PipelineBindPoint::GRAPHICS,
                self.bound_pipeline_layout,
                0,
                &self.bound_descriptor_sets[..DEFERRED_DESCRIPTOR_SETS_COUNT],
                &self.bound_dynamic_buffer_offsets[..DEFERRED_DYNAMIC_BUFFER_OFFSETS_COUNT],
            );
        }
        self.draw(3);
    }

    pub(crate) fn render_ssao(&mut self) {
        self.has_render_record = true;
        unsafe {
            self.vk_device.cmd_bind_descriptor_sets(
                self.vk_data,
                vk::PipelineBindPoint::GRAPHICS,
                self.bound_pipeline_layout,
                0,
                &self.bound_descriptor_sets[..SSAO_DESCRIPTOR_SETS_COUNT],
                &self.bound_dynamic_buffer_offsets[..SSAO_DYNAMIC_BUFFER_OFFSETS_COUNT],
            );
        }
        self.draw(3);
    }

    pub(crate) fn render_shadow_accumulator_directional(&mut self) {
        self.has_render_record = true;
        unsafe {
            self.vk_device.cmd_bind_descriptor_sets(
                self.vk_data,
                vk::PipelineBindPoint::GRAPHICS,
                self.bound_pipeline_layout,
                0,
                &self.bound_descriptor_sets[..SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_SETS_COUNT],
                &self.bound_dynamic_buffer_offsets
                    [..SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_SETS_COUNT],
            );
        }
        self.draw(3);
    }

    pub(crate) fn bind_deferred_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[DEFERRED_SCENE_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[DEFERRED_SCENE_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_deferred_deferred_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[DEFERRED_DEFERRED_DESCRIPTOR_OFFSET] =
            *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[DEFERRED_DEFERRED_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_ssao_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SSAO_SCENE_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[SSAO_SCENE_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_ssao_ssao_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SSAO_SSAO_DESCRIPTOR_OFFSET] = *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[SSAO_SSAO_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_shadow_mapper_light_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SHADOW_MAPPER_LIGHT_DESCRIPTOR_OFFSET] =
            *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[SHADOW_MAPPER_LIGHT_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_shadow_mapper_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SHADOW_MAPPER_MATERIAL_DESCRIPTOR_OFFSET] =
            *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[SHADOW_MAPPER_MATERIAL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_shadow_accumulator_directional_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_OFFSET] =
            *descriptor_set.get_data();
        self.bound_dynamic_buffer_offsets[SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn render_shadow_mapper(
        &mut self,
        vertex_buffer: &StaticBuffer,
        index_buffer: &StaticBuffer,
        indices_count: u32,
    ) {
        self.has_render_record = true;
        unsafe {
            self.vk_device.cmd_bind_descriptor_sets(
                self.vk_data,
                vk::PipelineBindPoint::GRAPHICS,
                self.bound_pipeline_layout,
                0,
                &self.bound_descriptor_sets[..SHADOW_MAPPER_DESCRIPTOR_SETS_COUNT],
                &self.bound_dynamic_buffer_offsets[..SHADOW_MAPPER_DESCRIPTOR_SETS_COUNT],
            );
        }
        self.bind_vertex_buffer(vertex_buffer.get_buffer());
        self.bind_index_buffer(index_buffer.get_buffer());
        self.draw_index(indices_count);
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.vk_device
                .free_command_buffers(self.pool.vk_data, &[self.vk_data]);
        }
    }
}

unsafe impl Send for Buffer {}

unsafe impl Sync for Buffer {}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum Type {
    Graphic,
    Compute,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Pool {
    pool_type: Type,
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::CommandPool,
}

impl Pool {
    pub(super) fn new(
        logical_device: Arc<LogicalDevice>,
        pool_type: Type,
        flags: vk::CommandPoolCreateFlags,
    ) -> Self {
        let mut vk_cmd_pool_info = vk::CommandPoolCreateInfo::default();
        match pool_type {
            Type::Graphic => {
                vk_cmd_pool_info.queue_family_index = logical_device
                    .get_physical()
                    .get_graphics_queue_node_index();
            }
            Type::Compute => {
                vk_cmd_pool_info.queue_family_index =
                    logical_device.get_physical().get_compute_queue_node_index();
            }
        }
        vk_cmd_pool_info.flags = vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER | flags;
        let vk_data = vxresult!(unsafe {
            logical_device
                .get_data()
                .create_command_pool(&vk_cmd_pool_info, None)
        });
        Self {
            pool_type,
            logical_device,
            vk_data,
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .get_data()
                .destroy_command_pool(self.vk_data, None);
        }
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vulkan CommandBuffer")
    }
}

unsafe impl Send for Pool {}

unsafe impl Sync for Pool {}
