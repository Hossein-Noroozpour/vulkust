#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vx_log_i {
    ($fmt:expr) => {
        println!("{}", format!("[\x1b[92mINFO\x1b[0m] {} \x1b[94m{}:{}\x1b[0m", $fmt, file!(), line!()));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!("{}", format!("[\x1b[92mINFO\x1b[0m] {} \x1b[94m{}:{}\x1b[0m", format!($fmt, $($arg)*), file!(), line!()));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vx_log_e {
    ($fmt:expr) => {
        eprintln!("{}", format!("[\x1b[93mERROR\x1b[0m] {} \x1b[94m{}:{}\x1b[0m", $fmt, file!(), line!()));
    };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("{}", format!("[\x1b[93mERROR\x1b[0m] {} \x1b[94m{}:{}\x1b[0m", format!($fmt, $($arg)*), file!(), line!()));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vx_log_f {
    ($fmt:expr) => (
        panic!("{}", format!("[\x1b[91mFATAL\x1b[0m] {} \x1b[94m{}:{}\x1b[0m", $fmt, file!(), line!()))
    );
    ($fmt:expr, $($arg:tt)*) => {
        panic!("{}", format!("[\x1b[91mFATAL\x1b[0m] {} \x1b[94m{}:{}\x1b[0m", format!($fmt, $($arg)*), file!(), line!()))
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vx_log_i {
    ($fmt:expr) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &format!(
            "Vulkust Information MSG in: {}:{} {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &format!(
            "Vulkust Information MSG in: {}:{} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vx_log_e {
    ($fmt:expr) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &format!(
            "Vulkust Error MSG in: {}:{} {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &format!(
            "Vulkust Error MSG in: {}:{} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vx_log_f {
    ($fmt:expr) => ({
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal, &format!(
            "Vulkust Fatal MSG in: {}:{} {}", file!(), line!(), format!($fmt)));
        panic!("Terminated!");
    });
    ($fmt:expr, $($arg:tt)*) => ({
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal,
            &format!("Vulkust Fatal MSG in: {}:{} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
        panic!("Terminated!");
    });
}

#[macro_export]
macro_rules! vx_unwrap {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => vx_log_f!("Unwrap failed!"),
        }
    };
}

#[macro_export]
macro_rules! vx_result {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => vx_log_f!("Unwrap failed! {:?}", e),
        }
    };
}

#[macro_export]
macro_rules! vx_unimplemented {
    () => {
        vx_log_f!("Not implemented")
    };
}

#[macro_export]
macro_rules! vx_unexpected {
    () => {
        vx_log_f!("Unexpected")
    };
}

#[macro_export]
macro_rules! vx_todo {
    () => {
        vx_log_e!("TODO")
    };
}

macro_rules! vx_flag_check {
    ($f:expr, $b:expr) => {
        $f & $b == $b
    };
}
