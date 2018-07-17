use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::device::logical::Logical as LogicalDevice;
use super::vulkan as vk;

use std::collections::BTreeMap;
use std::ptr::null;
use std::sync::{Arc, RwLock, Weak};

pub struct Memory {
    pub info: alc::Memory,
    pub size: vk::VkDeviceSize,
    pub vk_data: vk::VkDeviceMemory,
    pub manager: Arc<RwLock<Manager>>,
    pub root_memory: Arc<RwLock<RootMemory>>,
}

impl Memory {
    pub fn new(
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
}

impl Object for Memory {
    fn get_size(&self) -> isize {
        self.info.get_size()
    }

    fn get_offset(&self) -> isize {
        self.info.get_offset()
    }

    fn get_offset_alignment(&self) -> isize {
        self.info.get_offset_alignment()
    }

    fn place(&mut self, offset: isize) {
        self.info.place(offset);
    }
}

pub struct RootMemory {
    pub alignment: isize,
    pub logical_device: Arc<LogicalDevice>,
    pub manager: Weak<RwLock<Manager>>,
    itself: Option<Weak<RwLock<RootMemory>>>,
    pub vk_data: vk::VkDeviceMemory,
    pub container: alc::Container,
}

const DEFAULT_MEMORY_SIZE: vk::VkDeviceSize = 128 * 1024 * 1024;

impl RootMemory {
    pub fn new(
        type_index: u32,
        manager: Weak<RwLock<Manager>>,
        logical_device: &Arc<LogicalDevice>,
    ) -> Self {
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
        RootMemory {
            alignment,
            logical_device: logical_device.clone(),
            vk_data,
            manager,
            itself: None,
            container: alc::Container::new(DEFAULT_MEMORY_SIZE as isize, 2),
        }
    }

    pub fn allocate(&mut self, mem_req: &vk::VkMemoryRequirements) -> Arc<RwLock<Memory>> {
        let aligned_size = alc::align(mem_req.size as isize, self.alignment);
        let aligned_size = alc::align(aligned_size, mem_req.alignment as isize);
        let manager = vxunwrap_o!(self.manager.upgrade());
        let itself = vxunwrap_o!(vxunwrap!(self.itself).upgrade());
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

    pub fn set_itself(&mut self, itself: Weak<RwLock<RootMemory>>) {
        self.itself = Some(itself);
    }
}

impl Drop for RootMemory {
    fn drop(&mut self) {
        unsafe {
            vk::vkFreeMemory(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

pub enum Location {
    CPU,
    GPU,
}

pub struct Manager {
    pub logical_device: Arc<LogicalDevice>,
    itself: Option<Weak<RwLock<Manager>>>,
    root_memories: BTreeMap<u32, Arc<RwLock<RootMemory>>>,
}

impl Manager {
    pub fn new(logical_device: &Arc<LogicalDevice>) -> Self {
        Manager {
            logical_device: logical_device.clone(),
            itself: None,
            root_memories: BTreeMap::new(),
        }
    }

    pub fn get_memory_type_index(&self, mem_req: &vk::VkMemoryRequirements, l: Location) -> u32 {
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

    pub fn set_itself(&mut self, itself: Weak<RwLock<Manager>>) {
        self.itself = Some(itself);
    }

    pub fn allocate(
        &mut self,
        mem_req: &vk::VkMemoryRequirements,
        location: Location,
    ) -> Arc<RwLock<Memory>> {
        let memory_type_index = self.get_memory_type_index(mem_req, location);
        if let Some(root_memory) = self.root_memories.get_mut(&memory_type_index) {
            return vxresult!(root_memory.write()).allocate(mem_req);
        }
        let itself = vxunwrap!(self.itself).clone();
        let root_memory = RootMemory::new(memory_type_index, itself, &self.logical_device);
        let root_memory = Arc::new(RwLock::new(root_memory));
        let root_memory_w = Arc::downgrade(&root_memory);
        let allocated = {
            let mut root_memory_wl = vxresult!(root_memory.write());
            root_memory_wl.set_itself(root_memory_w);
            root_memory_wl.allocate(mem_req)
        };
        self.root_memories.insert(memory_type_index, root_memory);
        return allocated;
    }
}
