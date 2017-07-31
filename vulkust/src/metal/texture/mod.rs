use std::ptr::null_mut;
use super::super::core::application::ApplicationTrait;
use super::super::system::metal as mtl;
use super::super::system::metal::foundation as fnd;
use super::super::system::metal::kit as mtk;
use super::super::system::os::OsApplication;

#[derive(Debug)]
pub struct Texture2D {
    pub color_map: mtl::Id,
}

impl Texture2D {
    pub fn new<CoreApp>(data: Vec<u8>, os_app: *mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let texture_loader_option_texture_usage =
            fnd::NSNumber::new_uint(mtl::TEXTURE_USAGE_SHADER_READ.bits());
        let texture_loader_option_texture_storage_mode =
            fnd::NSNumber::new_uint(mtl::STORAGE_MODE_PRIVATE.bits());
        let texture_loader_options = fnd::NSDictionaryBuilder::new()
            .add(
                unsafe { mtk::MTKTextureLoaderOptionTextureUsage },
                texture_loader_option_texture_usage.id,
            )
            .add(
                unsafe { mtk::MTKTextureLoaderOptionTextureStorageMode },
                texture_loader_option_texture_storage_mode.id,
            )
            .build();
        let mut error = mtl::NSError::null();
        let data = fnd::NSData::new(data.as_ptr(), data.len());
        let color_map: mtl::Id = unsafe {
            msg_send![
                (*(*os_app).render_engine).texture_loader, 
                newTextureWithData:data
                options:texture_loader_options
                error:error.as_ptr()
            ]
        };
        if color_map == null_mut() || error.is_error() {
            logf!("Creating texture failed with error: {}", error);
        }
        Texture2D {
            color_map: color_map,
        }
    }
}
