use super::super::system::file::File;
use super::device::logical::Logical as LogicalDevice;
use super::vulkan as vk;
use std::io::Read;
use std::mem::transmute;
use std::ptr::null;
use std::sync::Arc;

pub struct Module {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkShaderModule,
}

impl Module {
    pub fn new_with_file(file_name: String, logical_device: Arc<LogicalDevice>) -> Self {
        let mut file = vxresult!(File::open(file_name));
        let mut data = Vec::new();
        let _ = vxresult!(file.read_to_end(&mut data));
        Self::new(data, logical_device)
    }

    pub fn new(data: Vec<u8>, logical_device: Arc<LogicalDevice>) -> Self {
        let mut module_create_info = vk::VkShaderModuleCreateInfo::default();
        module_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
        module_create_info.codeSize = data.len();
        module_create_info.pCode = unsafe { transmute(data.as_ptr()) };
        let mut vk_data = 0 as vk::VkShaderModule;
        vulkan_check!(vk::vkCreateShaderModule(
            logical_device.vk_data,
            &module_create_info,
            null(),
            &mut vk_data,
        ));
        Module {
            logical_device,
            vk_data,
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyShaderModule(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}
