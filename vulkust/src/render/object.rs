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
