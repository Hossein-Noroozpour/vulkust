pub trait Application {
    fn new() -> Self;
    fn initialize(&mut self) -> bool;
    fn update(&mut self) -> bool;
    fn terminate(&mut self);
}
