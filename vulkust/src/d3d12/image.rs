use super::super::render::image::{AttachmentType, Format};
use super::buffer::Manager as BufferManager;
use super::device::Device;
use super::memory::Manager as MemoryManager;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Image {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct View {}

impl View {
    pub(crate) fn new_texture_2d_with_pixels(
        width: u32,
        height: u32,
        data: &[u8],
        buffmgr: &Arc<RwLock<BufferManager>>,
    ) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn new_with_image(image: Arc<RwLock<Image>>) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn new_with_image_aspect(image: Arc<RwLock<Image>>, aspect_mask: u32) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn new_surface_attachment(
        logical_device: Arc<Device>,
        memory_mgr: &Arc<RwLock<MemoryManager>>,
        format: Format,
        attachment_type: AttachmentType,
    ) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn new_attachment(
        memory_mgr: &Arc<RwLock<MemoryManager>>,
        format: Format,
        attachment_type: AttachmentType,
        width: u32,
        height: u32,
    ) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn get_image(&self) -> &Arc<RwLock<Image>> {
        vxunimplemented!();
    }
}
