#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Memory {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {}

impl Manager {
    pub(super) fn new() -> Self {
        Self {}
    }
}
