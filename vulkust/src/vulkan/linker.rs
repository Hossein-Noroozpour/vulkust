use super::vulkan::*;
use super::super::system::linker::Linker as SysLinker;
// macro_rules! vkl {
//     ($()$) => {};
// }

pub struct Linker {
    library: SysLinker,
}

impl Linker {
    pub fn new() -> Self {
        let library = SysLinker::new("libvulkan.so");
        if !library.is_ok() {
            vxlogf!("Vulkan shared library (dll) not found.");
        }
        Linker {
            library
        }
    }
}
