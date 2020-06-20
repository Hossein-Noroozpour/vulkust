use super::super::render::image::Layout;
use super::image::View as ImageView;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct RenderPass {}

impl RenderPass {
    pub(crate) fn new(_views: Vec<Arc<ImageView>>, _clear: bool, _has_reader: bool) -> Self {
        vx_unimplemented!();
    }
    pub(crate) fn new_with_layouts(
        _views: Vec<Arc<ImageView>>,
        _clear: bool,
        _start_layouts: &[Layout],
        _end_layouts: &[Layout],
    ) -> Self {
        vx_unimplemented!();
    }
}
