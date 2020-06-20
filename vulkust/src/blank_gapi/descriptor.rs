use super::super::render::texture::Texture;
use super::buffer::Dynamic as DynamicBuffer;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Set {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {}

impl Manager {
    pub(crate) fn create_gbuff_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_buffer_only_set(&mut self, _uniform: &DynamicBuffer) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_deferred_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_ssao_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_shadow_accumulator_directional_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _texturess: Vec<Vec<Arc<RwLock<Texture>>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }
}
