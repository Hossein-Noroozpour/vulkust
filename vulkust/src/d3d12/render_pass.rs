use super::super::render::image::Layout;
use super::image::View as ImageView;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct RenderPass {}

impl RenderPass {
    pub(crate) fn new(views: Vec<Arc<ImageView>>, clear: bool, has_reader: bool) -> Self {
        vxunimplemented!();
    }
    pub(crate) fn new_with_layouts(
        views: Vec<Arc<ImageView>>,
        clear: bool,
        start_layouts: &[Layout],
        end_layouts: &[Layout],
    ) -> Self {
        vxunimplemented!();
    }
}
