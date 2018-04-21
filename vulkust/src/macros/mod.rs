#[macro_export]
#[cfg(desktop_os)]
macro_rules! vulkust_start {
    ($App:ident) => {
        fn main() {
            use vulkust::core::application::ApplicationTrait as CoreAppTrait;
            use vulkust::system::application::Application as SysApp;
            let core_app: Arc<RwLock<CoreAppTrait>> = Arc::new(RwLock::new($App::new()));
            let sys_app = Arc::new(RwLock::new(SysApp::new(core_app.clone())));
            core_app
                .write()
                .unwrap()
                .set_system_application(sys_app.clone());
            sys_app.read().unwrap().run();
        }
    };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! start {
    ($App:ident) => {
        #[allow(dead_code, non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ANativeActivity_onCreate(
            activity: *mut vulkust::system::android::activity::ANativeActivity,
            saved_state: *mut std::os::raw::c_void,
            saved_state_size: usize,
        ) {
            use std::mem::transmute;
            use vulkust::system::application::Application as SysApp;
            SysApp::<$App>::new(
                activity,
                transmute(saved_state),
                transmute(saved_state_size),
            );
        }
    };
}

#[macro_export]
#[cfg(desktop_os)]
macro_rules! vxlogi {
    ($fmt:expr) => {
        let s = format!("Vulkust information message in file: {} line: {} {}", file!(), line!(), $fmt);
        println!("{}", s);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let s = format!("Vulkust information message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*));
        println!("{}", s);
    };
}

#[macro_export]
#[cfg(desktop_os)]
macro_rules! vxloge {
    ($fmt:expr) => {
        let s = format!("Vulkust error message in file: {} line: {} {}", file!(), line!(), $fmt);
        eprintln!("{}", s);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let s = format!("Vulkust error message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*));
        eprintln!("{}", s);
    };
}

#[macro_export]
#[cfg(desktop_os)]
macro_rules! vxlogf {
    ($fmt:expr) => (
        panic!("{}", format!("Vulkust fatal message in file: {} line: {} {}", file!(), line!(), $fmt));
    );
    ($fmt:expr, $($arg:tt)*) => {
        panic!("{}", format!("Vulkust fatal message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! logi {
    ($fmt:expr) => {
        let s = format!(
            "Vulkust Information MSG in file: {} line: {} {}", file!(), line!(), format!($fmt));
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &s);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let s = format!(
            "Vulkust Information MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*));
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &s);
    };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! loge {
    ($fmt:expr) => {
        let s = format!(
            "Vulkust Error MSG in file: {} line: {} {}", file!(), line!(), format!($fmt));
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &s);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let s = format!(
            "Vulkust Error MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*));
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &s);
    };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! logf {
    ($fmt:expr) => {
        let s = format!(
            "Vulkust Fatal MSG in file: {} line: {} {}", file!(), line!(), format!($fmt));
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal, &s);
        panic!("Terminated!");
    };
    ($fmt:expr, $($arg:tt)*) => {
        let s = format!("Vulkust Fatal MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*));
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal, &s);
        panic!("Terminated!");
    };
}

#[macro_export]
macro_rules! vxunwrap {
    ($e:expr) => (
        match $e {
            Some(v) => v,
            None => vxlogf!("Unwrap failed!"),
        }
    )
}