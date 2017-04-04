use super::super::system::vulkan as vk;
use super::device::logical::Logical as LogicalDevice;
use std::default::Default;
use std::sync::Arc;
use std::ptr::{
    copy,
    null,
};
use std::os::raw::c_void;
use std::mem::transmute;

pub struct Mesh {
    pub vertices_buffer: vk::VkBuffer,
    pub vertices_memory: vk::VkDeviceMemory,
}

impl Mesh {
    pub fn new(
        logical_device: Arc<LogicalDevice>,
        vertex_buffer: *const c_void, vertex_buffer_size: u32,
        index_buffer: *const u32, index_buffer_size: u32) {
		let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
		mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
		let mut mem_reqs = vk::VkMemoryRequirements::default();
		let mut data = 0 as *mut c_void;
        let mut staging_buffers_vertices_memory = 0 as vk::VkDeviceMemory;
        let mut staging_buffers_vertices_buffer = 0 as vk::VkBuffer;
        let mut staging_buffers_indices_memory = 0 as vk::VkDeviceMemory;
        let mut staging_buffers_indices_buffer = 0 as vk::VkBuffer;
    	let mut vertex_buffer_info = vk::VkBufferCreateInfo::default();
    	vertex_buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    	vertex_buffer_info.size = vertex_buffer_size as vk::VkDeviceSize;
    	vertex_buffer_info.usage =
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32;
    	vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data, &vertex_buffer_info, null(),
            &mut staging_buffers_vertices_buffer));
    	vk::vkGetBufferMemoryRequirements(
            logical_device.vk_data, staging_buffers_vertices_buffer, &mut mem_reqs);
    	mem_alloc.allocationSize = mem_reqs.size;
    	mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32);
    	vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data, &mem_alloc, null(), &mut staging_buffers_vertices_memory));
    	vulkan_check!(vk::vkMapMemory(
            logical_device.vk_data, staging_buffers_vertices_memory, 0,
            mem_alloc.allocationSize, 0, &mut data));
    	unsafe {
            copy(vertex_buffer, data, vertex_buffer_size as usize);
        }
    	vk::vkUnmapMemory(logical_device.vk_data, staging_buffers_vertices_memory);
    	vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data, staging_buffers_vertices_buffer,
            staging_buffers_vertices_memory, 0));
    	vertex_buffer_info.usage =
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
        let mut vertices_buffer = 0 as vk::VkBuffer;
    	vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data, &vertex_buffer_info, null(), &mut vertices_buffer));
    	unsafe {
            vk::vkGetBufferMemoryRequirements(
                logical_device.vk_data, vertices_buffer, &mut mem_reqs);
        }
    	mem_alloc.allocationSize = mem_reqs.size;
    	mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32);
        let mut vertices_memory = 0 as vk::VkDeviceMemory;
    	vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data, &mem_alloc, null(), &mut vertices_memory));
    	vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data, vertices_buffer, vertices_memory, 0));

    	// Index buffer
    	let mut index_buffer_info = vk::VkBufferCreateInfo::default();
    	index_buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    	index_buffer_info.size = index_buffer_size as vk::VkDeviceSize;
    	index_buffer_info.usage =
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32;
    	vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data, &index_buffer_info,
            null(), &mut staging_buffers_indices_buffer));
    	vk::vkGetBufferMemoryRequirements(
            logical_device.vk_data, staging_buffers_indices_buffer, &mut mem_reqs);
    	mem_alloc.allocationSize = mem_reqs.size;
    	mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32);
    	vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data, &mem_alloc, null(), &mut staging_buffers_indices_memory));
    	vulkan_check!(vk::vkMapMemory(
            logical_device.vk_data, staging_buffers_indices_memory, 0,
            mem_alloc.allocationSize, 0, &mut data));
    	unsafe {
            copy(transmute(index_buffer), data, index_buffer_size as usize);
            vk::vkUnmapMemory(logical_device.vk_data, staging_buffers_indices_memory);
        }
    	vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data, staging_buffers_indices_buffer,
            staging_buffers_indices_memory, 0));

    	// Create destination buffer with device only visibility
    	index_buffer_info.usage =
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
        let mut indices_buffer = 0 as vk::VkBuffer;
    	vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data, &index_buffer_info, null(), &mut indices_buffer));
    	unsafe {
            vk::vkGetBufferMemoryRequirements(
                logical_device.vk_data, indices_buffer, &mut mem_reqs);
        }
    	mem_alloc.allocationSize = mem_reqs.size;
    	mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32);
        let mut indices_memory = 0 as vk::VkDeviceMemory;
    	vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data, &mem_alloc, null(), &mut indices_memory));
    	vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data, indices_buffer, indices_memory, 0));
    	let mut cmd_buffer_begin_info = vk::VkCommandBufferBeginInfo::default();
    	cmd_buffer_begin_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    	VkCommandBuffer copyCmd = getCommandBuffer(true);

    	// Put buffer region copies into command buffer
    	VkBufferCopy copyRegion = {};

    	// Vertex buffer
    	copyRegion.size = vertex_buffer_size;
    	vkCmdCopyBuffer(copyCmd, staging_buffers.vertices.buffer, vertices.buffer, 1, &copyRegion);
    	// Index buffer
    	copyRegion.size = index_buffer_size;
    	vkCmdCopyBuffer(copyCmd, staging_buffers.indices.buffer, indices.buffer,	1, &copyRegion);

    	// Flushing the command buffer will also submit it to the queue and uses a fence to ensure that all commands have been executed before returning
    	flushCommandBuffer(copyCmd);

    	// Destroy staging buffers
    	// Note: Staging buffer must not be deleted before the copies have been submitted and executed
    	vkDestroyBuffer(logical_device.vk_data, staging_buffers.vertices.buffer, nullptr);
    	vkFreeMemory(logical_device.vk_data, staging_buffers.vertices.memory, nullptr);
    	vkDestroyBuffer(logical_device.vk_data, staging_buffers.indices.buffer, nullptr);
    	vkFreeMemory(logical_device.vk_data, staging_buffers.indices.memory, nullptr);

		// Vertex input binding
		vertices.inputBinding.binding = VERTEX_BUFFER_BIND_ID;
		vertices.inputBinding.stride = sizeof(Vertex);
		vertices.inputBinding.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;

		// Inpute attribute binding describe shader attribute locations and memory layouts
		// These match the following shader layout (see triangle.vert):
		//	layout (location = 0) in vec3 inPos;
		//	layout (location = 1) in vec3 inColor;
		vertices.inputAttributes.resize(2);
		// Attribute location 0: Position
		vertices.inputAttributes[0].binding = VERTEX_BUFFER_BIND_ID;
		vertices.inputAttributes[0].location = 0;
		vertices.inputAttributes[0].format = VK_FORMAT_R32G32B32_SFLOAT;
		vertices.inputAttributes[0].offset = offsetof(Vertex, position);
		// Attribute location 1: Color
		vertices.inputAttributes[1].binding = VERTEX_BUFFER_BIND_ID;
		vertices.inputAttributes[1].location = 1;
		vertices.inputAttributes[1].format = VK_FORMAT_R32G32B32_SFLOAT;
		vertices.inputAttributes[1].offset = offsetof(Vertex, color);

		// Assign to the vertex input state used for pipeline creation
		vertices.inputState.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
		vertices.inputState.pNext = nullptr;
		vertices.inputState.flags = VK_FLAGS_NONE;
		vertices.inputState.vertexBindingDescriptionCount = 1;
		vertices.inputState.pVertexBindingDescriptions = &vertices.inputBinding;
		vertices.inputState.vertexAttributeDescriptionCount = static_cast<uint32_t>(vertices.inputAttributes.size());
		vertices.inputState.pVertexAttributeDescriptions = vertices.inputAttributes.data();
    }
}
