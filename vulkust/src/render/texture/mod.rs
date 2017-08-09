pub mod manager;

use std::fmt::Debug;
use super::super::system::file::File;
#[cfg(metal)]
use super::super::metal::texture::Texture2D as PlatformTexture2D;
#[cfg(vulkan)]
use super::super::vulkan::texture::Texture2D as PlatformTexture2D;

pub type Id = u64;

pub trait Texture: Debug {
    fn as_texture2d(&self) -> &Texture2D {
        logf!("This object can not convert to Texture2D.");
    }

    fn as_mut_texture2d(&mut self) -> &mut Texture2D {
        logf!("This object can not convert to Texture2D.");
    }
}

#[derive(Debug)]
pub struct Texture2D {
    pub raw: PlatformTexture2D,
}

impl Texture2D {
    pub fn new(file: &mut File) -> Self {
        let size: u64 = file.read_type();
        // logi!("Texture2D size is: {}", size);
        let data = file.read_bytes(size as usize);
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
