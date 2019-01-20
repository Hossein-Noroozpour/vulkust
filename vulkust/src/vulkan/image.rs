use super::super::core::allocate::Object as AlcObject;
use super::super::render::image::{AttachmentType, Format, Layout};
use super::buffer::Manager as BufferManager;
use super::command::Buffer as CmdBuffer;
use super::device::Logical as LogicalDevice;
use super::memory::{Location as MemoryLocation, Manager as MemoryManager, Memory};
use ash::version::DeviceV1_0;
use ash::vk;
use std::cmp::{max, min};
use std::sync::{Arc, RwLock};

pub(super) fn convert_layout(f: &Layout) -> vk::ImageLayout {
    match f {
        &Layout::Uninitialized => return vk::ImageLayout::UNDEFINED,
        &Layout::DepthStencil => {
            return vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
        }
        &Layout::Display => return vk::ImageLayout::PRESENT_SRC_KHR,
        &Layout::ShaderReadOnly => {
            return vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        }
        // _ => vxunexpected!(),
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Image {
    logical_device: Arc<LogicalDevice>,
    layout: vk::ImageLayout,
    format: vk::Format,
    mips_count: u8,
    usage: vk::ImageUsageFlags,
    memory: Option<Arc<RwLock<Memory>>>,
    width: u32,
    height: u32,
    depth: u32,
    vk_data: vk::Image,
}

impl Image {
    pub(crate) fn new_with_info(
        info: &vk::ImageCreateInfo,
        memory_mgr: &Arc<RwLock<MemoryManager>>,
    ) -> Self {
        let logical_device = vxresult!(memory_mgr.read()).get_device().clone();
        let vk_dev = logical_device.get_data().clone();
        let vk_data = vxresult!(unsafe { vk_dev.create_image(info, None) });
        let mem_reqs = unsafe { vk_dev.get_image_memory_requirements(vk_data) };
        let memory = vxresult!(memory_mgr.write()).allocate(&mem_reqs, MemoryLocation::GPU);
        {
            let memory_r = vxresult!(memory.read());
            let root_memory = vxresult!(memory_r.get_root().read());
            vxresult!(unsafe {
                vk_dev.bind_image_memory(
                    vk_data,
                    root_memory.get_data(),
                    memory_r.get_allocated_memory().get_offset() as vk::DeviceSize,
                )
            });
        }
        Self {
            logical_device,
            vk_data,
            layout: info.initial_layout,
            format: info.format,
            mips_count: info.mip_levels as u8,
            usage: info.usage,
            width: info.extent.width,
            height: info.extent.height,
            depth: info.extent.depth,
            memory: Some(memory),
        }
    }

    pub(crate) fn new_2d_with_vk_data(
        logical_device: Arc<LogicalDevice>,
        vk_data: vk::Image,
        layout: vk::ImageLayout,
        format: vk::Format,
        usage: vk::ImageUsageFlags,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            logical_device,
            layout,
            format,
            mips_count: 1,
            vk_data,
            usage,
            width,
            height,
            depth: 1,
            memory: None,
        }
    }

    pub(crate) fn new_2d_with_pixels(
        width: u32,
        height: u32,
        data: &[u8],
        buffmgr: &Arc<RwLock<BufferManager>>,
    ) -> Arc<RwLock<Self>> {
        let mip_levels = Self::calculate_mip_levels_2d(width, height);
        #[cfg(debug_mode)]
        {
            if mip_levels <= 0 {
                vxlogf!(
                    "Unexpected image aspects, width: {} height: {} mip-levels: {}",
                    width,
                    height,
                    mip_levels
                );
            }
        }
        let format = vk::Format::R8G8B8A8_UNORM;
        let image_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(
                vk::Extent3D::builder()
                    .width(width)
                    .height(height)
                    .depth(1)
                    .build(),
            )
            .mip_levels(mip_levels)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .usage(
                vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::TRANSFER_SRC
                    | vk::ImageUsageFlags::SAMPLED,
            );
        let memmgr = {
            let buffmgr = vxresult!(buffmgr.read());
            let memmgr = vxresult!(buffmgr.get_gpu_root_buffer().get_memory().read())
                .get_manager()
                .clone();
            memmgr
        };
        let myself = Arc::new(RwLock::new(Self::new_with_info(&image_info, &memmgr)));
        vxresult!(buffmgr.write()).create_staging_image(&myself, data, &image_info);
        myself
    }

    fn calculate_mip_levels_2d(width: u32, height: u32) -> u32 {
        let mut a = min(width, height);
        let mut result = 0;
        while a > 0 {
            a >>= 1;
            result += 1;
        }
        return result;
    }

    pub(crate) fn set_layout(&mut self, cmd: &mut CmdBuffer, new_layout: vk::ImageLayout) {
        let mut dst_stage = vk::PipelineStageFlags::TRANSFER;
        let mut src_access_mask = match self.layout {
            vk::ImageLayout::UNDEFINED => vk::AccessFlags::empty(),
            vk::ImageLayout::PREINITIALIZED => vk::AccessFlags::HOST_WRITE,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL => vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
            }
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL => vk::AccessFlags::TRANSFER_READ,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL => vk::AccessFlags::TRANSFER_WRITE,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL => vk::AccessFlags::SHADER_READ,
            _ => vxunexpected!(),
        };
        let dst_access_mask = match new_layout {
            vk::ImageLayout::TRANSFER_DST_OPTIMAL => vk::AccessFlags::TRANSFER_WRITE,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL => vk::AccessFlags::TRANSFER_READ,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL => vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
            }
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL => {
                dst_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
                if src_access_mask.is_empty() {
                    src_access_mask = vk::AccessFlags::HOST_WRITE | vk::AccessFlags::TRANSFER_WRITE;
                }
                vk::AccessFlags::SHADER_READ
            }
            _ => vxunexpected!(),
        };
        let barrier = vk::ImageMemoryBarrier::builder()
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .old_layout(self.layout)
            .new_layout(new_layout)
            .image(self.vk_data)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .layer_count(1)
                    .level_count(1)
                    .build(),
            )
            .src_access_mask(src_access_mask)
            .dst_access_mask(dst_access_mask)
            .build();
        let src_stage = vk::PipelineStageFlags::TRANSFER;
        cmd.pipeline_image_barrier(src_stage, dst_stage, vk::DependencyFlags::empty(), &barrier);
        self.layout = new_layout;
    }

    pub(super) fn generate_mips(&mut self, cmd: &mut CmdBuffer) {
        self.set_layout(cmd, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
        for mi in 1..self.mips_count {
            let image_blit = vk::ImageBlit::builder()
                .src_subresource(
                    vk::ImageSubresourceLayers::builder()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1)
                        .mip_level(mi as u32 - 1)
                        .build(),
                )
                .src_offsets([
                    vk::Offset3D { x: 0, y: 0, z: 0 },
                    vk::Offset3D {
                        x: max(1, self.width as i32 >> (mi - 1) as i32),
                        y: max(1, self.height as i32 >> (mi - 1) as i32),
                        z: max(1, self.depth as i32 >> (mi - 1) as i32),
                    },
                ])
                .dst_subresource(
                    vk::ImageSubresourceLayers::builder()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1)
                        .mip_level(mi as u32)
                        .build(),
                )
                .dst_offsets([
                    vk::Offset3D { x: 0, y: 0, z: 0 },
                    vk::Offset3D {
                        x: max(1, self.width as i32 >> mi as i32),
                        y: max(1, self.height as i32 >> mi as i32),
                        z: max(1, self.depth as i32 >> mi as i32),
                    },
                ])
                .build();

            let mip_sub_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(mi as u32)
                .level_count(1)
                .layer_count(1)
                .build();

            // // Transiton current mip level to transfer dest
            // vks::tools::setImageLayout(
            //     blitCmd,
            //     texture.image,
            //     VK_IMAGE_LAYOUT_UNDEFINED,
            //     VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            //     mipSubRange,
            //     VK_PIPELINE_STAGE_TRANSFER_BIT,
            //     VK_PIPELINE_STAGE_HOST_BIT,
            // );

            // // Blit from previous level
            // vkCmdBlitImage(
            //     blitCmd,
            //     texture.image,
            //     VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            //     texture.image,
            //     VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            //     1,
            //     &imageBlit,
            //     VK_FILTER_LINEAR,
            // );

            // // Transiton current mip level to transfer source for read in next iteration
            // vks::tools::setImageLayout(
            //     blitCmd,
            //     texture.image,
            //     VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            //     VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            //     mipSubRange,
            //     VK_PIPELINE_STAGE_HOST_BIT,
            //     VK_PIPELINE_STAGE_TRANSFER_BIT,
            // );
        }
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    // pub(crate) fn get_format(&self) -> Format {
    //     return convert_to_format(self.format);
    // }

    pub(crate) fn get_data(&self) -> vk::Image {
        return self.vk_data;
    }

    pub(crate) fn get_device(&self) -> &Arc<LogicalDevice> {
        return &self.logical_device;
    }

    pub(super) fn get_vk_usage(&self) -> vk::ImageUsageFlags {
        return self.usage;
    }

    pub(super) fn get_vk_format(&self) -> vk::Format {
        return self.format;
    }

    pub(super) fn get_mips_count(&self) -> u8 {
        return self.mips_count;
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if self.memory.is_some() {
            unsafe {
                self.logical_device
                    .get_data()
                    .destroy_image(self.vk_data, None);
            }
        }
    }
}

unsafe impl Send for Image {}

unsafe impl Sync for Image {}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct View {
    image: Arc<RwLock<Image>>,
    vk_data: vk::ImageView,
}

impl View {
    pub(crate) fn new_with_vk_image(
        logical_device: Arc<LogicalDevice>,
        vk_image: vk::Image,
        format: vk::Format,
        layout: vk::ImageLayout,
        usage: vk::ImageUsageFlags,
        width: u32,
        height: u32,
    ) -> Self {
        Self::new_with_image(Arc::new(RwLock::new(Image::new_2d_with_vk_data(
            logical_device,
            vk_image,
            layout,
            format,
            usage,
            width,
            height,
        ))))
    }

    pub(crate) fn new_texture_2d_with_pixels(
        width: u32,
        height: u32,
        data: &[u8],
        buffmgr: &Arc<RwLock<BufferManager>>,
    ) -> Self {
        let image = Image::new_2d_with_pixels(width, height, data, buffmgr);
        Self::new_with_image(image)
    }

    pub(crate) fn new_with_image(image: Arc<RwLock<Image>>) -> Self {
        return Self::new_with_image_aspect(image, vk::ImageAspectFlags::COLOR);
    }

    pub(crate) fn new_with_image_aspect(
        image: Arc<RwLock<Image>>,
        aspect_mask: vk::ImageAspectFlags,
    ) -> Self {
        let vk_data = {
            let img = vxresult!(image.read());
            let ref dev = &img.logical_device;
            let view_create_info = vk::ImageViewCreateInfo::builder()
                .image(img.vk_data)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(img.format)
                .components(
                    vk::ComponentMapping::builder()
                        .r(vk::ComponentSwizzle::R)
                        .g(vk::ComponentSwizzle::G)
                        .b(vk::ComponentSwizzle::B)
                        .a(vk::ComponentSwizzle::A)
                        .build(),
                )
                .subresource_range(
                    vk::ImageSubresourceRange::builder()
                        .aspect_mask(aspect_mask)
                        .level_count(img.mips_count as u32)
                        .layer_count(1)
                        .build(),
                );
            vxresult!(unsafe { dev.get_data().create_image_view(&view_create_info, None,) })
        };
        Self { image, vk_data }
    }

    pub(crate) fn new_surface_attachment(
        logical_device: Arc<LogicalDevice>,
        memory_mgr: &Arc<RwLock<MemoryManager>>,
        format: Format,
        attachment_type: AttachmentType,
    ) -> Self {
        let surface_caps = logical_device.get_physical().get_surface_capabilities();
        return Self::new_attachment(
            memory_mgr,
            format,
            attachment_type,
            surface_caps.current_extent.width,
            surface_caps.current_extent.height,
        );
    }

    pub(crate) fn new_attachment(
        memory_mgr: &Arc<RwLock<MemoryManager>>,
        format: Format,
        attachment_type: AttachmentType,
        width: u32,
        height: u32,
    ) -> Self {
        let aspect_mask = match attachment_type {
            AttachmentType::ColorGBuffer | AttachmentType::ColorDisplay => {
                vk::ImageAspectFlags::COLOR
            }
            AttachmentType::ShadowAccumulator => vk::ImageAspectFlags::COLOR,
            AttachmentType::DepthGBuffer | AttachmentType::DepthShadowBuffer => {
                vk::ImageAspectFlags::DEPTH
            }
            AttachmentType::DepthStencilDisplay => {
                vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
            }
        };
        let usage = match attachment_type {
            AttachmentType::ColorGBuffer => {
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED
            }
            AttachmentType::ShadowAccumulator => {
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED
            }
            AttachmentType::ColorDisplay => vk::ImageUsageFlags::COLOR_ATTACHMENT,
            AttachmentType::DepthGBuffer | AttachmentType::DepthShadowBuffer => {
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED
            }
            AttachmentType::DepthStencilDisplay => vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        };
        let vkfmt = vxresult!(memory_mgr.read())
            .get_device()
            .convert_format(format);
        let image_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vkfmt)
            .extent(
                vk::Extent3D::builder()
                    .width(width)
                    .height(height)
                    .depth(1)
                    .build(),
            )
            .mip_levels(1)
            .array_layers(1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .samples(vk::SampleCountFlags::TYPE_1);
        let image = Arc::new(RwLock::new(Image::new_with_info(&image_info, memory_mgr)));
        return Self::new_with_image_aspect(image, aspect_mask);
    }

    pub(crate) fn get_image(&self) -> &Arc<RwLock<Image>> {
        return &self.image;
    }

    pub(crate) fn get_data(&self) -> vk::ImageView {
        return self.vk_data;
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            let img = vxresult!(self.image.read());
            img.logical_device
                .get_data()
                .destroy_image_view(self.vk_data, None);
        }
    }
}

unsafe impl Send for View {}

unsafe impl Sync for View {}
