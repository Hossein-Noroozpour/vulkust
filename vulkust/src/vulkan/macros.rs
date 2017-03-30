#[macro_export]
macro_rules! vulkan_check {
    ( $x:expr ) => {
        unsafe {
            match $x {
                vk::VkResult::VK_SUCCESS => {},
				vk::VkResult::VK_NOT_READY => {
                    logf!("VK_NOT_READY");
                },
			    vk::VkResult::VK_TIMEOUT => {
                    logf!("VK_TIMEOUT");
                },
			    vk::VkResult::VK_EVENT_SET => {
                    logf!("VK_EVENT_SET");
                },
			    vk::VkResult::VK_EVENT_RESET => {
                    logf!("VK_EVENT_RESET");
                },
			    vk::VkResult::VK_INCOMPLETE => {
                    logf!("VK_INCOMPLETE");
                },
			    vk::VkResult::VK_ERROR_OUT_OF_HOST_MEMORY => {
                    logf!("VK_ERROR_OUT_OF_HOST_MEMORY");
                },
			    vk::VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY => {
                    logf!("VK_ERROR_OUT_OF_DEVICE_MEMORY");
                },
			    vk::VkResult::VK_ERROR_INITIALIZATION_FAILED => {
                    logf!("VK_ERROR_INITIALIZATION_FAILED");
                },
			    vk::VkResult::VK_ERROR_DEVICE_LOST => {
                    logf!("VK_ERROR_DEVICE_LOST");
                },
			    vk::VkResult::VK_ERROR_MEMORY_MAP_FAILED => {
                    logf!("VK_ERROR_MEMORY_MAP_FAILED");
                },
			    vk::VkResult::VK_ERROR_LAYER_NOT_PRESENT => {
                    logf!("VK_ERROR_LAYER_NOT_PRESENT");
                },
			    vk::VkResult::VK_ERROR_EXTENSION_NOT_PRESENT => {
                    logf!("VK_ERROR_EXTENSION_NOT_PRESENT");
                },
			    vk::VkResult::VK_ERROR_FEATURE_NOT_PRESENT => {
                    logf!("VK_ERROR_FEATURE_NOT_PRESENT");
                },
			    vk::VkResult::VK_ERROR_INCOMPATIBLE_DRIVER => {
                    logf!("VK_ERROR_INCOMPATIBLE_DRIVER");
                },
			    vk::VkResult::VK_ERROR_TOO_MANY_OBJECTS => {
                    logf!("VK_ERROR_TOO_MANY_OBJECTS");
                },
			    vk::VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED => {
                    logf!("VK_ERROR_FORMAT_NOT_SUPPORTED");
                },
			    vk::VkResult::VK_ERROR_SURFACE_LOST_KHR => {
                    logf!("VK_ERROR_SURFACE_LOST_KHR");
                },
			    vk::VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => {
                    logf!("VK_ERROR_NATIVE_WINDOW_IN_USE_KHR");
                },
			    vk::VkResult::VK_SUBOPTIMAL_KHR => {
                    logf!("VK_SUBOPTIMAL_KHR");
                },
			    vk::VkResult::VK_ERROR_OUT_OF_DATE_KHR => {
                    logf!("VK_ERROR_OUT_OF_DATE_KHR");
                },
			    vk::VkResult::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => {
                    logf!("VK_ERROR_INCOMPATIBLE_DISPLAY_KHR");
                },
			    vk::VkResult::VK_ERROR_VALIDATION_FAILED_EXT => {
                    logf!("VK_ERROR_VALIDATION_FAILED_EXT");
                },
			    vk::VkResult::VK_ERROR_INVALID_SHADER_NV => {
                    logf!("VK_ERROR_INVALID_SHADER_NV");
                },
			    vk::VkResult::VK_RESULT_RANGE_SIZE => {
                    logf!("VK_RESULT_RANGE_SIZE");
                },
				_ => {
                    logf!("Unknown error");
                },
            }
        }
    };
}
