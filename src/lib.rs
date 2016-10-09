#[macro_use]
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

mod pipeline;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        pipeline::run();
    }
}
