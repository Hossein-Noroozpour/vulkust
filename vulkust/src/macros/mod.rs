#[macro_export]
macro_rules! start {
    ($App:ident) => {
        fn main() {
            use vulkust::system::application::Application as SysApp;
            let mut app = Box::new(SysApp::<$App>::new());
            app.run();
        }
    };
}

#[macro_export]
macro_rules! loginfo {
    ($x:expr) => {
        println!("Vulkust Information MSG:{:?} file: {} line: {}", $x, file!(), line!());
    };
}
