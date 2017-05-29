use super::types::Id;

#[link(name = "Foundation", kind = "framework")]
extern {
    pub static NSDefaultRunLoopMode: Id;
}
