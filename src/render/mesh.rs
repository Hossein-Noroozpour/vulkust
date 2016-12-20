use super::super::vulkan::device::Device;
use super::super::vulkan::command::buffer::Buffer as CmdBuff;
use super::super::vulkan::Driver;
use super::super::vulkan::buffer::Buffer;

use std::sync::{
    Arc,
};

pub struct Mesh {

}

impl Mesh {
    pub fn new(driver: Arc<Driver>) {
        let vertex_buffer = vec![
            1.0f32, 1.0f32, 0.0f32,   1.0f32,  0.0f32, 0.0f32,   -1.0f32, 1.0f32, 0.0f32,
            0.0f32, 1.0f32, 0.0f32,   0.0f32, -1.0f32, 0.0f32,    0.0f32, 0.0f32, 1.0f32,
        ];
        let vertex_buffer_size = 72u32;
        let index_buffer = [0u32, 1u32, 2u32];
        let indices_count = 3u32;
        let index_buffer_size = 12u32;
        let mut vertex_buffer = Buffer::new_in_vram(driver.device, vertex_buffer);
//        stgvb.
//
//
//
//
//
//        let mut index_buffer_info = VkBufferCreateInfo::default();
//        index_buffer_info.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
//        index_buffer_info.size = index_buffer_size;
//        index_buffer_info.usage = VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
//        vulkan_check!(vkCreateBuffer(
//            dev.vk_device, &index_buffer_info, 0 as *const VkAllocationCallback, &staging_buffers[1].buffer));
//        vulkan_check!(vkGetBufferMemoryRequirements(dev.vk_device, staging_buffers[1].buffer, &mem_rs));
//        mem_alloc.allocationSize = mem_rs.size;
//        mem_alloc.memoryTypeIndex = getMemoryTypeIndex(memReqs.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
//        vulkan_check!(vkAllocateMemory(device, &memAlloc, nullptr, &staging_buffers.indices.memory));
//        vulkan_check!(vkMapMemory(device, staging_buffers.indices.memory, 0, indexBufferSize, 0, &data));
//        memcpy(data, indexBuffer.data(), indexBufferSize);
//        vkUnmapMemory(device, staging_buffers.indices.memory);
//        vulkan_check!(vkBindBufferMemory(device, staging_buffers.indices.buffer, staging_buffers.indices.memory, 0));
//        index_buffer_info.usage = VK_BUFFER_USAGE_INDEX_BUFFER_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT;
//        vulkan_check!(vkCreateBuffer(device, &indexbufferInfo, nullptr, &indices.buffer));
//        vkGetBufferMemoryRequirements(device, indices.buffer, &memReqs);
//        memAlloc.allocationSize = memReqs.size;
//        memAlloc.memoryTypeIndex = getMemoryTypeIndex(memReqs.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
//        vulkan_check!(vkAllocateMemory(device, &memAlloc, nullptr, &indices.memory));
//        vulkan_check!(vkBindBufferMemory(device, indices.buffer, indices.memory, 0));
//
//        let mut cmd_buffer_begin_info = VkCommandBufferBeginInfo::default();
//        cmd_buffer_begin_info.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
//        let copy_cmd = CmdBuff::new(drv.cmd_pool);
//        let mut copy_region = VkBufferCopy::default();
//        copy_region.size = vertex_buffer_size;
//        vulkan_check!(vkCmdCopyBuffer(copy_cmd, staging_buffers[0].buffer, vertices.buffer, 1, &copy_region));
//        copy_region.size = index_buffer_size;
//        vulkan_check!(vkCmdCopyBuffer(copy_cmd, staging_buffers[1].buffer, indices.buffer, 1, &copy_region));
//        copy_cmd.flush();
//        vkDestroyBuffer(device, staging_buffers.vertices.buffer, nullptr);
//        vkFreeMemory(device, staging_buffers.vertices.memory, nullptr);
//        vkDestroyBuffer(device, staging_buffers.indices.buffer, nullptr);
//        vkFreeMemory(device, staging_buffers.indices.memory, nullptr);
//
//        // Vertex input binding
//        vertices.inputBinding.binding = VERTEX_BUFFER_BIND_ID;
//        vertices.inputBinding.stride = sizeof(Vertex);
//        vertices.inputBinding.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;
//
//        // Inpute attribute binding describe shader attribute locations and memory layouts
//        // These match the following shader layout (see triangle.vert):
//        //	layout (location = 0) in vec3 inPos;
//        //	layout (location = 1) in vec3 inColor;
//        vertices.inputAttributes.resize(2);
//        // Attribute location 0: Position
//        vertices.inputAttributes[0].binding = VERTEX_BUFFER_BIND_ID;
//        vertices.inputAttributes[0].location = 0;
//        vertices.inputAttributes[0].format = VK_FORMAT_R32G32B32_SFLOAT;
//        vertices.inputAttributes[0].offset = offsetof(Vertex, position);
//        // Attribute location 1: Color
//        vertices.inputAttributes[1].binding = VERTEX_BUFFER_BIND_ID;
//        vertices.inputAttributes[1].location = 1;
//        vertices.inputAttributes[1].format = VK_FORMAT_R32G32B32_SFLOAT;
//        vertices.inputAttributes[1].offset = offsetof(Vertex, color);
//
//        // Assign to the vertex input state used for pipeline creation
//        vertices.inputState.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
//        vertices.inputState.pNext = nullptr;
//        vertices.inputState.flags = VK_FLAGS_NONE;
//        vertices.inputState.vertexBindingDescriptionCount = 1;
//        vertices.inputState.pVertexBindingDescriptions = &vertices.inputBinding;
//        vertices.inputState.vertexAttributeDescriptionCount = static_cast<uint32_t>(vertices.inputAttributes.size());
//        vertices.inputState.pVertexAttributeDescriptions = vertices.inputAttributes.data();
    }
}