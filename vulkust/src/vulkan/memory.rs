use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::device::logical::Logical as LogicalDevice;
use super::vulkan as vk;

use std::collections::BTreeMap;
use std::ptr::null;
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Memory {
    info: alc::Memory,
    size: vk::VkDeviceSize,
    vk_data: vk::VkDeviceMemory,
    manager: Arc<RwLock<Manager>>,
    root_memory: Arc<RwLock<RootMemory>>,
}

impl Memory {
    pub(crate) fn new(
        size: isize,
        mem_req: &vk::VkMemoryRequirements,
        vk_data: vk::VkDeviceMemory,
        manager: Arc<RwLock<Manager>>,
        root_memory: Arc<RwLock<RootMemory>>,
    ) -> Self {
        let info = alc::Memory::new(size, mem_req.alignment as isize);
        let size = mem_req.size;
        Memory {
            info,
            size,
            vk_data,
            manager,
            root_memory,
        }
    }

    pub(crate) fn get_root(&self) -> &Arc<RwLock<RootMemory>> {
        return &self.root_memory;
    }

    pub(crate) fn get_data(&self) -> vk::VkDeviceMemory {
        return self.vk_data;
    }

    pub(crate) fn get_manager(&self) -> &Arc<RwLock<Manager>> {
        return &self.manager;
    }
}

impl Object for Memory {
    fn get_size(&self) -> isize {
        return self.info.get_size();
    }

    fn get_offset(&self) -> isize {
        return self.info.get_offset();
    }

    fn get_offset_alignment(&self) -> isize {
        return self.info.get_offset_alignment();
    }

    fn get_offset_alignment_mask(&self) -> isize {
        return self.info.get_offset_alignment_mask();
    }

    fn get_offset_alignment_not_mask(&self) -> isize {
        return self.info.get_offset_alignment_not_mask();
    }

    fn place(&mut self, offset: isize) {
        self.info.place(offset);
    }
}

unsafe impl Send for Memory {}

unsafe impl Sync for Memory {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct RootMemory {
    alignment: isize,
    alignment_mask: isize,
    alignment_not_mask: isize,
    logical_device: Arc<LogicalDevice>,
    manager: Weak<RwLock<Manager>>,
    itself: Option<Weak<RwLock<RootMemory>>>,
    vk_data: vk::VkDeviceMemory,
    container: alc::Container,
}

const DEFAULT_MEMORY_SIZE: vk::VkDeviceSize = 512 * 1024 * 1024;

impl RootMemory {
    pub(crate) fn new(
        type_index: u32,
        manager: Weak<RwLock<Manager>>,
        logical_device: &Arc<LogicalDevice>,
    ) -> Arc<RwLock<Self>> {
        let alignment = logical_device.physical_device.get_max_min_alignment() as isize;
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = DEFAULT_MEMORY_SIZE;
        mem_alloc.memoryTypeIndex = type_index;
        let mut vk_data = 0 as vk::VkDeviceMemory;
        vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut vk_data,
        ));
        let alignment_mask = alignment -1;
        let alignment_not_mask = !alignment_mask;
        let itself = Arc::new(RwLock::new(RootMemory {
            alignment,
            alignment_mask,
            alignment_not_mask,
            logical_device: logical_device.clone(),
            vk_data,
            manager,
            itself: None,
            container: alc::Container::new(DEFAULT_MEMORY_SIZE as isize, 2),
        }));
        let w = Arc::downgrade(&itself);
        vxresult!(itself.write()).itself = Some(w);
        return itself;
    }

    pub(crate) fn allocate(&mut self, mem_req: &vk::VkMemoryRequirements) -> Arc<RwLock<Memory>> {
        let aligned_size = alc::align(mem_req.size as isize, self.alignment, self.alignment_mask, self.alignment_not_mask);
        let mem_alignment = mem_req.alignment as isize;
        let mem_alignment_mask = mem_alignment -1;
        let mem_alignment_not_mask = !mem_alignment_mask;
        let aligned_size = alc::align(aligned_size, mem_alignment, mem_alignment_mask, mem_alignment_not_mask);
        let manager = vxunwrap!(self.manager.upgrade());
        let itself = vxunwrap!(vxunwrap!(&self.itself).upgrade());
        let memory = Arc::new(RwLock::new(Memory::new(
            aligned_size,
            mem_req,
            self.vk_data,
            manager,
            itself,
        )));
        let obj: Arc<RwLock<Object>> = memory.clone();
        self.container.allocate(&obj);
        return memory;
    }

    pub(crate) fn get_data(&self) -> vk::VkDeviceMemory {
        return self.vk_data;
    }

    pub(crate) fn get_size(&self) -> isize {
        return self.container.get_size();
    }
}

impl Drop for RootMemory {
    fn drop(&mut self) {
        unsafe {
            vk::vkFreeMemory(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

unsafe impl Send for RootMemory {}

unsafe impl Sync for RootMemory {}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum Location {
    CPU,
    GPU,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    logical_device: Arc<LogicalDevice>,
    itself: Option<Weak<RwLock<Manager>>>,
    root_memories: BTreeMap<u32, Arc<RwLock<RootMemory>>>,
}

impl Manager {
    pub(crate) fn new(logical_device: &Arc<LogicalDevice>) -> Arc<RwLock<Self>> {
        let itself = Arc::new(RwLock::new(Manager {
            logical_device: logical_device.clone(),
            itself: None,
            root_memories: BTreeMap::new(),
        }));
        let w = Arc::downgrade(&itself);
        vxresult!(itself.write()).itself = Some(w);
        return itself;
    }

    pub(crate) fn get_memory_type_index(&self, mem_req: &vk::VkMemoryRequirements, l: Location) -> u32 {
        let l = match l {
            Location::GPU => {
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
            }
            Location::CPU => {
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
                    | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
            }
        };
        let ref memory_prop = self.logical_device.physical_device.memory_properties;
        let mut type_bits = mem_req.memoryTypeBits;
        for index in 0..memory_prop.memoryTypeCount {
            if (type_bits & 1) == 1
                && ((memory_prop.memoryTypes[index as usize].propertyFlags as u32) & l) != 0
            {
                return index;
            }
            type_bits >>= 1;
        }
        vxunexpected!();
    }

    pub(crate) fn allocate(
        &mut self,
        mem_req: &vk::VkMemoryRequirements,
        location: Location,
    ) -> Arc<RwLock<Memory>> {
        let memory_type_index = self.get_memory_type_index(mem_req, location);
        if let Some(root_memory) = self.root_memories.get_mut(&memory_type_index) {
            return vxresult!(root_memory.write()).allocate(mem_req);
        }
        let itself = vxunwrap!(&self.itself).clone();
        let root_memory = RootMemory::new(memory_type_index, itself, &self.logical_device);
        let allocated = vxresult!(root_memory.write()).allocate(mem_req);
        self.root_memories.insert(memory_type_index, root_memory);
        return allocated;
    }

    pub(crate) fn get_device(&self) -> &Arc<LogicalDevice> {
        return &self.logical_device;
    }
}

unsafe impl Send for Manager {}

unsafe impl Sync for Manager {}
