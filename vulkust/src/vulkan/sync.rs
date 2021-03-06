use super::device::Logical as LogicalDevice;
use ash::version::DeviceV1_0;
use ash::vk;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Semaphore {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::Semaphore,
}

impl Semaphore {
    pub(super) fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
        let vk_data = vx_result!(unsafe {
            logical_device
                .get_data()
                .create_semaphore(&semaphore_create_info, None)
        });
        Self {
            logical_device,
            vk_data,
        }
    }

    pub(super) fn get_data(&self) -> &vk::Semaphore {
        return &self.vk_data;
    }
}
impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .get_data()
                .destroy_semaphore(self.vk_data, None);
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Fence {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::Fence,
}

impl Fence {
    // pub(super) fn new(logical_device: Arc<LogicalDevice>) -> Self {
    //     let fence_create_info = vk::FenceCreateInfo::default();
    //     let vk_data = vx_result!(unsafe {
    //         logical_device
    //             .get_data()
    //             .create_fence(&fence_create_info, None)
    //     });
    //     Self {
    //         logical_device,
    //         vk_data,
    //     }
    // }

    pub(super) fn new_signaled(logical_device: Arc<LogicalDevice>) -> Self {
        let fence_create_info =
            vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
        let vk_data = vx_result!(unsafe {
            logical_device
                .get_data()
                .create_fence(&fence_create_info, None)
        });
        Self {
            logical_device,
            vk_data,
        }
    }

    pub(crate) fn wait(&self) {
        vx_result!(unsafe {
            self.logical_device
                .get_data()
                .wait_for_fences(&[self.vk_data], true, u64::max_value())
        });
    }

    pub(crate) fn reset(&self) {
        vx_result!(unsafe { self.logical_device.get_data().reset_fences(&[self.vk_data]) });
    }

    pub(super) fn get_data(&self) -> &vk::Fence {
        return &self.vk_data;
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .get_data()
                .destroy_fence(self.vk_data, None);
        }
    }
}
