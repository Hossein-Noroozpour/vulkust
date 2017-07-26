pub mod uniform;
use super::super::math::number::Float;
use super::super::system::vulkan as vk;
use super::device::logical::Logical as LogicalDevice;
use super::command::pool::Pool as CmdPool;
use super::command::buffer::Buffer as CmdBuff;
use std::default::Default;
use std::sync::Arc;
use std::ptr::{copy, null, null_mut};
use std::os::raw::c_void;
use std::mem::{transmute, size_of};

struct Buffer {
    pub cmd_pool: Arc<CmdPool>,
    pub vk_data: vk::VkBuffer,
    pub memory: vk::VkDeviceMemory,
    pub offset: u32,
    pub size: u32,
}

impl Buffer {
    fn create(&mut self, size: u32) {
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = size as vk::VkDeviceSize;
        buffer_info.usage =
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
        vulkan_check!(vk::vkCreateBuffer(
            self.cmd_pool.logical_device.vk_data,
            &buffer_info,
            null(),
            &mut self.vk_data,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetBufferMemoryRequirements(
                self.cmd_pool.logical_device.vk_data,
                self.vk_data,
                &mut mem_reqs,
            );
        }
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = mem_reqs.size;
        mem_alloc.memoryTypeIndex = self.cmd_pool.logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32);
        let mut vertices_memory = 0 as vk::VkDeviceMemory;
        vulkan_check!(vk::vkAllocateMemory(
            self.cmd_pool.logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut self.memory,
        ));
        vulkan_check!(vk::vkBindBufferMemory(
            self.cmd_pool.logical_device.vk_data,
            self.vk_data,
            self.memory,
            0,
        ));
    }

    pub fn new(cmd_pool: Arc<CmdPool>, size_hint: u32) -> Self {
        let mut b = Buffer {
            cmd_pool: cmd_pool,
            vk_data: null_mut(),
            memory: null_mut(),
            size: 0,
        };
        b.create();
        return b;
    }

    pub fn append()
}

// pub struct Buffer {
//     pub logical_device: Arc<LogicalDevice>,
//     pub vertices_buffer: vk::VkBuffer,
//     pub vertices_memory: vk::VkDeviceMemory,
//     pub indices_buffer: vk::VkBuffer,
//     pub indices_memory: vk::VkDeviceMemory,
//     pub indices_count: u32,
// }

// impl Buffer {
//     pub fn new(
//         logical_device: Arc<LogicalDevice>,
//         cmd_pool: Arc<CmdPool>,
//         vertex_buffer: *const u8,
//         vertex_buffer_size: u32,
//         index_buffer: *const u8,
//         index_buffer_size: u32,
//     ) -> Buffer {
//         let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
//         mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
//         let mut mem_reqs = vk::VkMemoryRequirements::default();
//         let mut data = 0 as *mut c_void;
//         let mut staging_buffers_vertices_memory = 0 as vk::VkDeviceMemory;
//         let mut staging_buffers_vertices_buffer = 0 as vk::VkBuffer;
//         let mut staging_buffers_indices_memory = 0 as vk::VkDeviceMemory;
//         let mut staging_buffers_indices_buffer = 0 as vk::VkBuffer;
//         let mut vertex_buffer_info = vk::VkBufferCreateInfo::default();
//         vertex_buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
//         vertex_buffer_info.size = vertex_buffer_size as vk::VkDeviceSize;
//         vertex_buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as
//             u32;
//         vulkan_check!(vk::vkCreateBuffer(
//             logical_device.vk_data,
//             &vertex_buffer_info,
//             null(),
//             &mut staging_buffers_vertices_buffer,
//         ));
//         unsafe {
//             vk::vkGetBufferMemoryRequirements(
//                 logical_device.vk_data,
//                 staging_buffers_vertices_buffer,
//                 &mut mem_reqs,
//             );
//         }
//         mem_alloc.allocationSize = mem_reqs.size;
//         mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
//             mem_reqs.memoryTypeBits,
//             vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
//                 vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
//         );
//         vulkan_check!(vk::vkAllocateMemory(
//             logical_device.vk_data,
//             &mem_alloc,
//             null(),
//             &mut staging_buffers_vertices_memory,
//         ));
//         vulkan_check!(vk::vkMapMemory(
//             logical_device.vk_data,
//             staging_buffers_vertices_memory,
//             0,
//             mem_alloc.allocationSize,
//             0,
//             &mut data,
//         ));
//         unsafe {
//             copy(vertex_buffer, transmute(data), vertex_buffer_size as usize);
//             vk::vkUnmapMemory(logical_device.vk_data, staging_buffers_vertices_memory);
//         }
//         vulkan_check!(vk::vkBindBufferMemory(
//             logical_device.vk_data,
//             staging_buffers_vertices_buffer,
//             staging_buffers_vertices_memory,
//             0,
//         ));
//         vertex_buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as
//             u32 |
//             vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
//         let mut vertices_buffer = 0 as vk::VkBuffer;
//         vulkan_check!(vk::vkCreateBuffer(
//             logical_device.vk_data,
//             &vertex_buffer_info,
//             null(),
//             &mut vertices_buffer,
//         ));
//         unsafe {
//             vk::vkGetBufferMemoryRequirements(
//                 logical_device.vk_data,
//                 vertices_buffer,
//                 &mut mem_reqs,
//             );
//         }
//         mem_alloc.allocationSize = mem_reqs.size;
//         mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
//             mem_reqs.memoryTypeBits,
//             vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
//         );
//         let mut vertices_memory = 0 as vk::VkDeviceMemory;
//         vulkan_check!(vk::vkAllocateMemory(
//             logical_device.vk_data,
//             &mem_alloc,
//             null(),
//             &mut vertices_memory,
//         ));
//         vulkan_check!(vk::vkBindBufferMemory(
//             logical_device.vk_data,
//             vertices_buffer,
//             vertices_memory,
//             0,
//         ));
//         let mut index_buffer_info = vk::VkBufferCreateInfo::default();
//         index_buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
//         index_buffer_info.size = index_buffer_size as vk::VkDeviceSize;
//         index_buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as
//             u32;
//         vulkan_check!(vk::vkCreateBuffer(
//             logical_device.vk_data,
//             &index_buffer_info,
//             null(),
//             &mut staging_buffers_indices_buffer,
//         ));
//         unsafe {
//             vk::vkGetBufferMemoryRequirements(
//                 logical_device.vk_data,
//                 staging_buffers_indices_buffer,
//                 &mut mem_reqs,
//             );
//         }
//         mem_alloc.allocationSize = mem_reqs.size;
//         mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
//             mem_reqs.memoryTypeBits,
//             vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
//                 vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
//         );
//         vulkan_check!(vk::vkAllocateMemory(
//             logical_device.vk_data,
//             &mem_alloc,
//             null(),
//             &mut staging_buffers_indices_memory,
//         ));
//         vulkan_check!(vk::vkMapMemory(
//             logical_device.vk_data,
//             staging_buffers_indices_memory,
//             0,
//             mem_alloc.allocationSize,
//             0,
//             &mut data,
//         ));
//         unsafe {
//             copy(index_buffer, transmute(data), index_buffer_size as usize);
//             vk::vkUnmapMemory(logical_device.vk_data, staging_buffers_indices_memory);
//         }
//         vulkan_check!(vk::vkBindBufferMemory(
//             logical_device.vk_data,
//             staging_buffers_indices_buffer,
//             staging_buffers_indices_memory,
//             0,
//         ));
//
//         // Create destination buffer with device only visibility
//         index_buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as
//             u32 |
//             vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
//         let mut indices_buffer = 0 as vk::VkBuffer;
//         vulkan_check!(vk::vkCreateBuffer(
//             logical_device.vk_data,
//             &index_buffer_info,
//             null(),
//             &mut indices_buffer,
//         ));
//         unsafe {
//             vk::vkGetBufferMemoryRequirements(
//                 logical_device.vk_data,
//                 indices_buffer,
//                 &mut mem_reqs,
//             );
//         }
//         mem_alloc.allocationSize = mem_reqs.size;
//         mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
//             mem_reqs.memoryTypeBits,
//             vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
//         );
//         let mut indices_memory = 0 as vk::VkDeviceMemory;
//         vulkan_check!(vk::vkAllocateMemory(
//             logical_device.vk_data,
//             &mem_alloc,
//             null(),
//             &mut indices_memory,
//         ));
//         vulkan_check!(vk::vkBindBufferMemory(
//             logical_device.vk_data,
//             indices_buffer,
//             indices_memory,
//             0,
//         ));
//         let mut cmd_buffer_begin_info = vk::VkCommandBufferBeginInfo::default();
//         cmd_buffer_begin_info.sType =
//             vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
//         let copy_cmd = CmdBuff::new(cmd_pool);
//         let mut copy_region = vk::VkBufferCopy::default();
//         copy_region.size = vertex_buffer_size as vk::VkDeviceSize;
//         unsafe {
//             vk::vkCmdCopyBuffer(
//                 copy_cmd.vk_data,
//                 staging_buffers_vertices_buffer,
//                 vertices_buffer,
//                 1,
//                 &copy_region,
//             );
//         }
//         copy_region.size = index_buffer_size as vk::VkDeviceSize;
//         unsafe {
//             vk::vkCmdCopyBuffer(
//                 copy_cmd.vk_data,
//                 staging_buffers_indices_buffer,
//                 indices_buffer,
//                 1,
//                 &copy_region,
//             );
//         }
//         copy_cmd.flush();
//         unsafe {
//             vk::vkDestroyBuffer(
//                 logical_device.vk_data,
//                 staging_buffers_vertices_buffer,
//                 null(),
//             );
//             vk::vkFreeMemory(
//                 logical_device.vk_data,
//                 staging_buffers_vertices_memory,
//                 null(),
//             );
//             vk::vkDestroyBuffer(
//                 logical_device.vk_data,
//                 staging_buffers_indices_buffer,
//                 null(),
//             );
//             vk::vkFreeMemory(
//                 logical_device.vk_data,
//                 staging_buffers_indices_memory,
//                 null(),
//             );
//         }
//         Buffer {
//             logical_device: logical_device,
//             vertices_buffer: vertices_buffer,
//             vertices_memory: vertices_memory,
//             indices_buffer: indices_buffer,
//             indices_memory: indices_memory,
//             indices_count: index_buffer_size / size_of::<u32>() as u32,
//         }
//     }
// }

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.vk_data == null_mut() {
            return;
        }
        unsafe {
            vk::vkDestroyBuffer(self.cmd_pool.logical_device.vk_data, self.vk_data, null());
            vk::vkFreeMemory(self.cmd_pool.logical_device.vk_data, self.memory, null());
        }
    }
}
