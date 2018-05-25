use super::vulkan as vk;
use super::swapchain::Swapchain;
use std::sync::Arc;
use std::ptr::null;

pub struct RenderPass {
    pub swapchain: Arc<Swapchain>,
    pub vk_data: vk::VkRenderPass,
}

impl RenderPass {
    pub fn new(swapchain: &Arc<Swapchain>) -> Self {
        let mut attachment_descriptions = vec![vk::VkAttachmentDescription::default(); 2];
        attachment_descriptions[0].format = swapchain.surface_format.format;
        attachment_descriptions[0].samples = vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        attachment_descriptions[0].loadOp = vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR;
        attachment_descriptions[0].storeOp = vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE;
        attachment_descriptions[0].stencilLoadOp =
            vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE;
        attachment_descriptions[0].stencilStoreOp =
            vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE;
        attachment_descriptions[0].initialLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        attachment_descriptions[0].finalLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        attachment_descriptions[1].format = swapchain
            .logical_device
            .physical_device
            .get_supported_depth_format();
        attachment_descriptions[1].samples = vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        attachment_descriptions[1].loadOp = vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR;
        attachment_descriptions[1].storeOp =
            vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE;
        attachment_descriptions[1].stencilLoadOp =
            vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE;
        attachment_descriptions[1].stencilStoreOp =
            vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE;
        attachment_descriptions[1].initialLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        attachment_descriptions[1].finalLayout =
            vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
        let mut colour_reference = vk::VkAttachmentReference::default();
        colour_reference.layout = vk::VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        colour_reference.attachment = 0;
        let mut depth_reference = vk::VkAttachmentReference::default();
        depth_reference.layout =
            vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
        depth_reference.attachment = 1;
        let mut subpass_description = vk::VkSubpassDescription::default();
        subpass_description.pipelineBindPoint =
            vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS;
        subpass_description.colorAttachmentCount = 1;
        subpass_description.pColorAttachments = &colour_reference;
        subpass_description.pDepthStencilAttachment = &depth_reference;
        let mut dependencies = vec![vk::VkSubpassDependency::default(); 2];
        dependencies[0].srcSubpass = vk::VK_SUBPASS_EXTERNAL;
        dependencies[0].srcStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as u32;
        dependencies[0].dstStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        dependencies[0].srcAccessMask = vk::VkAccessFlagBits::VK_ACCESS_MEMORY_READ_BIT as u32;
        dependencies[0].dstAccessMask = vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as
            u32 |
            vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32;
        dependencies[0].dependencyFlags =
            vk::VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as u32;
        dependencies[1].dstSubpass = vk::VK_SUBPASS_EXTERNAL;
        dependencies[1].srcStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        dependencies[1].dstStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as u32;
        dependencies[1].srcAccessMask = vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as
            u32 |
            vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32;
        dependencies[1].dstAccessMask = vk::VkAccessFlagBits::VK_ACCESS_MEMORY_READ_BIT as u32;
        dependencies[1].dependencyFlags =
            vk::VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as u32;
        let mut render_pass_create_info = vk::VkRenderPassCreateInfo::default();
        render_pass_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
        render_pass_create_info.attachmentCount = attachment_descriptions.len() as u32;
        render_pass_create_info.pAttachments = attachment_descriptions.as_ptr();
        render_pass_create_info.subpassCount = 1;
        render_pass_create_info.pSubpasses = &subpass_description;
        render_pass_create_info.dependencyCount = dependencies.len() as u32;
        render_pass_create_info.pDependencies = dependencies.as_ptr();
        let mut vk_data = 0 as vk::VkRenderPass;
        vulkan_check!(vk::vkCreateRenderPass(
            swapchain.logical_device.vk_data,
            &render_pass_create_info,
            null(),
            &mut vk_data,
        ));
        RenderPass {
            swapchain: swapchain.clone(),
            vk_data,
        }
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyRenderPass(self.swapchain.logical_device.vk_data, self.vk_data, null());
        }
    }
}
