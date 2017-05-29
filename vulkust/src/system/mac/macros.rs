#[macro_export]
#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
macro_rules! get_class {
    ($fmt:expr) => {
        match objc::runtime::Class::get($fmt) {
            Some(c) => { c },
            None => { logf!("Error class does not exists."); },
        }
    }
}
