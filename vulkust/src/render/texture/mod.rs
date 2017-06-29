pub mod manager;

use std::fmt::Debug;
use super::super::system::file::File;
use super::super::system::os::OsApplication;
use super::super::core::application::ApplicationTrait;
#[cfg(metal)]
use super::super::metal::texture::Texture2D as PlatformTexture2D;
#[cfg(vulkan)]
use super::super::vulkan::texture::Texture2D as PlatformTexture2D;

pub trait TextureTrait: Debug {
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
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let size: u64 = file.read_type();
        // logi!("Texture2D size is: {}", size);
        let data = file.read_bytes(size as usize);
        Texture2D { raw: PlatformTexture2D::new(data, os_app) }
    }
}

impl TextureTrait for Texture2D {
    fn as_texture2d(&self) -> &Texture2D {
        return self;
    }

    fn as_mut_texture2d(&mut self) -> &mut Texture2D {
        return self;
    }
}
