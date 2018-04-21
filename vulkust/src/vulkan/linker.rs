use super::super::system::linker::Linker as SysLinker;
use super::vulkan::*;
// macro_rules! vkl {
//     ($()$) => {};
// }

pub struct Linker {
    library: SysLinker,
    // instance
    create_instance_ptr: PFN_vkCreateInstance,
    destroy_instance_ptr: PFN_vkDestroyInstance,
    // debug
    create_debug_report_callback_ext_ptr: PFN_vkCreateDebugReportCallbackEXT,
    destroy_debug_report_callback_ext_ptr: PFN_vkDestroyDebugReportCallbackEXT,
    debug_report_message_ext_ptr: PFN_vkDebugReportMessageEXT,
    // vkEnumeratePhysicalDevices: Option<PFN_vkEnumeratePhysicalDevices>,
    // vkGetPhysicalDeviceFeatures: Option<PFN_vkGetPhysicalDeviceFeatures>,
    // vkGetPhysicalDeviceFormatProperties: Option<PFN_vkGetPhysicalDeviceFormatProperties>,
    // vkGetPhysicalDeviceImageFormatProperties: Option<PFN_vkGetPhysicalDeviceImageFormatProperties>,
    // vkGetPhysicalDeviceProperties: Option<PFN_vkGetPhysicalDeviceProperties>,
    // vkGetPhysicalDeviceQueueFamilyProperties: Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
    // vkGetPhysicalDeviceMemoryProperties: Option<PFN_vkGetPhysicalDeviceMemoryProperties>,
    // vkGetInstanceProcAddr: Option<PFN_vkGetInstanceProcAddr>,
    // vkGetDeviceProcAddr: Option<PFN_vkGetDeviceProcAddr>,
    // vkCreateDevice: Option<PFN_vkCreateDevice>,
    // vkDestroyDevice: Option<PFN_vkDestroyDevice>,
    // vkEnumerateInstanceExtensionProperties: Option<PFN_vkEnumerateInstanceExtensionProperties>,
    // vkEnumerateDeviceExtensionProperties: Option<PFN_vkEnumerateDeviceExtensionProperties>,
    // vkEnumerateInstanceLayerProperties: Option<PFN_vkEnumerateInstanceLayerProperties>,
    // vkEnumerateDeviceLayerProperties: Option<PFN_vkEnumerateDeviceLayerProperties>,
    // vkGetDeviceQueue: Option<PFN_vkGetDeviceQueue>,
    // vkQueueSubmit: Option<PFN_vkQueueSubmit>,
    // vkQueueWaitIdle: Option<PFN_vkQueueWaitIdle>,
    // vkDeviceWaitIdle: Option<PFN_vkDeviceWaitIdle>,
    // vkAllocateMemory: Option<PFN_vkAllocateMemory>,
    // vkFreeMemory: Option<PFN_vkFreeMemory>,
    // vkMapMemory: Option<PFN_vkMapMemory>,
    // vkUnmapMemory: Option<PFN_vkUnmapMemory>,
    // vkFlushMappedMemoryRanges: Option<PFN_vkFlushMappedMemoryRanges>,
    // vkInvalidateMappedMemoryRanges: Option<PFN_vkInvalidateMappedMemoryRanges>,
    // vkGetDeviceMemoryCommitment: Option<PFN_vkGetDeviceMemoryCommitment>,
    // vkBindBufferMemory: Option<PFN_vkBindBufferMemory>,
    // vkBindImageMemory: Option<PFN_vkBindImageMemory>,
    // vkGetBufferMemoryRequirements: Option<PFN_vkGetBufferMemoryRequirements>,
    // vkGetImageMemoryRequirements: Option<PFN_vkGetImageMemoryRequirements>,
    // vkGetImageSparseMemoryRequirements: Option<PFN_vkGetImageSparseMemoryRequirements>,
    // vkGetPhysicalDeviceSparseImageFormatProperties:
    //     Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,
    // vkQueueBindSparse: Option<PFN_vkQueueBindSparse>,
    // vkCreateFence: Option<PFN_vkCreateFence>,
    // vkDestroyFence: Option<PFN_vkDestroyFence>,
    // vkResetFences: Option<PFN_vkResetFences>,
    // vkGetFenceStatus: Option<PFN_vkGetFenceStatus>,
    // vkWaitForFences: Option<PFN_vkWaitForFences>,
    // vkCreateSemaphore: Option<PFN_vkCreateSemaphore>,
    // vkDestroySemaphore: Option<PFN_vkDestroySemaphore>,
    // vkCreateEvent: Option<PFN_vkCreateEvent>,
    // vkDestroyEvent: Option<PFN_vkDestroyEvent>,
    // vkGetEventStatus: Option<PFN_vkGetEventStatus>,
    // vkSetEvent: Option<PFN_vkSetEvent>,
    // vkResetEvent: Option<PFN_vkResetEvent>,
    // vkCreateQueryPool: Option<PFN_vkCreateQueryPool>,
    // vkDestroyQueryPool: Option<PFN_vkDestroyQueryPool>,
    // vkGetQueryPoolResults: Option<PFN_vkGetQueryPoolResults>,
    // vkCreateBuffer: Option<PFN_vkCreateBuffer>,
    // vkDestroyBuffer: Option<PFN_vkDestroyBuffer>,
    // vkCreateBufferView: Option<PFN_vkCreateBufferView>,
    // vkDestroyBufferView: Option<PFN_vkDestroyBufferView>,
    // vkCreateImage: Option<PFN_vkCreateImage>,
    // vkDestroyImage: Option<PFN_vkDestroyImage>,
    // vkGetImageSubresourceLayout: Option<PFN_vkGetImageSubresourceLayout>,
    // vkCreateImageView: Option<PFN_vkCreateImageView>,
    // vkDestroyImageView: Option<PFN_vkDestroyImageView>,
    // vkCreateShaderModule: Option<PFN_vkCreateShaderModule>,
    // vkDestroyShaderModule: Option<PFN_vkDestroyShaderModule>,
    // vkCreatePipelineCache: Option<PFN_vkCreatePipelineCache>,
    // vkDestroyPipelineCache: Option<PFN_vkDestroyPipelineCache>,
    // vkGetPipelineCacheData: Option<PFN_vkGetPipelineCacheData>,
    // vkMergePipelineCaches: Option<PFN_vkMergePipelineCaches>,
    // vkCreateGraphicsPipelines: Option<PFN_vkCreateGraphicsPipelines>,
    // vkCreateComputePipelines: Option<PFN_vkCreateComputePipelines>,
    // vkDestroyPipeline: Option<PFN_vkDestroyPipeline>,
    // vkCreatePipelineLayout: Option<PFN_vkCreatePipelineLayout>,
    // vkDestroyPipelineLayout: Option<PFN_vkDestroyPipelineLayout>,
    // vkCreateSampler: Option<PFN_vkCreateSampler>,
    // vkDestroySampler: Option<PFN_vkDestroySampler>,
    // vkCreateDescriptorSetLayout: Option<PFN_vkCreateDescriptorSetLayout>,
    // vkDestroyDescriptorSetLayout: Option<PFN_vkDestroyDescriptorSetLayout>,
    // vkCreateDescriptorPool: Option<PFN_vkCreateDescriptorPool>,
    // vkDestroyDescriptorPool: Option<PFN_vkDestroyDescriptorPool>,
    // vkResetDescriptorPool: Option<PFN_vkResetDescriptorPool>,
    // vkAllocateDescriptorSets: Option<PFN_vkAllocateDescriptorSets>,
    // vkFreeDescriptorSets: Option<PFN_vkFreeDescriptorSets>,
    // vkUpdateDescriptorSets: Option<PFN_vkUpdateDescriptorSets>,
    // vkCreateFramebuffer: Option<PFN_vkCreateFramebuffer>,
    // vkDestroyFramebuffer: Option<PFN_vkDestroyFramebuffer>,
    // vkCreateRenderPass: Option<PFN_vkCreateRenderPass>,
    // vkDestroyRenderPass: Option<PFN_vkDestroyRenderPass>,
    // vkGetRenderAreaGranularity: Option<PFN_vkGetRenderAreaGranularity>,
    // vkCreateCommandPool: Option<PFN_vkCreateCommandPool>,
    // vkDestroyCommandPool: Option<PFN_vkDestroyCommandPool>,
    // vkResetCommandPool: Option<PFN_vkResetCommandPool>,
    // vkAllocateCommandBuffers: Option<PFN_vkAllocateCommandBuffers>,
    // vkFreeCommandBuffers: Option<PFN_vkFreeCommandBuffers>,
    // vkBeginCommandBuffer: Option<PFN_vkBeginCommandBuffer>,
    // vkEndCommandBuffer: Option<PFN_vkEndCommandBuffer>,
    // vkResetCommandBuffer: Option<PFN_vkResetCommandBuffer>,
    // vkCmdBindPipeline: Option<PFN_vkCmdBindPipeline>,
    // vkCmdSetViewport: Option<PFN_vkCmdSetViewport>,
    // vkCmdSetScissor: Option<PFN_vkCmdSetScissor>,
    // vkCmdSetLineWidth: Option<PFN_vkCmdSetLineWidth>,
    // vkCmdSetDepthBias: Option<PFN_vkCmdSetDepthBias>,
    // vkCmdSetBlendConstants: Option<PFN_vkCmdSetBlendConstants>,
    // vkCmdSetDepthBounds: Option<PFN_vkCmdSetDepthBounds>,
    // vkCmdSetStencilCompareMask: Option<PFN_vkCmdSetStencilCompareMask>,
    // vkCmdSetStencilWriteMask: Option<PFN_vkCmdSetStencilWriteMask>,
    // vkCmdSetStencilReference: Option<PFN_vkCmdSetStencilReference>,
    // vkCmdBindDescriptorSets: Option<PFN_vkCmdBindDescriptorSets>,
    // vkCmdBindIndexBuffer: Option<PFN_vkCmdBindIndexBuffer>,
    // vkCmdBindVertexBuffers: Option<PFN_vkCmdBindVertexBuffers>,
    // vkCmdDraw: Option<PFN_vkCmdDraw>,
    // vkCmdDrawIndexed: Option<PFN_vkCmdDrawIndexed>,
    // vkCmdDrawIndirect: Option<PFN_vkCmdDrawIndirect>,
    // vkCmdDrawIndexedIndirect: Option<PFN_vkCmdDrawIndexedIndirect>,
    // vkCmdDispatch: Option<PFN_vkCmdDispatch>,
    // vkCmdDispatchIndirect: Option<PFN_vkCmdDispatchIndirect>,
    // vkCmdCopyBuffer: Option<PFN_vkCmdCopyBuffer>,
    // vkCmdCopyImage: Option<PFN_vkCmdCopyImage>,
    // vkCmdBlitImage: Option<PFN_vkCmdBlitImage>,
    // vkCmdCopyBufferToImage: Option<PFN_vkCmdCopyBufferToImage>,
    // vkCmdCopyImageToBuffer: Option<PFN_vkCmdCopyImageToBuffer>,
    // vkCmdUpdateBuffer: Option<PFN_vkCmdUpdateBuffer>,
    // vkCmdFillBuffer: Option<PFN_vkCmdFillBuffer>,
    // vkCmdClearColorImage: Option<PFN_vkCmdClearColorImage>,
    // vkCmdClearDepthStencilImage: Option<PFN_vkCmdClearDepthStencilImage>,
    // vkCmdClearAttachments: Option<PFN_vkCmdClearAttachments>,
    // vkCmdResolveImage: Option<PFN_vkCmdResolveImage>,
    // vkCmdSetEvent: Option<PFN_vkCmdSetEvent>,
    // vkCmdResetEvent: Option<PFN_vkCmdResetEvent>,
    // vkCmdWaitEvents: Option<PFN_vkCmdWaitEvents>,
    // vkCmdPipelineBarrier: Option<PFN_vkCmdPipelineBarrier>,
    // vkCmdBeginQuery: Option<PFN_vkCmdBeginQuery>,
    // vkCmdEndQuery: Option<PFN_vkCmdEndQuery>,
    // vkCmdResetQueryPool: Option<PFN_vkCmdResetQueryPool>,
    // vkCmdWriteTimestamp: Option<PFN_vkCmdWriteTimestamp>,
    // vkCmdCopyQueryPoolResults: Option<PFN_vkCmdCopyQueryPoolResults>,
    // vkCmdPushConstants: Option<PFN_vkCmdPushConstants>,
    // vkCmdBeginRenderPass: Option<PFN_vkCmdBeginRenderPass>,
    // vkCmdNextSubpass: Option<PFN_vkCmdNextSubpass>,
    // vkCmdEndRenderPass: Option<PFN_vkCmdEndRenderPass>,
    // vkCmdExecuteCommands: Option<PFN_vkCmdExecuteCommands>,
    // vkDestroySurfaceKHR: Option<PFN_vkDestroySurfaceKHR>,
    // vkGetPhysicalDeviceSurfaceSupportKHR: Option<PFN_vkGetPhysicalDeviceSurfaceSupportKHR>,
    // vkGetPhysicalDeviceSurfaceCapabilitiesKHR:
    //     Option<PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    // vkGetPhysicalDeviceSurfaceFormatsKHR: Option<PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
    // vkGetPhysicalDeviceSurfacePresentModesKHR:
    //     Option<PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
    // vkCreateSwapchainKHR: Option<PFN_vkCreateSwapchainKHR>,
    // vkDestroySwapchainKHR: Option<PFN_vkDestroySwapchainKHR>,
    // vkGetSwapchainImagesKHR: Option<PFN_vkGetSwapchainImagesKHR>,
    // vkAcquireNextImageKHR: Option<PFN_vkAcquireNextImageKHR>,
    // vkQueuePresentKHR: Option<PFN_vkQueuePresentKHR>,
    // vkGetPhysicalDeviceDisplayPropertiesKHR: Option<PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>,
    // vkGetPhysicalDeviceDisplayPlanePropertiesKHR:
    //     Option<PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>,
    // vkGetDisplayPlaneSupportedDisplaysKHR: Option<PFN_vkGetDisplayPlaneSupportedDisplaysKHR>,
    // vkGetDisplayModePropertiesKHR: Option<PFN_vkGetDisplayModePropertiesKHR>,
    // vkCreateDisplayModeKHR: Option<PFN_vkCreateDisplayModeKHR>,
    // vkGetDisplayPlaneCapabilitiesKHR: Option<PFN_vkGetDisplayPlaneCapabilitiesKHR>,
    // vkCreateDisplayPlaneSurfaceKHR: Option<PFN_vkCreateDisplayPlaneSurfaceKHR>,
    // vkCreateSharedSwapchainsKHR: Option<PFN_vkCreateSharedSwapchainsKHR>,
}

impl Linker {
    pub fn new() -> Self {
        let library = SysLinker::new("libvulkan.so");
        macro_rules! vxlink (
            ($name:ident) => (
                vxunwrap!(library.get_function(stringify!($name)))
            )
        );
        if !library.is_ok() {
            vxlogf!("Vulkan shared library (dll) not found.");
        }
        let create_instance_ptr = vxlink!(vkCreateInstance);
        let destroy_instance_ptr = vxlink!(vkDestroyInstance);
        let create_debug_report_callback_ext_ptr = vxlink!(vkCreateDebugReportCallbackEXT);
        let destroy_debug_report_callback_ext_ptr = vxlink!(vkDestroyDebugReportCallbackEXT);
        let debug_report_message_ext_ptr = vxlink!(vkDebugReportMessageEXT);
        Linker {
            library,
            create_instance_ptr,
            destroy_instance_ptr,
            create_debug_report_callback_ext_ptr,
            destroy_debug_report_callback_ext_ptr,
            debug_report_message_ext_ptr,
        }
    }
    
    pub fn create_instance(
        &self, 
        p_create_info: *const VkInstanceCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_instance: *mut VkInstance,
    ) -> VkResult {
        unsafe {
            (self.create_instance_ptr)(
                p_create_info,
                p_allocator,
                p_instance)
        }
    }
    
    pub fn destroy_instance(&self, instance: VkInstance, p_allocator: *const VkAllocationCallbacks) {
        unsafe {
            (self.destroy_instance_ptr)(instance, p_allocator)
        }
    }
    
    pub fn create_debug_report_callback_ext(
        &self, 
        instance: VkInstance,
        p_create_info: *const VkDebugReportCallbackCreateInfoEXT,
        p_allocator: *const VkAllocationCallbacks,
        p_callback: *mut VkDebugReportCallbackEXT,
    ) -> VkResult {
        unsafe {
            (self.create_debug_report_callback_ext_ptr)(instance, p_create_info, p_allocator, p_callback)
        }
    }
    
    pub fn destroy_debug_report_callback_ext(
        &self,
        instance: VkInstance,
        callback: VkDebugReportCallbackEXT,
        p_allocator: *const VkAllocationCallbacks) {
        unsafe {
            (self.destroy_debug_report_callback_ext_ptr)(instance, callback, p_allocator)
        }
    }

    // pub fn debug_report_message_ext(&self) -> vk::VkResult {}
}
