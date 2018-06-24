use gltf;

pub trait Object {
    fn name(&self) -> &str {
        vxunimplemented!();
    }

    fn render(&self) {
        vxunimplemented!();
    }

    fn disable_rendering(&mut self) {
        vxunimplemented!();
    }

    fn enable_rendering(&mut self) {
        vxunimplemented!();
    }
}

pub trait Loadable: Object + Sized {
    fn new_with_gltf(gltf::Node) -> Self {
        vxunexpected!();
    }
}
