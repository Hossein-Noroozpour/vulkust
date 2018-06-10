use std::mem::transmute;
use std::sync::Arc;
use std::ptr::null;
use super::vulkan as vk;
use super::device::logical::Logical as LogicalDevice;

pub struct Module {
    pub logical_device: Arc<LogicalDevice>,
    pub module: vk::VkShaderModule,
}

impl Module {
    pub fn new(file_name: String, logical_device: Arc<LogicalDevice>) -> Self {
        vxunimplemented!();
    }

    pub fn new(data: Vec<u8>, logical_device: Arc<LogicalDevice>) -> Self {
        let mut module_create_info = vk::VkShaderModuleCreateInfo::default();
        module_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
        module_create_info.codeSize = data.len();
        module_create_info.pCode = unsafe { transmute(data.as_ptr()) };
        let mut shader_module = 0 as vk::VkShaderModule;
        vulkan_check!(vk::vkCreateShaderModule(
            logical_device.vk_data,
            &module_create_info,
            null(),
            &mut shader_module,
        ));
        Module {
            logical_device: logical_device,
            module: shader_module,
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyShaderModule(self.logical_device.vk_data, self.module, null());
        }
    }
}
