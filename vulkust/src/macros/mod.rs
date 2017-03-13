#[macro_export]
macro_rules! start {
    ($App:ident) => {
        fn main() {
            let mut app = $App::new();
            app.initialize();
            app.update();
            app.terminate();
        }
    };
}

#[macro_export]
macro_rules! loginfo {
    ($x:expr) => {
        println!("Vulkust Information MSG:{:?} file: {} line: {}", $x, file!(), line!());
    };
}
