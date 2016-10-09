#[macro_use]
extern crate vulkano;

mod pipeline;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        pipeline::run();
    }
}
