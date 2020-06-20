use super::super::render::image::{AttachmentType, Format};
use super::buffer::Manager as BufferManager;
use super::device::Device;
use super::memory::Manager as MemoryManager;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Image {}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct View {}

impl View {
    pub(crate) fn new_texture_2d_with_pixels(
        _width: u32,
        _height: u32,
        _data: &[u8],
        _buffmgr: &Arc<RwLock<BufferManager>>,
    ) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn new_with_image(_image: Arc<RwLock<Image>>) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn new_with_image_aspect(_image: Arc<RwLock<Image>>, _aspect_mask: u32) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn new_surface_attachment(
        _logical_device: Arc<Device>,
        _memory_mgr: &Arc<RwLock<MemoryManager>>,
        _format: Format,
        _attachment_type: AttachmentType,
    ) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn new_attachment(
        _memory_mgr: &Arc<RwLock<MemoryManager>>,
        _format: Format,
        _attachment_type: AttachmentType,
        _width: u32,
        _height: u32,
    ) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn get_image(&self) -> &Arc<RwLock<Image>> {
        vx_unimplemented!();
    }
}
