#[macro_export]
#[cfg(not(target_os = "android"))]
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
#[cfg(target_os = "android")]
macro_rules! start {
    ($App:ident) => {
        #[allow(dead_code, non_snake_case)]
        #[no_mangle]
        pub unsafe extern fn ANativeActivity_onCreate(
            activity: *mut activity::ANativeActivity, saved_state: *mut libc::c_void,
            saved_state_size: libc::size_t) {
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
