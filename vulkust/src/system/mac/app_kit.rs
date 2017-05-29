use super::types::Id;

#[link(name = "AppKit", kind = "framework")]
extern {
    pub static NSImageHintCTM: Id;
}
