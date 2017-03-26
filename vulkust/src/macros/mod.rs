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
macro_rules! logi {
    ($fmt:expr) => {
        print!("Vulkust Information MSG in file: {} line: {} ", file!(), line!());
        println!($fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!("Vulkust Information MSG in file: {} line: {} ", file!(), line!());
        println!($fmt, $($arg)*);
    };
}

#[macro_export]
macro_rules! loge {
    ($fmt:expr) => {
        print!("Vulkust Error MSG in file: {} line: {} ", file!(), line!());
        println!($fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!("Vulkust Error MSG in file: {} line: {} ", file!(), line!());
        println!($fmt, $($arg)*);
    };
}

#[macro_export]
macro_rules! logf {
    ($fmt:expr) => {
        print!("Vulkust Fatal MSG in file: {} line: {} ", file!(), line!());
        panic!($fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!("Vulkust Fatal MSG in file: {} line: {} ", file!(), line!());
        panic!($fmt, $($arg)*);
    };
}

macro_rules! default_window_width {
    () => ( 1280 as _ )
}

macro_rules! default_window_height {
    () => ( 720 as _ )
}
