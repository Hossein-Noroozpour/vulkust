pub mod manager;
use std::sync::Arc;
use super::super::system::file::File;
use super::super::util::cell::DebugCell;
#[cfg(metal)]
use super::super::metal::texture::Texture2D as PlatformTexture2D;
#[cfg(vulkan)]
use super::super::vulkan::texture::Texture2D as PlatformTexture2D;

pub type Id = u64;

pub trait Texture {
    fn as_texture2d(&self) -> &Texture2D {
        logf!("This object can not convert to Texture2D.");
    }

    fn as_mut_texture2d(&mut self) -> &mut Texture2D {
        logf!("This object can not convert to Texture2D.");
    }
}

pub struct Texture2D {
    pub raw: PlatformTexture2D,
}

impl Texture2D {
    pub fn new(file: &Arc<DebugCell<File>>) -> Self {
        let size = file.borrow_mut().read_count();
        // logi!("Texture2D size is: {}", size);
        let data = file.borrow_mut().read_bytes(size as usize);
        Texture2D {
            raw: PlatformTexture2D::new(data),
        }
    }
}

impl Texture for Texture2D {
    fn as_texture2d(&self) -> &Texture2D {
        return self;
    }

    fn as_mut_texture2d(&mut self) -> &mut Texture2D {
        return self;
    }
}
