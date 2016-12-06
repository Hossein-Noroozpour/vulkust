

pub struct Mesh {

}

impl Mesh {
    pub fn new() {
        let vertex_buffer = [
            1.0f32, 1.0f32, 0.0f32,   1.0f32,  0.0f32, 0.0f32,   -1.0f32, 1.0f32, 0.0f32,
            0.0f32, 1.0f32, 0.0f32,   0.0f32, -1.0f32, 0.0f32,    0.0f32, 0.0f32, 1.0f32,
        ];
        let vertex_buffer_size = 72u32;
        let index_buffer = [0u32, 1u32, 2u32];
        let indices_count = 3u32;
        let index_buffer_size = 12u32;
        // TODO: all vulkan-sys related works must move to vulkan module
        VkMemoryAllocateInfo memAlloc = {};
        memAlloc.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        VkMemoryRequirements memReqs;

        void *data;
        // Static data like vertex and index buffer should be stored on the device memory
        // for optimal (and fastest) access by the GPU
        //
        // To achieve this we use so-called "staging buffers" :
        // - Create a buffer that's visible to the host (and can be mapped)
        // - Copy the data to this buffer
        // - Create another buffer that's local on the device (VRAM) with the same size
        // - Copy the data from the host to the device using a command buffer
        // - Delete the host visible (staging) buffer
        // - Use the device local buffers for rendering

        struct StagingBuffer {
        VkDeviceMemory memory;
        VkBuffer buffer;
        };

        struct {
        StagingBuffer vertices;
        StagingBuffer indices;
        } stagingBuffers;

        // Vertex buffer
        VkBufferCreateInfo vertexBufferInfo = {};
        vertexBufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        vertexBufferInfo.size = vertexBufferSize;
        // Buffer is used as the copy source
        vertexBufferInfo.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
        // Create a host-visible buffer to copy the vertex data to (staging buffer)
        VK_CHECK_RESULT(vkCreateBuffer(device, &vertexBufferInfo, nullptr, &stagingBuffers.vertices.buffer));
        vkGetBufferMemoryRequirements(device, stagingBuffers.vertices.buffer, &memReqs);
        memAlloc.allocationSize = memReqs.size;
        // Request a host visible memory type that can be used to copy our data do
        // Also request it to be coherent, so that writes are visible to the GPU right after unmapping the buffer
        memAlloc.memoryTypeIndex = getMemoryTypeIndex(memReqs.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
        VK_CHECK_RESULT(vkAllocateMemory(device, &memAlloc, nullptr, &stagingBuffers.vertices.memory));
        // Map and copy
        VK_CHECK_RESULT(vkMapMemory(device, stagingBuffers.vertices.memory, 0, memAlloc.allocationSize, 0, &data));
        memcpy(data, vertexBuffer.data(), vertexBufferSize);
        vkUnmapMemory(device, stagingBuffers.vertices.memory);
        VK_CHECK_RESULT(vkBindBufferMemory(device, stagingBuffers.vertices.buffer, stagingBuffers.vertices.memory, 0));

        // Create a device local buffer to which the (host local) vertex data will be copied and which will be used for rendering
        vertexBufferInfo.usage = VK_BUFFER_USAGE_VERTEX_BUFFER_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        VK_CHECK_RESULT(vkCreateBuffer(device, &vertexBufferInfo, nullptr, &vertices.buffer));
        vkGetBufferMemoryRequirements(device, vertices.buffer, &memReqs);
        memAlloc.allocationSize = memReqs.size;
        memAlloc.memoryTypeIndex = getMemoryTypeIndex(memReqs.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
        VK_CHECK_RESULT(vkAllocateMemory(device, &memAlloc, nullptr, &vertices.memory));
        VK_CHECK_RESULT(vkBindBufferMemory(device, vertices.buffer, vertices.memory, 0));

        // Index buffer
        VkBufferCreateInfo indexbufferInfo = {};
        indexbufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        indexbufferInfo.size = indexBufferSize;
        indexbufferInfo.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
        // Copy index data to a buffer visible to the host (staging buffer)
        VK_CHECK_RESULT(vkCreateBuffer(device, &indexbufferInfo, nullptr, &stagingBuffers.indices.buffer));
        vkGetBufferMemoryRequirements(device, stagingBuffers.indices.buffer, &memReqs);
        memAlloc.allocationSize = memReqs.size;
        memAlloc.memoryTypeIndex = getMemoryTypeIndex(memReqs.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
        VK_CHECK_RESULT(vkAllocateMemory(device, &memAlloc, nullptr, &stagingBuffers.indices.memory));
        VK_CHECK_RESULT(vkMapMemory(device, stagingBuffers.indices.memory, 0, indexBufferSize, 0, &data));
        memcpy(data, indexBuffer.data(), indexBufferSize);
        vkUnmapMemory(device, stagingBuffers.indices.memory);
        VK_CHECK_RESULT(vkBindBufferMemory(device, stagingBuffers.indices.buffer, stagingBuffers.indices.memory, 0));

        // Create destination buffer with device only visibility
        indexbufferInfo.usage = VK_BUFFER_USAGE_INDEX_BUFFER_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        VK_CHECK_RESULT(vkCreateBuffer(device, &indexbufferInfo, nullptr, &indices.buffer));
        vkGetBufferMemoryRequirements(device, indices.buffer, &memReqs);
        memAlloc.allocationSize = memReqs.size;
        memAlloc.memoryTypeIndex = getMemoryTypeIndex(memReqs.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
        VK_CHECK_RESULT(vkAllocateMemory(device, &memAlloc, nullptr, &indices.memory));
        VK_CHECK_RESULT(vkBindBufferMemory(device, indices.buffer, indices.memory, 0));

        VkCommandBufferBeginInfo cmdBufferBeginInfo = {};
        cmdBufferBeginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        cmdBufferBeginInfo.pNext = nullptr;

        // Buffer copies have to be submitted to a queue, so we need a command buffer for them
        // Note: Some devices offer a dedicated transfer queue (with only the transfer bit set) that may be faster when doing lots of copies
        VkCommandBuffer copyCmd = getCommandBuffer(true);

        // Put buffer region copies into command buffer
        VkBufferCopy copyRegion = {};

        // Vertex buffer
        copyRegion.size = vertexBufferSize;
        vkCmdCopyBuffer(copyCmd, stagingBuffers.vertices.buffer, vertices.buffer, 1, &copyRegion);
        // Index buffer
        copyRegion.size = indexBufferSize;
        vkCmdCopyBuffer(copyCmd, stagingBuffers.indices.buffer, indices.buffer,	1, &copyRegion);

        // Flushing the command buffer will also submit it to the queue and uses a fence to ensure that all commands have been executed before returning
        flushCommandBuffer(copyCmd);

        // Destroy staging buffers
        // Note: Staging buffer must not be deleted before the copies have been submitted and executed
        vkDestroyBuffer(device, stagingBuffers.vertices.buffer, nullptr);
        vkFreeMemory(device, stagingBuffers.vertices.memory, nullptr);
        vkDestroyBuffer(device, stagingBuffers.indices.buffer, nullptr);
        vkFreeMemory(device, stagingBuffers.indices.memory, nullptr);

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
        vertices.inputState.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
        vertices.inputState.pNext = nullptr;
        vertices.inputState.flags = VK_FLAGS_NONE;
        vertices.inputState.vertexBindingDescriptionCount = 1;
        vertices.inputState.pVertexBindingDescriptions = &vertices.inputBinding;
        vertices.inputState.vertexAttributeDescriptionCount = static_cast<uint32_t>(vertices.inputAttributes.size());
        vertices.inputState.pVertexAttributeDescriptions = vertices.inputAttributes.data();
    }
}