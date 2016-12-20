use super::super::system::vulkan::{
    VkImage,
    VkResult,
    VkImageView,
    VkRenderPass,
    VkImageLayout,
    VkSharingMode,
    VkFramebuffer,
    VkSwapchainKHR,
    VkImageViewType,
    VkStructureType,
    VkPipelineCache,
    VkAccessFlagBits,
    VkPresentModeKHR,
    vkCreateImageView,
    vkCreateRenderPass,
    vkDestroyImageView,
    VkImageAspectFlags,
    VkComponentSwizzle,
    VkComponentMapping,
    VkAttachmentLoadOp,
    VkAttachmentStoreOp,
    vkCreateFramebuffer,
    vkDestroyRenderPass,
    VkPipelineBindPoint,
    VkSubpassDependency,
    VkDependencyFlagBits,
    vkCreateSwapchainKHR,
    VkImageUsageFlagBits,
    VkSubpassDescription,
    vkDestroyFramebuffer,
    VkAttachmentReference,
    VkSampleCountFlagBits,
    VkImageAspectFlagBits,
    VkImageViewCreateInfo,
    vkDestroySwapchainKHR,
    VkAllocationCallbacks,
    vkCreatePipelineCache,
    vkDestroyPipelineCache,
    VkRenderPassCreateInfo,
    VkFramebufferCreateInfo,
    VkPipelineStageFlagBits,
    VkImageSubresourceRange,
    VkAttachmentDescription,
    vkGetSwapchainImagesKHR,
    VkSwapchainCreateInfoKHR,
    VkPipelineCacheCreateInfo,
    VkCompositeAlphaFlagBitsKHR,
    VkSurfaceTransformFlagBitsKHR,

    VK_SUBPASS_EXTERNAL
};

use super::window::Window;
use super::image::view::View as ImageView;
use super::image::Image;

use std::sync::{
    Arc,
    RwLock,
};

pub struct Swapchain {
    window: Arc<Window>,
    depth_stencil_image_view: ImageView,
    vk_image_views: Vec<VkImageView>,
    vk_swapchain: VkSwapchainKHR,
    vk_render_pass: VkRenderPass,
    vk_pipeline_cache: VkPipelineCache,
    vk_frame_buffers: Vec<VkFramebuffer>,
}

impl Swapchain {
    pub fn new(window: Arc<Window>) -> Self {
        if win.vk_surface_capabilities.maxImageCount < 2 {
            panic!("Vulkan driver does not support 2 double image buffering.");
        }
        let pre_transform = if (win.vk_surface_capabilities.supportedTransforms as u32) &
            (VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR as u32) != 0 {
            VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR
        } else {
            win.vk_surface_capabilities.currentTransform
        };

        let swapchain_ci = VkSwapchainCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            surface: win.vk_surface,
            minImageCount: 2,
            imageFormat: win.vk_surface_format.format,
            imageColorSpace: win.vk_surface_format.colorSpace,
            imageExtent: win.vk_surface_capabilities.currentExtent,
            imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32,
            preTransform: pre_transform,
            imageArrayLayers: 1,
            imageSharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            presentMode: VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR,
            clipped: 1u32,
            compositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            ..VkSwapchainCreateInfoKHR::default()
        };
        let mut vk_swapchain = 0 as VkSwapchainKHR;
        vulkan_check!(vkCreateSwapchainKHR(
            dev.vk_device, &swapchain_ci as *const VkSwapchainCreateInfoKHR,
            0 as *const VkAllocationCallbacks, &mut vk_swapchain as *mut VkSwapchainKHR));
        let mut image_count = 0u32;
        vulkan_check!(vkGetSwapchainImagesKHR(
            dev.vk_device, vk_swapchain, &mut image_count as *mut u32, 0 as *mut VkImage));
        let mut vk_images = vec![0 as VkImage; image_count as usize];
        vulkan_check!(vkGetSwapchainImagesKHR(
            dev.vk_device, vk_swapchain, &mut image_count as *mut u32,
            vk_images.as_mut_ptr() as *mut VkImage));
        let mut vk_image_views = vec![0 as VkImageView; image_count as usize];
        for i in 0..image_count {
            let mut color_attachment_view = VkImageViewCreateInfo::default();
            color_attachment_view.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
            color_attachment_view.image = vk_images[i as usize];
            color_attachment_view.viewType = VkImageViewType::VK_IMAGE_VIEW_TYPE_2D;
            color_attachment_view.format = win.vk_surface_format.format;
            color_attachment_view.components = VkComponentMapping {
                r: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_R,
                g: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_G,
                b: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_B,
                a: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_A,
            };
            color_attachment_view.subresourceRange = VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            let ptr_vk_image_views = vk_image_views.as_mut_ptr();
            vulkan_check!(vkCreateImageView(
                dev.vk_device, &color_attachment_view as *const VkImageViewCreateInfo,
                0 as *const VkAllocationCallbacks, ptr_vk_image_views.offset(i as isize)));
        }
        let depth_stencil_image_view = ImageView::new_depth_stencil(Arc::new(RwLock::new(
            Image::new_depth_with_format(
                win.device.clone(),
                win.vk_surface_capabilities.currentExtent.width,
                win.vk_surface_capabilities.currentExtent.height
            )
        )));
        let mut attachments = [VkAttachmentDescription::default(); 2];
        attachments[0].format = win.vk_surface_format.format;
        attachments[0].samples = VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        attachments[0].loadOp = VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR;
        attachments[0].storeOp = VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE;
        attachments[0].stencilLoadOp = VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE;
        attachments[0].stencilStoreOp = VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE;
        attachments[0].initialLayout = VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        attachments[0].finalLayout = VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        attachments[1].format = dev.vk_depth_format;
        attachments[1].samples = VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        attachments[1].loadOp = VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR;
        attachments[1].storeOp = VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE;
        attachments[1].stencilLoadOp = VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE;
        attachments[1].stencilStoreOp = VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE;
        attachments[1].initialLayout = VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        attachments[1].finalLayout = VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
        let mut color_reference = VkAttachmentReference::default();
        color_reference.layout = VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        let mut depth_reference = VkAttachmentReference::default();
        depth_reference.layout = VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
        let mut subpass_description = VkSubpassDescription::default();
        subpass_description.pipelineBindPoint = VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS;
        subpass_description.colorAttachmentCount = 1;
        subpass_description.pColorAttachments = &color_reference as *const VkAttachmentReference;
        subpass_description.pDepthStencilAttachment = &depth_reference as *const VkAttachmentReference;
        let mut dependencies = [VkSubpassDependency::default(); 2];
        dependencies[0].srcSubpass = VK_SUBPASS_EXTERNAL;
        dependencies[0].srcStageMask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as u32;
        dependencies[0].dstStageMask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        dependencies[0].srcAccessMask = VkAccessFlagBits::VK_ACCESS_MEMORY_READ_BIT as u32;
        dependencies[0].dstAccessMask = VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as u32 | VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32;
        dependencies[0].dependencyFlags = VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as u32;
        dependencies[1].dstSubpass = VK_SUBPASS_EXTERNAL;
        dependencies[1].srcStageMask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        dependencies[1].dstStageMask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as u32;
        dependencies[1].srcAccessMask = VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as u32 | VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32;
        dependencies[1].dstAccessMask = VkAccessFlagBits::VK_ACCESS_MEMORY_READ_BIT as u32;
        dependencies[1].dependencyFlags = VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as u32;
        let mut render_pass_info = VkRenderPassCreateInfo::default();
        render_pass_info.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
        render_pass_info.attachmentCount = attachments.len() as u32;
        render_pass_info.pAttachments = attachments.as_ptr() as *const VkAttachmentDescription;
        render_pass_info.subpassCount = 1;
        render_pass_info.pSubpasses = &subpass_description as *const VkSubpassDescription;
        render_pass_info.dependencyCount = dependencies.len() as u32;
        render_pass_info.pDependencies = dependencies.as_ptr() as *const VkSubpassDependency;
        let mut vk_render_pass = 0 as VkRenderPass;
        vulkan_check!(vkCreateRenderPass(
            dev.vk_device, &render_pass_info as *const VkRenderPassCreateInfo,
            0 as *const VkAllocationCallbacks, &mut vk_render_pass as *mut VkRenderPass));
        let mut pipeline_cache_create_info = VkPipelineCacheCreateInfo::default();
        pipeline_cache_create_info.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO;
        let mut vk_pipeline_cache = 0 as VkPipelineCache;
        vulkan_check!(vkCreatePipelineCache(
            dev.vk_device, &pipeline_cache_create_info as *const VkPipelineCacheCreateInfo,
            0 as *const VkAllocationCallbacks, &mut vk_pipeline_cache as *mut VkPipelineCache));
        let mut attachments = [0 as VkImageView; 2];
        attachments[1] = depth_stencil_image_view.vk_view;
        let mut frame_buffer_create_info = VkFramebufferCreateInfo::default();
        frame_buffer_create_info.sType = VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        frame_buffer_create_info.renderPass = vk_render_pass;
        frame_buffer_create_info.attachmentCount = 2;
        frame_buffer_create_info.pAttachments = attachments.as_ptr() as *const VkImageView;
        frame_buffer_create_info.width = win.vk_surface_capabilities.currentExtent.width;
        frame_buffer_create_info.height = win.vk_surface_capabilities.currentExtent.height;
        frame_buffer_create_info.layers = 1;
        let mut vk_frame_buffers = vec![0 as VkFramebuffer; vk_image_views.len()];
        for i in 0..vk_frame_buffers.len() {
            attachments[0] = vk_image_views[i];
            let mut ptr_frame_buffer = unsafe {vk_frame_buffers.as_mut_ptr().offset(i as isize)};
            vulkan_check!(vkCreateFramebuffer(
                dev.vk_device, &frame_buffer_create_info as *const VkFramebufferCreateInfo,
                0 as *const VkAllocationCallbacks, ptr_frame_buffer));
        }
        Swapchain {
            window: window.clone(),
            depth_stencil_image_view: depth_stencil_image_view,
            vk_image_views: vk_image_views,
            vk_swapchain: vk_swapchain,
            vk_render_pass: vk_render_pass,
            vk_pipeline_cache: vk_pipeline_cache,
            vk_frame_buffers: vk_frame_buffers,
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        for i in 0..self.vk_frame_buffers.len() {
            unsafe {
                vkDestroyFramebuffer(dev.vk_device, self.vk_frame_buffers[i], 0 as *const VkAllocationCallbacks);
            }
        }
        unsafe {
            vkDestroyPipelineCache(dev.vk_device, self.vk_pipeline_cache, 0 as *const VkAllocationCallbacks);
        }
        unsafe {
            vkDestroyRenderPass(
                dev.vk_device, self.vk_render_pass, 0 as *const VkAllocationCallbacks);
        }
        for i in 0..self.vk_image_views.len() {
            unsafe {
                vkDestroyImageView(dev.vk_device, self.vk_image_views[i], 0 as *const VkAllocationCallbacks);
            }
        }
        unsafe {
            vkDestroySwapchainKHR(
                dev.vk_device, self.vk_swapchain, 0 as *const VkAllocationCallbacks);
        }
    }
}