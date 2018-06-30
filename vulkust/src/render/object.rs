use gltf;
use math;

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

pub trait Transferable {
    fn set_orientation(&mut self, math::Quaternion<f32>) {
        vxunimplemented!();
    }

    fn set_orientation_location(&mut self, math::Quaternion<f32>, math::Vector3<f32>) {
        vxunimplemented!();
    }
}
