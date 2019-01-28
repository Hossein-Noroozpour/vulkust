use super::super::super::core::storage::Storage;
use super::super::command::Buffer as CmdBuffer;
use super::super::config::Configurations;
use super::super::framebuffer::Framebuffer;
use super::super::g_buffer_filler::GBufferFiller;
use super::super::gapi::GraphicApiEngine;
use super::super::image::{AttachmentType, Format, View as ImageView};
use super::super::pipeline::{Pipeline, PipelineType};
use super::super::render_pass::RenderPass;
use super::super::texture::{Manager as TextureManager, Texture};
use super::Pass;

/// A manager structure for passes
///
/// On its initialization it tries to initialize all the predefined passes.
/// User can add a customized pass through ```add```

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    storage: Storage<Pass>,
}

impl Manager {
    pub fn new(
        eng: &GraphicApiEngine,
        texmgr: &mut TextureManager,
        config: &Configurations,
    ) -> Self {
        Self {
            storage: Storage::new(),
        }
    }
}
