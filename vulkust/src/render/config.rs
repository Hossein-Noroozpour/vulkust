#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Configurations {
    pub number_cascaded_shadows: usize,
}

impl Default for Configurations {
    fn default() -> Self {
        Configurations {
            number_cascaded_shadows: 6,
        }
    }
}
