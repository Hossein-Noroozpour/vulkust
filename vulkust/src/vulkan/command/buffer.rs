use super::super::vulkan as vk;
use std::sync::{Arc, RwLock};
use super::super::buffer::Buffer as BufBuffer;
use super::super::descriptor::Set as DescriptorSet;
use super::super::pipeline::{Pipeline, Layout as PipelineLayout};
use super::super::synchronizer::fence::Fence;
use super::pool::Pool;

pub struct Buffer {
    pub pool: Arc<Pool>,
    pub vk_data: vk::VkCommandBuffer,
}

impl Buffer {
    pub fn new(pool: Arc<Pool>) -> Self {
        let mut cmd_buf_allocate_info = vk::VkCommandBufferAllocateInfo::default();
        cmd_buf_allocate_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
        cmd_buf_allocate_info.commandPool = pool.vk_data;
        cmd_buf_allocate_info.level = vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY;
        cmd_buf_allocate_info.commandBufferCount = 1;
        let mut vk_data = 0 as vk::VkCommandBuffer;
        vulkan_check!(vk::vkAllocateCommandBuffers(
            pool.logical_device.vk_data,
            &cmd_buf_allocate_info,
            &mut vk_data,
        ));
        let mut cmd_buf_info = vk::VkCommandBufferBeginInfo::default();
        cmd_buf_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        vulkan_check!(vk::vkBeginCommandBuffer(vk_data, &cmd_buf_info));
        Buffer {
            pool: pool.clone(),
            vk_data: vk_data,
        }
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
    pub fn set_viewport(&mut self, viewport: vk::VkViewport) {
        unsafe {
            vk::vkCmdSetViewport(self.vk_data, 0, 1, &viewport);
        }
    }
    pub fn set_scissor(&mut self, rec: vk::VkRect2D) {
        unsafe {
            vk::vkCmdSetScissor(self.vk_data, 0, 1, &rec);
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

    pub fn reset(&mut self) {
        unsafe { vk::vkResetCommandBuffer(self.vk_data, 0); }
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

    pub fn bind_descriptor_set(
        &mut self, 
        pl: &PipelineLayout, 
        ds: &Arc<DescriptorSet>, 
        offset: usize
    ) {
        let offset = offset as u32;
        let bind_point = vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS;
        unsafe {
            vk::vkCmdBindDescriptorSets(
                self.vk_data, bind_point, pl.vk_data, 0, 1, &ds.vk_data, 1, &offset);
        }
    }

    pub fn bind_pipeline(&mut self, p: &Arc<Pipeline>) {
        let bind_point = vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS;
        unsafe {
            vk::vkCmdBindPipeline(
                self.vk_data, bind_point, p.vk_data,
            );
        }
    }

    pub fn bind_vertex_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let offset = buffer.info.offset as vk::VkDeviceSize;
        unsafe {
            vk::vkCmdBindVertexBuffers(
                self.vk_data,
                0, 1,
                &buffer.vk_data,
                &offset,
            );
        }

    }

    pub fn bind_index_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        let buffer = vxresult!(buffer.read());
        let offset = buffer.info.offset as vk::VkDeviceSize;
        unsafe {
            vk::vkCmdBindIndexBuffer(
                self.vk_data,
                buffer.vk_data,
                offset,
                vk::VkIndexType::VK_INDEX_TYPE_UINT32,
            );
        }
    }

    pub fn draw_index(&mut self, indices_count: u32) {
        unsafe {
            vk::vkCmdDrawIndexed(self.vk_data, indices_count, 1, 0, 0, 1);
        }
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
