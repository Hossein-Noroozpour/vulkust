use super::super::super::core::storage::Storage;
use super::super::config::Configurations;
use super::super::gapi::GraphicApiEngine;
use super::super::texture::Manager as TextureManager;
use super::Pass;

/// A manager structure for passes
///
/// On its initialization it tries to initialize all the predefined passes.
/// User can add a customized pass through ```add```

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    storage: Storage<dyn Pass>,
}

impl Manager {
    pub fn new(
        _eng: &GraphicApiEngine,
        _texmgr: &mut TextureManager,
        _config: &Configurations,
    ) -> Self {
        Self {
            storage: Storage::new(),
        }
    }
}
