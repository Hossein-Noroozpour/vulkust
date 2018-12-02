use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::device::Logical as LogicalDevice;
use super::vulkan as vk;

use std::collections::BTreeMap;
use std::ptr::null;
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Memory {
    info: alc::Memory,
    vk_data: vk::VkDeviceMemory,
    manager: Arc<RwLock<Manager>>,
    root_memory: Arc<RwLock<RootMemory>>,
}

impl Memory {
    pub(crate) fn new(
        mem_req: &vk::VkMemoryRequirements,
        vk_data: vk::VkDeviceMemory,
        manager: Arc<RwLock<Manager>>,
        root_memory: Arc<RwLock<RootMemory>>,
    ) -> Self {
        let info = alc::Memory::new(mem_req.size as isize, mem_req.alignment as isize);
        Memory {
            info,
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
    fn get_allocated_memory(&self) -> &alc::Memory {
        return self.info.get_allocated_memory();
    }

    fn place(&mut self, offset: isize) {
        self.info.place(offset);
    }
}

unsafe impl Send for Memory {}

unsafe impl Sync for Memory {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct RootMemory {
    logical_device: Arc<LogicalDevice>,
    manager: Weak<RwLock<Manager>>,
    itself: Option<Weak<RwLock<RootMemory>>>,
    vk_data: vk::VkDeviceMemory,
    container: alc::Container,
}

const DEFAULT_MEMORY_SIZE: vk::VkDeviceSize = 600 * 1024 * 1024;

impl RootMemory {
    pub(crate) fn new(
        type_index: u32,
        manager: Weak<RwLock<Manager>>,
        logical_device: &Arc<LogicalDevice>,
    ) -> Arc<RwLock<Self>> {
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = DEFAULT_MEMORY_SIZE;
        mem_alloc.memoryTypeIndex = type_index;
        let mut vk_data = 0 as vk::VkDeviceMemory;
        vulkan_check!(vk::vkAllocateMemory(
            logical_device.get_data(),
            &mem_alloc,
            null(),
            &mut vk_data,
        ));
        let itself = Arc::new(RwLock::new(RootMemory {
            logical_device: logical_device.clone(),
            vk_data,
            manager,
            itself: None,
            container: alc::Container::new(DEFAULT_MEMORY_SIZE as isize, 1),
        }));
        let w = Arc::downgrade(&itself);
        vxresult!(itself.write()).itself = Some(w);
        return itself;
    }

    pub(crate) fn allocate(&mut self, mem_req: &vk::VkMemoryRequirements) -> Arc<RwLock<Memory>> {
        let manager = vxunwrap!(self.manager.upgrade());
        let itself = vxunwrap!(vxunwrap!(&self.itself).upgrade());
        let memory = Arc::new(RwLock::new(Memory::new(
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
        return self.container.get_allocated_memory().get_size();
    }
}

impl Drop for RootMemory {
    fn drop(&mut self) {
        unsafe {
            vk::vkFreeMemory(self.logical_device.get_data(), self.vk_data, null());
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

    pub(crate) fn get_memory_type_index(
        &self,
        mem_req: &vk::VkMemoryRequirements,
        l: Location,
    ) -> u32 {
        let l = match l {
            Location::GPU => {
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
            }
            Location::CPU => {
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
                    | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
            }
        };
        return self
            .logical_device
            .get_physical()
            .get_memory_type_index(mem_req.memoryTypeBits, l);
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
