use super::super::core::allocate::Object as CoreAllocObj;
use super::buffer::{Buffer as BufBuffer, StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::device::logical::Logical as LogicalDevice;
use super::pipeline::Pipeline;
use super::synchronizer::fence::Fence;
use super::vulkan as vk;
use std::default::Default;
use std::ptr::null;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Buffer {
    pool: Arc<Pool>,
    vk_data: vk::VkCommandBuffer,
    bound_pipeline_layout: vk::VkPipelineLayout,
    bound_descriptor_sets: [vk::VkDescriptorSet; 3],
    bound_dynamic_buffer_offsets: [u32; 3],
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

const MAX_DESCRIPTOR_SETS_COUNT: usize = 3;
const MAX_DYNAMIC_BUFFER_OFFSETS_COUNT: usize = 3;

impl Buffer {
    pub fn new_primary(pool: Arc<Pool>) -> Self {
        return Self::new(
            pool,
            vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        );
    }

    pub fn new_secondary(pool: Arc<Pool>) -> Self {
        return Self::new(
            pool,
            vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_SECONDARY,
        );
    }

    fn new(pool: Arc<Pool>, level: vk::VkCommandBufferLevel) -> Self {
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
            bound_pipeline_layout: 0 as vk::VkPipelineLayout,
            bound_descriptor_sets: [0 as vk::VkDescriptorSet; MAX_DESCRIPTOR_SETS_COUNT],
            bound_dynamic_buffer_offsets: [0; MAX_DYNAMIC_BUFFER_OFFSETS_COUNT],
        }
    }

    pub fn fill_submit_info(&self, subinfo: &mut vk::VkSubmitInfo) {
        subinfo.pCommandBuffers = &self.vk_data;
        subinfo.commandBufferCount = 1;
    }

    pub fn begin(&mut self) {
        let mut cmd_buf_info = vk::VkCommandBufferBeginInfo::default();
        cmd_buf_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        vulkan_check!(vk::vkBeginCommandBuffer(self.vk_data, &cmd_buf_info));
    }

    pub fn begin_render_pass_with_info(
        &mut self,
        render_pass_begin_info: vk::VkRenderPassBeginInfo,
    ) {
        unsafe {
            vk::vkCmdBeginRenderPass(
                self.vk_data,
                &render_pass_begin_info,
                vk::VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE,
            );
        }
    }
    pub fn set_viewport(&mut self, viewport: &vk::VkViewport) {
        unsafe {
            vk::vkCmdSetViewport(self.vk_data, 0, 1, viewport);
        }
    }
    pub fn set_scissor(&mut self, rec: &vk::VkRect2D) {
        unsafe {
            vk::vkCmdSetScissor(self.vk_data, 0, 1, rec);
        }
    }

    pub fn copy_buffer(
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

    pub fn copy_buffer_to_image(
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

    pub fn reset(&mut self) {
        unsafe {
            vk::vkResetCommandBuffer(self.vk_data, 0);
        }
    }

    pub fn flush(&mut self) {
        let fence = Fence::new(self.pool.logical_device.clone());
        vulkan_check!(vk::vkEndCommandBuffer(self.vk_data));
        let mut submit_info = vk::VkSubmitInfo::default();
        submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit_info.commandBufferCount = 1;
        submit_info.pCommandBuffers = &self.vk_data;
        vulkan_check!(vk::vkQueueSubmit(
            self.pool.logical_device.vk_graphic_queue,
            1,
            &submit_info,
            fence.vk_data,
        ));
        fence.wait();
    }

    pub fn end_render_pass(&mut self) {
        unsafe {
            vk::vkCmdEndRenderPass(self.vk_data);
        }
    }

    pub fn end(&mut self) {
        vulkan_check!(vk::vkEndCommandBuffer(self.vk_data));
    }

    pub fn bind_pipeline(&mut self, p: &Arc<Pipeline>) {
        let info = p.get_info_for_binding();
        self.bound_pipeline_layout = p.get_layout().vk_data;
        unsafe {
            vk::vkCmdBindPipeline(self.vk_data, info.0, info.1);
        }
    }

    pub fn bind_vertex_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let info = buffer.get_info_for_binding();
        unsafe {
            vk::vkCmdBindVertexBuffers(self.vk_data, 0, 1, &info.1, &info.0);
        }
    }

    pub fn bind_index_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let info = buffer.get_info_for_binding();
        unsafe {
            vk::vkCmdBindIndexBuffer(
                self.vk_data,
                info.1,
                info.0,
                vk::VkIndexType::VK_INDEX_TYPE_UINT32,
            );
        }
    }

    pub fn draw_index(&mut self, indices_count: u32) {
        unsafe {
            vk::vkCmdDrawIndexed(self.vk_data, indices_count, 1, 0, 0, 1);
        }
    }

    pub fn draw(&mut self, vertices_count: u32) {
        unsafe {
            vk::vkCmdDraw(self.vk_data, vertices_count, 1, 0, 0);
        }
    }

    pub fn pipeline_image_barrier(
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
            buffer.get_offset() as u32;
    }

    pub(crate) fn bind_gbuff_model_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_MODEL_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[GBUFF_MODEL_DESCRIPTOR_OFFSET] =
            buffer.get_offset() as u32;
    }

    pub(crate) fn bind_gbuff_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[GBUFF_MATERIAL_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[GBUFF_MATERIAL_DESCRIPTOR_OFFSET] =
            buffer.get_offset() as u32;
    }

    pub(crate) fn render_gbuff(
        &mut self,
        vertex_buffer: &StaticBuffer,
        index_buffer: &StaticBuffer,
        indices_count: u32,
    ) {
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

    pub(crate) fn bind_deferred_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[DEFERRED_SCENE_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[DEFERRED_SCENE_DESCRIPTOR_OFFSET] =
            buffer.get_offset() as u32;
    }

    pub(crate) fn bind_deferred_deferred_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        self.bound_descriptor_sets[DEFERRED_DEFERRED_DESCRIPTOR_OFFSET] = descriptor_set.vk_data;
        self.bound_dynamic_buffer_offsets[DEFERRED_DEFERRED_DESCRIPTOR_OFFSET] =
            buffer.get_offset() as u32;
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
