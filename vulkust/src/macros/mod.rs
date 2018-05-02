#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vulkust_start {
    ($App:ident) => {
        fn main() {
            use $crate::core::application::ApplicationTrait as CoreAppTrait;
            use $crate::system::application::Application as SysApp;
            use $crate::system::os::application::Application as OsApp;
            let os_app = Arc::new(RwLock::new(OsApp::new()));
            let core_app: Arc<RwLock<CoreAppTrait>> = Arc::new(RwLock::new($App::new()));
            let sys_app = Arc::new(RwLock::new(SysApp::new(core_app.clone(), os_app)));
            core_app
                .write()
                .unwrap()
                .set_system_application(sys_app.clone());
            sys_app.read().unwrap().run();
        }
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vulkust_start {
    ($App:ident) => {
        #[allow(dead_code, non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ANativeActivity_onCreate(
            activity: *mut $crate::system::android::activity::ANativeActivity,
            saved_state: *mut $crate::libc::c_void,
            saved_state_size: $crate::libc::size_t,
        ) {
            use $crate::core::application::ApplicationTrait as CoreAppTrait;
            use $crate::system::os::application::Application as OsApp;
            let core_app: Arc<RwLock<CoreAppTrait>> = Arc::new(RwLock::new($App::new()));
            let os_app = Arc::new(RwLock::new(OsApp::new(
                activity,
                saved_state,
                saved_state_size,
            )));
            let os_app_clone = os_app.clone();
            vxresult!(os_app.read()).initialize(os_app_clone, core_app);
        }
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vxlogi {
    ($fmt:expr) => {
        println!("{}", format!("Vulkust information message in file: {} line: {} {}", file!(), line!(), $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!("{}", format!("Vulkust information message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vxloge {
    ($fmt:expr) => {
        eprintln!("{}", format!("Vulkust error message in file: {} line: {} {}", file!(), line!(), $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("{}", format!("Vulkust error message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vxlogf {
    ($fmt:expr) => (
        panic!("{}", format!("Vulkust fatal message in file: {} line: {} {}", file!(), line!(), $fmt));
    );
    ($fmt:expr, $($arg:tt)*) => {
        panic!("{}", format!("Vulkust fatal message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vxlogi {
    ($fmt:expr) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &format!(
            "Vulkust Information MSG in file: {} line: {} {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &format!(
            "Vulkust Information MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vxloge {
    ($fmt:expr) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &format!(
            "Vulkust Error MSG in file: {} line: {} {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &format!(
            "Vulkust Error MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vxlogf {
    ($fmt:expr) => ({
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal, &format!(
            "Vulkust Fatal MSG in file: {} line: {} {}", file!(), line!(), format!($fmt)));
        panic!("Terminated!");
    });
    ($fmt:expr, $($arg:tt)*) => ({
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal,
            &format!("Vulkust Fatal MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
        panic!("Terminated!");
    });
}

#[macro_export]
macro_rules! vxunwrap {
    ($e:expr) => {
        match &$e {
            Some(v) => v,
            None => vxlogf!("Unwrap failed!"),
        }
    };
}

#[macro_export]
macro_rules! vxresult {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => vxlogf!("Unwrap failed! {:?}", e),
        }
    };
}

#[macro_export]
macro_rules! vxunimplemented {
    () => {
        vxlogf!("Not implemented")
    };
}

#[macro_export]
macro_rules! vxunexpected {
    () => {
        vxlogf!("Unexpected")
    };
}
