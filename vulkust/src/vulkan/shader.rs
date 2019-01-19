// use super::super::system::file::File;
use super::device::Logical as LogicalDevice;
use ash::version::DeviceV1_0;
use ash::vk;
// use std::io::Read;
use std::mem::transmute;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Module {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::ShaderModule,
}

impl Module {
    // pub(super) fn new_with_file(file_name: &str, logical_device: Arc<LogicalDevice>) -> Self {
    //     let mut file = vxresult!(File::open(file_name));
    //     let mut data = Vec::new();
    //     let _ = vxresult!(file.read_to_end(&mut data));
    //     Self::new(&data, logical_device)
    // }

    pub(super) fn new(data: &[u8], logical_device: Arc<LogicalDevice>) -> Self {
        let mut module_create_info = vk::ShaderModuleCreateInfo::default();
        module_create_info.code_size = data.len();
        module_create_info.p_code = unsafe { transmute(data.as_ptr()) };
        let vk_data = vxresult!(unsafe {
            logical_device
                .get_data()
                .create_shader_module(&module_create_info, None)
        });
        Self {
            logical_device,
            vk_data,
        }
    }

    pub(super) fn get_data(&self) -> &vk::ShaderModule {
        return &self.vk_data;
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .get_data()
                .destroy_shader_module(self.vk_data, None);
        }
    }
}
