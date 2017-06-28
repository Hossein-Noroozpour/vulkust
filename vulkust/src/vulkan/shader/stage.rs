use std::mem::transmute;
use std::sync::Arc;
use std::ptr::null;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::vulkan as vk;
use super::super::super::system::os::OsApplication;
use super::super::device::logical::Logical as LogicalDevice;

pub struct Stage {
    pub logical_device: Arc<LogicalDevice>,
    pub module: vk::VkShaderModule,
}

impl Stage {
    pub fn new<CoreApp>(data: Vec<u8>, os_app: *mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let logical_device = unsafe {
            (*(*os_app).render_engine)
                .logical_device
                .as_ref()
                .unwrap()
                .clone()
        };
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
        Stage {
            logical_device: logical_device,
            module: shader_module,
        }
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyShaderModule(self.logical_device.vk_data, self.module, null());
        }
    }
}
