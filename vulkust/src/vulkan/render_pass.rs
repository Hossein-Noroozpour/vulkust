use super::image::View as ImageView;
use super::swapchain::Swapchain;
use super::vulkan as vk;
use std::ptr::null;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct RenderPass {
    swapchain: Option<Arc<Swapchain>>,
    colors: Vec<Arc<ImageView>>,
    depth: Option<Arc<ImageView>>,
    pub vk_data: vk::VkRenderPass,
}

impl RenderPass {
    pub fn new_with_swapchain(swapchain: Arc<Swapchain>, clear: bool) -> Self {
        let vs = vec![swapchain.image_views[0].clone()];
        let mut result = Self::new(vs, clear, false);
        result.swapchain = Some(swapchain);
        return result;
    }

    pub fn new(views: Vec<Arc<ImageView>>, clear: bool, has_reader: bool) -> Self {
        let mut attachment_descriptions = Vec::new(); // vec![vk::VkAttachmentDescription::default(); views_len];
        let mut color_attachments_refs = Vec::new(); // vec![vk::VkAttachmentReference::default(); views_len - 1];
        let mut depth_attachment_ref = vk::VkAttachmentReference::default();
        let mut vkdev = 0 as vk::VkDevice;
        let mut depth = None;
        let mut colors = Vec::new();
        for v in &views {
            let img = vxresult!(v.image.read());
            vkdev = img.logical_device.vk_data;

            let mut attachment_description = vk::VkAttachmentDescription::default();
            attachment_description.format = img.format;
            attachment_description.samples = img.samples;
            attachment_description.loadOp = if clear {
                vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR
            } else {
                vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_LOAD
            };
            attachment_description.storeOp =
                vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE;
            attachment_description.stencilLoadOp =
                vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE;
            attachment_description.stencilStoreOp =
                vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE;
            attachment_description.initialLayout = if clear {
                vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED
            } else {
                vk::VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR
            };
            if img.usage & vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT
                as vk::VkImageUsageFlags
                != 0
            {
                attachment_description.finalLayout = if has_reader {
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
                } else {
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR
                };
                let mut color_attachment_ref = vk::VkAttachmentReference::default();
                color_attachment_ref.attachment = attachment_descriptions.len() as u32;
                color_attachment_ref.layout =
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
                color_attachments_refs.push(color_attachment_ref);
                colors.push(v.clone());
            } else if img.usage
                & vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT
                    as vk::VkImageUsageFlags
                != 0
            {
                attachment_description.finalLayout =
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
                depth_attachment_ref.layout =
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
                depth_attachment_ref.attachment = attachment_descriptions.len() as u32;
                depth = Some(v.clone());
            } else {
                vxunexpected!();
            }
            attachment_descriptions.push(attachment_description);
        }
        colors.shrink_to_fit();

        let mut subpass_description = vk::VkSubpassDescription::default();
        subpass_description.pipelineBindPoint =
            vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS;
        subpass_description.colorAttachmentCount = color_attachments_refs.len() as u32;
        subpass_description.pColorAttachments = color_attachments_refs.as_ptr();
        if depth.is_some() {
            subpass_description.pDepthStencilAttachment = &depth_attachment_ref;
        }

        let mut dependencies = [vk::VkSubpassDependency::default(); 2];
        dependencies[0].srcSubpass = vk::VK_SUBPASS_EXTERNAL;
        dependencies[0].srcStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as u32;
        dependencies[0].dstStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        dependencies[0].srcAccessMask = vk::VkAccessFlagBits::VK_ACCESS_MEMORY_READ_BIT as u32;
        dependencies[0].dstAccessMask = vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT
            as u32
            | vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32;
        dependencies[0].dependencyFlags =
            vk::VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as u32;
        dependencies[1].dstSubpass = vk::VK_SUBPASS_EXTERNAL;
        dependencies[1].srcStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        dependencies[1].dstStageMask =
            vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as u32;
        dependencies[1].srcAccessMask = vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT
            as u32
            | vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32;
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
            vkdev,
            &render_pass_create_info,
            null(),
            &mut vk_data,
        ));

        RenderPass {
            swapchain: None,
            colors,
            depth,
            vk_data,
        }
    }

    pub fn get_color_attachments(&self) -> &[Arc<ImageView>] {
        return &self.colors;
    }

    pub fn get_depth_attachment(&self) -> Option<&Arc<ImageView>> {
        return self.depth.as_ref();
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        let vkdev = if let Some(swapchain) = &self.swapchain {
            swapchain.logical_device.vk_data
        } else if self.colors.len() > 0 {
            let i = vxresult!(self.colors[0].image.read());
            i.logical_device.vk_data
        } else if let Some(depth) = &self.depth {
            let i = vxresult!(depth.image.read());
            i.logical_device.vk_data
        } else {
            vxunexpected!();
        };
        unsafe {
            vk::vkDestroyRenderPass(vkdev, self.vk_data, null());
        }
    }
}
