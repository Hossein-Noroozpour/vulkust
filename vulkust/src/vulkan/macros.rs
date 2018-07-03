#[macro_export]
macro_rules! vulkan_check {
    ($x:expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::vulkan::vulkan as vk;
            match $x {
                vk::VkResult::VK_SUCCESS => {},
                c => {
                    vxlogf!("{:?}, value is: {}", c, c as i32);
                }
            }
        }
    };
}
