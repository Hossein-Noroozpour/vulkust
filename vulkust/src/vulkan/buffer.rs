use libc;

use std::default::Default;
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::sync::Arc;
use super::super::render::mesh::INDEX_ELEMENTS_SIZE;
use super::super::system::vulkan as vk;
use super::super::util::cell::DebugCell;
use super::super::util::gc::{Gc, GcObject};
use super::command::buffer::Buffer as CmdBuff;
use super::device::logical::Logical as LogicalDevice;

struct Buffer {
    logical_device: Arc<LogicalDevice>,
    main_buffer: vk::VkBuffer,
    main_memory: vk::VkDeviceMemory,
    staging_buffer: vk::VkBuffer,
    staging_memory: vk::VkDeviceMemory,
    address: *mut libc::c_void,
    size: usize,
    best_alignment: usize,
    best_alignment_flag: usize,
    best_alignment_complement: usize,
}

impl Buffer {
    fn new(logical_device: Arc<LogicalDevice>, size: usize) -> Self {
        let best_alignment = logical_device.physical_device.get_max_min_alignment() as usize;
        let mut main_buffer = null_mut();
        let mut main_memory = null_mut();
        let mut staging_buffer = null_mut();
        let mut staging_memory = null_mut();
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = size as vk::VkDeviceSize;
        buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
        vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data,
            &buffer_info,
            null(),
            &mut main_buffer,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetBufferMemoryRequirements(logical_device.vk_data, main_buffer, &mut mem_reqs);
        }
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = mem_reqs.size;
        mem_alloc.memoryTypeIndex = logical_device
            .physical_device
            .get_memory_type_index(
                mem_reqs.memoryTypeBits,
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
            );
        vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut main_memory,
        ));
        vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data,
            main_buffer,
            main_memory,
            0,
        ));
        buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32;
        vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data,
            &buffer_info,
            null(),
            &mut staging_buffer,
        ));
        unsafe {
            vk::vkGetBufferMemoryRequirements(logical_device.vk_data, staging_buffer, &mut mem_reqs);
        }
        mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
        );
        vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut staging_memory,
        ));
        let mut address = 0 as *mut libc::c_void;
        vulkan_check!(vk::vkMapMemory(
            logical_device.vk_data,
            staging_memory,
            0,
            mem_alloc.allocationSize,
            0,
            transmute(&mut address),
        ));
        vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data,
            staging_buffer,
            staging_memory,
            0,
        ));
        Buffer {
            logical_device: logical_device, 
            size: size,
            address: address,
            main_buffer: main_buffer,
            main_memory: main_memory,
            staging_buffer: staging_buffer,
            staging_memory: staging_memory,
            best_alignment: best_alignment,
            best_alignment_flag: best_alignment - 1,
            best_alignment_complement: !(best_alignment - 1),
        }
    }

    fn size_aligner(&self, size: usize) -> usize {
        (size & self.best_alignment_complement) + 
            if (size & self.best_alignment_flag) != 0 {
                self.best_alignment
            } else {
                0
            }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.main_buffer == null_mut() {
            return;
        }
        unsafe {
            vk::vkDestroyBuffer(self.logical_device.vk_data, self.staging_buffer, null());
            vk::vkFreeMemory(self.logical_device.vk_data, self.staging_memory, null());
            vk::vkDestroyBuffer(self.logical_device.vk_data, self.main_buffer, null());
            vk::vkFreeMemory(self.logical_device.vk_data, self.main_memory, null());
        }
        self.main_buffer = null_mut();
        self.main_memory = null_mut();
        self.staging_buffer = null_mut();
        self.staging_memory = null_mut();
    }
}

struct BufferGcObject {
    need_refresh: bool,
    offset: usize,
    aligned_size: usize,
    buffer: Arc<Buffer>,
}

impl GcObject for BufferGcObject {
    fn get_size(&self) -> usize {
        return self.aligned_size;
    }

    fn move_to(&mut self, offset: usize) {
        self.need_refresh = true;
        let new_address = unsafe { self.buffer.address.offset(offset as isize) };
        let old_address = unsafe { self.buffer.address.offset(self.offset as isize) };
        unsafe {
            libc::memmove(
                transmute(new_address), 
                transmute(old_address),
                self.aligned_size as libc::size_t);
        }
        self.offset = offset;
    }
}

impl BufferGcObject {
    fn write(&mut self, offset: usize, data: *const libc::c_void, size: usize) {
        unsafe {
            libc::memcpy(transmute(self.buffer.address.offset(
                (self.offset + offset) as isize)), data, size as libc::size_t);
        }
    }
}

pub struct MeshBuffer {
    buffer_gc_obj: BufferGcObject,
    vertex_size: usize,
    vertices_count: usize,
    vertices_size: usize,
    vertices_aligned_size: usize,
    indices_count: usize,
    indices_size: usize,
    indices_size_aligned: usize,
}

impl GcObject for MeshBuffer {
    fn get_size(&self) -> usize {
        return self.buffer_gc_obj.aligned_size;
    }

    fn move_to(&mut self, offset: usize) {
        self.buffer_gc_obj.move_to(offset);
    }
}

impl MeshBuffer {
    pub fn upload_vertices(&mut self, data: *const libc::c_void, length: usize) {
        #[cfg(buffer_debug)]
        {   
            if length > self.vertices_size {
                logf!("Unexpected size of data.");
            }
        }
        self.buffer_gc_obj.write(0, data, length);
    }

    pub fn upload_indices(&mut self, data: *const libc::c_void, length: usize) {
        #[cfg(buffer_debug)]
        {   
            if length > self.indices_size {
                logf!("Unexpected size of data.");
            }
        }
        self.buffer_gc_obj.write(self.vertices_aligned_size, data, length);
    }
}

pub struct UniformBuffer {
    buffer_gc_obj: BufferGcObject,
    size: usize,
}

impl UniformBuffer {
    pub fn upload(&mut self, data: *const libc::c_void) {
        self.buffer_gc_obj.write(0, data, self.size);
    }
}

impl GcObject for UniformBuffer {
    fn get_size(&self) -> usize {
        return self.buffer_gc_obj.aligned_size;
    }

    fn move_to(&mut self, offset: usize) {
        self.buffer_gc_obj.move_to(offset);
    }
}

pub struct SceneDynamics {
    buffer: Arc<Buffer>,
    uniforms: Gc,
}

impl SceneDynamics {
    pub fn create_uniform(&mut self, size: usize) -> Arc<DebugCell<UniformBuffer>> {
        let aligned_size = self.buffer.size_aligner(size);
        let uniform = Arc::new(
            DebugCell::new(
                UniformBuffer {
                    size: size,
                    buffer_gc_obj: BufferGcObject {
                        need_refresh: true,
                        offset: 0,
                        aligned_size: aligned_size,
                        buffer: self.buffer.clone(),
                    },
                }
            )
        );
        let gc_obj: Arc<DebugCell<GcObject>> = uniform.clone();
        self.uniforms.allocate(&gc_obj);
        return uniform;
    }
}

impl GcObject for SceneDynamics {
    fn get_size(&self) -> usize {
        return self.uniforms.get_size();
    }

    fn move_to(&mut self, offset: usize) {
        self.uniforms.move_to(offset);
    }
}

pub struct Manager {
    buffer: Arc<Buffer>,
    meshes_size: usize,
    meshes: Gc,
    scenes_dynamics_size: usize,
    scenes_dynamics: Vec<Gc>,
}

impl Manager {
    pub fn new(
        logical_device: Arc<LogicalDevice>,
        meshes_size: usize, 
        scenes_dynamics_size: usize, 
        frames_count: usize) -> Self {
        let size = meshes_size + (scenes_dynamics_size * frames_count);
        let meshes = Gc::new(0, meshes_size);
        let mut scenes_dynamics = Vec::new();
        for i in 0..frames_count {
            scenes_dynamics.push(Gc::new(i * scenes_dynamics_size + meshes_size, scenes_dynamics_size));
        }
        Manager {
            buffer: Arc::new(Buffer::new(logical_device, size)),
            meshes_size: meshes_size,
            meshes: meshes,
            scenes_dynamics_size: scenes_dynamics_size,
            scenes_dynamics: scenes_dynamics,
        }
    }

    fn clean(&mut self) {
        self.meshes.clean();
        let sdc = self.scenes_dynamics.len();
        for i in 0..sdc {
            self.scenes_dynamics[i].clean();
        }
    }

    pub fn create_mesh(
        &mut self, 
        vertex_size: usize, 
        vertices_count: usize, 
        indices_count: usize) -> Arc<DebugCell<MeshBuffer>> {
        let vertices_size = vertices_count * vertex_size;
        let indices_size = indices_count * INDEX_ELEMENTS_SIZE;
        let vertices_aligned_size = self.buffer.size_aligner(vertices_size);
        let indices_size_aligned = self.buffer.size_aligner(indices_size);
        let size = vertices_aligned_size + indices_size_aligned;
        let mesh = Arc::new(
            DebugCell::new(
                MeshBuffer {
                    buffer_gc_obj: BufferGcObject {
                        need_refresh: true,
                        offset: 0,
                        aligned_size: size,
                        buffer: self.buffer.clone(),
                    },
                    vertex_size: vertex_size,
                    vertices_count: vertices_count,
                    vertices_size: vertices_size,
                    vertices_aligned_size: vertices_aligned_size,
                    indices_count: indices_count,
                    indices_size: indices_size,
                    indices_size_aligned: indices_size_aligned,
                }
            )
        );
        let gc_obj: Arc<DebugCell<GcObject>> = mesh.clone();
        self.meshes.allocate(&gc_obj);
        return mesh;
    }

    pub fn create_scene_dynamics(&mut self, size: usize) -> Vec<Arc<DebugCell<SceneDynamics>>> {
        let frames_count = self.scenes_dynamics.len();
        let mut res = Vec::new();
        for i in 0..frames_count {
            let sd = Arc::new(DebugCell::new(
                SceneDynamics {
                    buffer: self.buffer.clone(),
                    uniforms: Gc::new(0, size),
                }
            ));
            let gc_obj: Arc<DebugCell<GcObject>> = sd.clone();
            self.scenes_dynamics[i].allocate(&gc_obj);
            res.push(sd);
        }
        return res;
    }

    fn commit_meshes(&self, cmd: &mut CmdBuff) {
        let mut region = vk::VkBufferCopy::default();
        region.srcOffset = 0 as vk::VkDeviceSize;
        region.dstOffset = 0 as vk::VkDeviceSize;
        region.size = self.scenes_dynamics_size as vk::VkDeviceSize;
        let regions = vec![region; 1];
        cmd.copy_buffer(self.buffer.staging_buffer, self.buffer.main_buffer, &regions);
    }

    fn commit_scenes_dynamics(&self, frame_number: usize, cmd: &mut CmdBuff) {
        let start = self.meshes_size + (self.scenes_dynamics_size * frame_number);
        let mut region = vk::VkBufferCopy::default();
        region.srcOffset = start as vk::VkDeviceSize;
        region.dstOffset = start as vk::VkDeviceSize;
        region.size = self.scenes_dynamics_size as vk::VkDeviceSize;
        let regions = vec![region; 1];
        cmd.copy_buffer(self.buffer.staging_buffer, self.buffer.main_buffer, &regions);
    }

    pub fn get_buffer(&self) -> vk::VkBuffer {
        self.buffer.main_buffer
    }

    pub fn get_device(&self) -> &Arc<LogicalDevice> {
        &self.buffer.logical_device
    }
}