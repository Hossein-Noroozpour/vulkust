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
            activity: *mut vulkust::system::android::activity::ANativeActivity,
            saved_state: *mut std::os::raw::c_void,
            saved_state_size: usize) {
            use std::mem::transmute;
            use vulkust::system::application::Application as SysApp;
            Box::new(SysApp::<$App>::new(
                activity, transmute(saved_state), transmute(saved_state_size)));
        }
    };
}

#[macro_export]
#[cfg(any(target_os = "linux", target_os = "windows"))]
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
#[cfg(any(target_os = "linux", target_os = "windows"))]
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
#[cfg(any(target_os = "linux", target_os = "windows"))]
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

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! logi {
    ($fmt:expr) => {
        extern crate vulkust;
        ::vulkust::system::android::log::print(
            ::vulkust::system::android::log::Priority::Info,
            format!("Vulkust Information MSG in file: {} line: {} ", file!(), line!()) +
            format!($fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        extern crate vulkust;
        ::vulkust::system::android::log::print(
            ::vulkust::system::android::log::Priority::Info,
            format!("Vulkust Information MSG in file: {} line: {} ", file!(), line!()) +
            format!($fmt, $($arg)*));
    };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! loge {
    ($fmt:expr) => {
        extern crate vulkust;
        ::vulkust::system::android::log::print(
            ::vulkust::system::android::log::Priority::Error,
            format!("Vulkust Error MSG in file: {} line: {} ", file!(), line!()) +
            format!($fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        extern crate vulkust;
        ::vulkust::system::android::log::print(
            ::vulkust::system::android::log::Priority::Error,
            format!("Vulkust Error MSG in file: {} line: {} ", file!(), line!()) +
            format!($fmt, $($arg)*));
    };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! logf {
    ($fmt:expr) => {
        extern crate vulkust;
        ::vulkust::system::android::log::print(
            ::vulkust::system::android::log::Priority::Fatal,
            format!("Vulkust Fatal MSG in file: {} line: {} ", file!(), line!()) +
            format!($fmt));
        panic!("Terminated!");
    };
    ($fmt:expr, $($arg:tt)*) => {
        extern crate vulkust;
        ::vulkust::system::android::log::print(
            ::vulkust::system::android::log::Priority::Fatal,
            format!("Vulkust Fatal MSG in file: {} line: {} ", file!(), line!()) +
            format!($fmt, $($arg)*));
        panic!("Terminated!");
    };
}

macro_rules! default_window_width {
    () => ( 1280 as _ )
}

macro_rules! default_window_height {
    () => ( 720 as _ )
}
