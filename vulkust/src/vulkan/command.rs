use super::super::core::allocate::Object as CoreAllocObj;
use super::buffer::{Buffer as BufBuffer, StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::device::logical::Logical as LogicalDevice;
use super::framebuffer::Framebuffer;
use super::pipeline::Pipeline;
// use super::sync::Fence;
use super::vulkan as vk;
use std::default::Default;
use std::ptr::null;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Buffer {
    pool: Arc<Pool>,
    vk_data: vk::VkCommandBuffer,
    has_render_record: bool,
    bound_pipeline_layout: vk::VkPipelineLayout,
    bound_descriptor_sets: [vk::VkDescriptorSet; 3],
    bound_dynamic_buffer_offsets: [u32; 3],
    #[cfg(debug_mode)]
    is_secondary: bool,
}

const GBUFF_SCENE_DESCRIPTOR_OFFSET: usize = 0;
const GBUFF_MODEL_DESCRIPTOR_OFFSET: usize = 1;
const GBUFF_MATERIAL_DESCRIPTOR_OFFSET: usize = 2;

const GBUFF_DESCRIPTOR_SETS_COUNT: usize = 3;
const GBUFF_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 3;

const DEFERRED_SCENE_DESCRIPTOR_OFFSET: usize = 0;
const DEFERRED_DEFERRED_DESCRIPTOR_OFFSET: usize = 1;

const DEFERRED_DESCRIPTOR_SETS_COUNT: usize = 2;
const DEFERRED_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 2;

const RESOLVER_DESCRIPTOR_SETS_COUNT: usize = 1;
const RESOLVER_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 1;
const RESOLVER_DESCRIPTOR_OFFSET: usize = 0;

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
            vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_SECONDARY
        } else {
            vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY
        };
        let mut cmd_buf_allocate_info = vk::VkCommandBufferAllocateInfo::default();
        cmd_buf_allocate_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
        cmd_buf_allocate_info.commandPool = pool.vk_data;
        cmd_buf_allocate_info.level = level;
        cmd_buf_allocate_info.commandBufferCount = 1;
        let mut vk_data = 0 as vk::VkCommandBuffer;
        vulkan_check!(vk::vkAllocateCommandBuffers(
            pool.logical_device.vk_data,
            &cmd_buf_allocate_info,
            &mut vk_data,
        ));
        Buffer {
            pool: pool.clone(),
            vk_data: vk_data,
            has_render_record: false,
            bound_pipeline_layout: 0 as vk::VkPipelineLayout,
            bound_descriptor_sets: [0 as vk::VkDescriptorSet; MAX_DESCRIPTOR_SETS_COUNT],
            bound_dynamic_buffer_offsets: [0; MAX_DYNAMIC_BUFFER_OFFSETS_COUNT],
            #[cfg(debug_mode)]
            is_secondary,
        }
    }

    pub(crate) fn get_data(&self) -> vk::VkCommandBuffer {
        return self.vk_data;
    }

    pub(crate) fn get_has_render_record(&self) -> bool {
        return self.has_render_record;
    }

    pub(crate) fn exe_cmds_with_data(&mut self, data: &[vk::VkCommandBuffer]) {
        if data.len() < 1 {
            return;
        }
        self.has_render_record = true;
        unsafe {
            vk::vkCmdExecuteCommands(self.vk_data, data.len() as u32, data.as_ptr());
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
        let mut cmd_buf_info = vk::VkCommandBufferBeginInfo::default();
        cmd_buf_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        vulkan_check!(vk::vkBeginCommandBuffer(self.vk_data, &cmd_buf_info));
    }

    pub(crate) fn begin_secondary(&mut self, framebuffer: &Framebuffer) {
        #[cfg(debug_mode)]
        {
            if !self.is_secondary {
                vxunexpected!();
            }
        }
        self.has_render_record = false;

        let mut inheritance_info = vk::VkCommandBufferInheritanceInfo::default();
        inheritance_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO;
        inheritance_info.framebuffer = framebuffer.get_data();
        inheritance_info.renderPass = framebuffer.get_render_pass().get_data();

        let mut cmd_buf_info = vk::VkCommandBufferBeginInfo::default();
        cmd_buf_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        cmd_buf_info.pInheritanceInfo = &inheritance_info;
        cmd_buf_info.flags =
            vk::VkCommandBufferUsageFlagBits::VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT
                as vk::VkCommandBufferUsageFlags;
        vulkan_check!(vk::vkBeginCommandBuffer(self.vk_data, &cmd_buf_info));
        unsafe {
            vk::vkCmdSetViewport(self.vk_data, 0, 1, framebuffer.get_vk_viewport());
            vk::vkCmdSetScissor(self.vk_data, 0, 1, framebuffer.get_vk_scissor());
        }
    }

    pub(crate) fn begin_render_pass_with_info(
        &mut self,
        render_pass_begin_info: vk::VkRenderPassBeginInfo,
    ) {
        unsafe {
            vk::vkCmdBeginRenderPass(
                self.vk_data,
                &render_pass_begin_info,
                vk::VkSubpassContents::VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
            );
        }
    }

    pub(crate) fn copy_buffer(
        &mut self,
        src: vk::VkBuffer,
        dst: vk::VkBuffer,
        regions: &Vec<vk::VkBufferCopy>,
    ) {
        unsafe {
            vk::vkCmdCopyBuffer(
                self.vk_data,
                src,
                dst,
                regions.len() as u32,
                regions.as_ptr(),
            );
        }
    }

    pub(crate) fn copy_buffer_to_image(
        &mut self,
        src: vk::VkBuffer,
        dst: vk::VkImage,
        region: &vk::VkBufferImageCopy,
    ) {
        unsafe {
            vk::vkCmdCopyBufferToImage(
                self.vk_data,
                src,
                dst,
                vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                1,
                region,
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
    //     vulkan_check!(vk::vkEndCommandBuffer(self.vk_data));
    //     let mut submit_info = vk::VkSubmitInfo::default();
    //     submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
    //     submit_info.commandBufferCount = 1;
    //     submit_info.pCommandBuffers = &self.vk_data;
    //     vulkan_check!(vk::vkQueueSubmit(
    //         self.pool.logical_device.vk_graphic_queue,
    //         1,
    //         &submit_info,
    //         fence.vk_data,
    //     ));
    //     fence.wait();
    // }

    pub(crate) fn end_render_pass(&mut self) {
        unsafe {
            vk::vkCmdEndRenderPass(self.vk_data);
        }
    }

    pub(crate) fn end(&mut self) {
        vulkan_check!(vk::vkEndCommandBuffer(self.vk_data));
    }

    pub(crate) fn bind_pipeline(&mut self, p: &Pipeline) {
        let info = p.get_info_for_binding();
        self.bound_pipeline_layout = p.get_layout().vk_data;
        unsafe {
            vk::vkCmdBindPipeline(self.vk_data, info.0, info.1);
        }
    }

    pub(crate) fn bind_vertex_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let vkbuff = buffer.get_data();
        let offset = buffer.get_allocated_memory().get_offset() as vk::VkDeviceSize;
        unsafe {
            vk::vkCmdBindVertexBuffers(self.vk_data, 0, 1, &vkbuff, &offset);
        }
    }

    pub(crate) fn bind_index_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let vkbuff = buffer.get_data();
        let offset = buffer.get_allocated_memory().get_offset() as vk::VkDeviceSize;
        unsafe {
            vk::vkCmdBindIndexBuffer(
                self.vk_data,
                vkbuff,
                offset,
                vk::VkIndexType::VK_INDEX_TYPE_UINT32,
            );
        }
    }

    pub(crate) fn draw_index(&mut self, indices_count: u32) {
        unsafe {
            vk::vkCmdDrawIndexed(self.vk_data, indices_count, 1, 0, 0, 1);
        }
    }

    pub(crate) fn draw(&mut self, vertices_count: u32) {
        unsafe {
            vk::vkCmdDraw(self.vk_data, vertices_count, 1, 0, 0);
        }
    }

    pub(crate) fn pipeline_image_barrier(
        &mut self,
        src_stage: vk::VkPipelineStageFlags,
        dst_stage: vk::VkPipelineStageFlags,
        dependancy: vk::VkDependencyFlags,
        info: &vk::VkImageMemoryBarrier,
    ) {
        unsafe {
            vk::vkCmdPipelineBarrier(
                self.vk_data,
                src_stage,
                dst_stage,
                dependancy,
                0,
                null(),
                0,
                null(),
                1,
                info,
            );
        }
    }

    pub(crate) fn bind_gbuff_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_SCENE_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[GBUFF_SCENE_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_gbuff_model_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_MODEL_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[GBUFF_MODEL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_gbuff_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_MATERIAL_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
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
            vk::vkCmdBindDescriptorSets(
                self.vk_data,
                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                self.bound_pipeline_layout,
                0,
                GBUFF_DESCRIPTOR_SETS_COUNT as u32,
                self.bound_descriptor_sets.as_ptr(),
                GBUFF_DYNAMIC_BUFFER_OFFSETS_COUNT as u32,
                self.bound_dynamic_buffer_offsets.as_ptr(),
            );
        }
        self.bind_vertex_buffer(vertex_buffer.get_buffer());
        self.bind_index_buffer(index_buffer.get_buffer());
        self.draw_index(indices_count);
    }

    pub(crate) fn render_resolver(&mut self) {
        self.has_render_record = true;
        unsafe {
            vk::vkCmdBindDescriptorSets(
                self.vk_data,
                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                self.bound_pipeline_layout,
                0,
                RESOLVER_DESCRIPTOR_SETS_COUNT as u32,
                self.bound_descriptor_sets.as_ptr(),
                RESOLVER_DYNAMIC_BUFFER_OFFSETS_COUNT as u32,
                self.bound_dynamic_buffer_offsets.as_ptr(),
            );
        }
        self.draw(3);
    }

    pub(crate) fn render_deferred(&mut self) {
        self.has_render_record = true;
        unsafe {
            vk::vkCmdBindDescriptorSets(
                self.vk_data,
                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                self.bound_pipeline_layout,
                0,
                DEFERRED_DESCRIPTOR_SETS_COUNT as u32,
                self.bound_descriptor_sets.as_ptr(),
                DEFERRED_DYNAMIC_BUFFER_OFFSETS_COUNT as u32,
                self.bound_dynamic_buffer_offsets.as_ptr(),
            );
        }
        self.draw(3);
    }

    pub(crate) fn render_shadow_accumulator_directional(&mut self) {
        self.has_render_record = true;
        unsafe {
            vk::vkCmdBindDescriptorSets(
                self.vk_data,
                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                self.bound_pipeline_layout,
                0,
                SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_SETS_COUNT as u32,
                self.bound_descriptor_sets.as_ptr(),
                SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_SETS_COUNT as u32,
                self.bound_dynamic_buffer_offsets.as_ptr(),
            );
        }
        self.draw(3);
    }

    pub(crate) fn bind_deferred_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[DEFERRED_SCENE_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[DEFERRED_SCENE_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_resolver_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[RESOLVER_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[RESOLVER_DESCRIPTOR_OFFSET] = 
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_deferred_deferred_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[DEFERRED_DEFERRED_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[DEFERRED_DEFERRED_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_shadow_mapper_light_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SHADOW_MAPPER_LIGHT_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[SHADOW_MAPPER_LIGHT_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_shadow_mapper_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SHADOW_MAPPER_MATERIAL_DESCRIPTOR_OFFSET] =
            descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[SHADOW_MAPPER_MATERIAL_DESCRIPTOR_OFFSET] =
            buffer.get_allocated_memory().get_offset() as u32;
    }

    pub(crate) fn bind_shadow_accumulator_directional_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[SHADOW_ACCUMULATOR_DIRECTIONAL_DESCRIPTOR_OFFSET] =
            descriptor_set.vk_data;
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
            vk::vkCmdBindDescriptorSets(
                self.vk_data,
                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                self.bound_pipeline_layout,
                0,
                SHADOW_MAPPER_DESCRIPTOR_SETS_COUNT as u32,
                self.bound_descriptor_sets.as_ptr(),
                SHADOW_MAPPER_DESCRIPTOR_SETS_COUNT as u32,
                self.bound_dynamic_buffer_offsets.as_ptr(),
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
            vk::vkFreeCommandBuffers(
                self.pool.logical_device.vk_data,
                self.pool.vk_data,
                1,
                &mut self.vk_data,
            );
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
    pub pool_type: Type,
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkCommandPool,
}

impl Pool {
    pub fn new(logical_device: Arc<LogicalDevice>, pool_type: Type, flags: u32) -> Self {
        let mut vk_cmd_pool_info = vk::VkCommandPoolCreateInfo::default();
        vk_cmd_pool_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
        match pool_type {
            Type::Graphic => {
                vk_cmd_pool_info.queueFamilyIndex =
                    logical_device.physical_device.graphics_queue_node_index;
            }
            Type::Compute => {
                vk_cmd_pool_info.queueFamilyIndex =
                    logical_device.physical_device.compute_queue_node_index;
            }
        }
        vk_cmd_pool_info.flags =
            vk::VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32
                | flags;
        let mut vk_data = 0 as vk::VkCommandPool;
        vulkan_check!(vk::vkCreateCommandPool(
            logical_device.vk_data,
            &vk_cmd_pool_info,
            null(),
            &mut vk_data,
        ));
        Pool {
            pool_type,
            logical_device,
            vk_data: vk_data,
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyCommandPool(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

unsafe impl Send for Pool {}

unsafe impl Sync for Pool {}
