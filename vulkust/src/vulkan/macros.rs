#[macro_export]
macro_rules! vulkan_check {
    ( $x:expr ) => {
        unsafe {
            match $x {
                VkResult::VK_SUCCESS => {},
				VkResult::VK_NOT_READY => {
                    loge!("VK_NOT_READY");
                },
			    VkResult::VK_TIMEOUT => {
                    loge!("VK_TIMEOUT");
                },
			    VkResult::VK_EVENT_SET => {
                    loge!("VK_EVENT_SET");
                },
			    VkResult::VK_EVENT_RESET => {
                    loge!("VK_EVENT_RESET");
                },
			    VkResult::VK_INCOMPLETE => {
                    loge!("VK_INCOMPLETE");
                },
			    VkResult::VK_ERROR_OUT_OF_HOST_MEMORY => {
                    loge!("VK_ERROR_OUT_OF_HOST_MEMORY");
                },
			    VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY => {
                    loge!("VK_ERROR_OUT_OF_DEVICE_MEMORY");
                },
			    VkResult::VK_ERROR_INITIALIZATION_FAILED => {
                    loge!("VK_ERROR_INITIALIZATION_FAILED");
                },
			    VkResult::VK_ERROR_DEVICE_LOST => {
                    loge!("VK_ERROR_DEVICE_LOST");
                },
			    VkResult::VK_ERROR_MEMORY_MAP_FAILED => {
                    loge!("VK_ERROR_MEMORY_MAP_FAILED");
                },
			    VkResult::VK_ERROR_LAYER_NOT_PRESENT => {
                    loge!("VK_ERROR_LAYER_NOT_PRESENT");
                },
			    VkResult::VK_ERROR_EXTENSION_NOT_PRESENT => {
                    loge!("VK_ERROR_EXTENSION_NOT_PRESENT");
                },
			    VkResult::VK_ERROR_FEATURE_NOT_PRESENT => {
                    loge!("VK_ERROR_FEATURE_NOT_PRESENT");
                },
			    VkResult::VK_ERROR_INCOMPATIBLE_DRIVER => {
                    loge!("VK_ERROR_INCOMPATIBLE_DRIVER");
                },
			    VkResult::VK_ERROR_TOO_MANY_OBJECTS => {
                    loge!("VK_ERROR_TOO_MANY_OBJECTS");
                },
			    VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED => {
                    loge!("VK_ERROR_FORMAT_NOT_SUPPORTED");
                },
			    VkResult::VK_ERROR_SURFACE_LOST_KHR => {
                    loge!("VK_ERROR_SURFACE_LOST_KHR");
                },
			    VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => {
                    loge!("VK_ERROR_NATIVE_WINDOW_IN_USE_KHR");
                },
			    VkResult::VK_SUBOPTIMAL_KHR => {
                    loge!("VK_SUBOPTIMAL_KHR");
                },
			    VkResult::VK_ERROR_OUT_OF_DATE_KHR => {
                    loge!("VK_ERROR_OUT_OF_DATE_KHR");
                },
			    VkResult::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => {
                    loge!("VK_ERROR_INCOMPATIBLE_DISPLAY_KHR");
                },
			    VkResult::VK_ERROR_VALIDATION_FAILED_EXT => {
                    loge!("VK_ERROR_VALIDATION_FAILED_EXT");
                },
			    VkResult::VK_ERROR_INVALID_SHADER_NV => {
                    loge!("VK_ERROR_INVALID_SHADER_NV");
                },
			    VkResult::VK_RESULT_RANGE_SIZE => {
                    loge!("VK_RESULT_RANGE_SIZE");
                },
				_ => {
                    loge!("Unknown error");
                },
            }
        }
    };
}
