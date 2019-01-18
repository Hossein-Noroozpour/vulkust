use super::super::render::image::Layout;
use super::image::convert_layout;
use super::image::View as ImageView;
use super::swapchain::Swapchain;
use ash::version::DeviceV1_0;
use ash::vk;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct RenderPass {
    swapchain: Option<Arc<Swapchain>>,
    colors: Vec<Arc<ImageView>>,
    depth: Option<Arc<ImageView>>,
    vk_data: vk::RenderPass,
}

impl RenderPass {
    pub(crate) fn new_with_swapchain(swapchain: Arc<Swapchain>, clear: bool) -> Self {
        let vs = vec![swapchain.get_image_views()[0].clone()];
        let mut result = Self::new(vs, clear, false);
        result.swapchain = Some(swapchain);
        return result;
    }

    pub(crate) fn new_with_layouts(
        views: Vec<Arc<ImageView>>,
        clear: bool,
        start_layouts: &[Layout],
        end_layouts: &[Layout],
    ) -> Self {
        let mut attachment_descriptions = Vec::with_capacity(views.len()); // vec![vk::VkAttachmentDescription::default(); views_len];
        let mut color_attachments_refs = Vec::with_capacity(views.len()); // vec![vk::VkAttachmentReference::default(); views_len - 1];
        let mut colors = Vec::with_capacity(views.len());
        let mut depth_attachment_ref = vk::AttachmentReference::default();
        let mut vk_dev = None;
        let mut depth = None;
        let mut view_index = 0;
        for v in &views {
            let img = vxresult!(v.get_image().read());
            vk_dev = Some(*img.get_device().get_data());

            let mut attachment_description = vk::AttachmentDescription::builder()
                .format(img.get_vk_format())
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(if clear {
                    vk::AttachmentLoadOp::CLEAR
                } else {
                    vk::AttachmentLoadOp::LOAD
                })
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(convert_layout(&start_layouts[view_index]))
                .final_layout(convert_layout(&end_layouts[view_index]))
                .build();
            if vxflagcheck!(img.get_vk_usage(), vk::ImageUsageFlags::COLOR_ATTACHMENT) {
                let mut color_attachment_ref = vk::AttachmentReference::builder()
                    .attachment(attachment_descriptions.len() as u32)
                    .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .build();
                color_attachments_refs.push(color_attachment_ref);
                colors.push(v.clone());
            } else if vxflagcheck!(
                img.get_vk_usage(),
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
            ) {
                depth_attachment_ref.layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
                depth_attachment_ref.attachment = attachment_descriptions.len() as u32;
                depth = Some(v.clone());
            } else {
                vxunexpected!();
            }
            attachment_descriptions.push(attachment_description);
            view_index += 1;
        }
        colors.shrink_to_fit();

        let mut subpass_description = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments_refs);
        if depth.is_some() {
            subpass_description =
                subpass_description.depth_stencil_attachment(&depth_attachment_ref);
        }
        let subpass_description = subpass_description.build();

        let mut dependencies = [
            vk::SubpassDependency::builder()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .src_stage_mask(vk::PipelineStageFlags::BOTTOM_OF_PIPE)
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags::MEMORY_READ)
                .dst_access_mask(
                    vk::AccessFlags::COLOR_ATTACHMENT_READ
                        | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .dependency_flags(vk::DependencyFlags::BY_REGION)
                .build(),
            vk::SubpassDependency::builder()
                .dst_subpass(vk::SUBPASS_EXTERNAL)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_stage_mask(vk::PipelineStageFlags::BOTTOM_OF_PIPE)
                .src_access_mask(
                    vk::AccessFlags::COLOR_ATTACHMENT_READ
                        | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .dst_access_mask(vk::AccessFlags::MEMORY_READ)
                .dependency_flags(vk::DependencyFlags::BY_REGION)
                .build(),
        ];

        let mut render_pass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachment_descriptions)
            .subpasses(&[subpass_description])
            .dependencies(&dependencies);

        let vk_dev = vxunwrap!(vk_dev);

        let vk_data =
            vxresult!(unsafe { vk_dev.create_render_pass(&render_pass_create_info, None) });

        Self {
            swapchain: None,
            colors,
            depth,
            vk_data,
        }
    }

    pub(crate) fn new(views: Vec<Arc<ImageView>>, clear: bool, has_reader: bool) -> Self {
        let mut start_layouts = Vec::with_capacity(views.len());
        let mut end_layouts = Vec::with_capacity(views.len());
        for v in &views {
            start_layouts.push(if clear {
                Layout::Uninitialized
            } else {
                Layout::Display
            });
            let img = vxresult!(v.get_image().read());
            end_layouts.push(
                if vxflagcheck!(img.get_vk_usage(), vk::ImageUsageFlags::COLOR_ATTACHMENT) {
                    if has_reader {
                        Layout::ShaderReadOnly
                    } else {
                        Layout::Display
                    }
                } else if vxflagcheck!(
                    img.get_vk_usage(),
                    vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
                ) {
                    if has_reader {
                        Layout::ShaderReadOnly
                    } else {
                        Layout::DepthStencil
                    }
                } else {
                    vxunexpected!();
                },
            );
        }
        return Self::new_with_layouts(views, clear, &start_layouts, &end_layouts);
    }

    pub(crate) fn get_color_attachments(&self) -> &[Arc<ImageView>] {
        return &self.colors;
    }

    // pub(crate) fn get_depth_attachment(&self) -> Option<&Arc<ImageView>> {
    //     return self.depth.as_ref();
    // }

    pub(super) fn get_data(&self) -> &vk::RenderPass {
        return &self.vk_data;
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        let vkdev = if let Some(swapchain) = &self.swapchain {
            swapchain.get_logical_device().get_data()
        } else if self.colors.len() > 0 {
            let i = vxresult!(self.colors[0].get_image().read());
            i.get_device().get_data()
        } else if let Some(depth) = &self.depth {
            let i = vxresult!(depth.get_image().read());
            i.get_device().get_data()
        } else {
            vxunexpected!();
        };
        unsafe {
            vkdev.destroy_render_pass(self.vk_data, None);
        }
    }
}
