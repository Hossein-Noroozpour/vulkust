#[macro_export]
#[cfg(any(target_os = "macos"))]
macro_rules! get_class {
    ($class_name:expr) => {
        match objc::runtime::Class::get($class_name) {
            Some(c) => { c },
            None => { logf!("Class: {:?} does not exist.", $class_name); },
        }
    }
}

#[macro_export]
#[cfg(any(target_os = "macos"))]
macro_rules! dec_class {
    ($class_name:expr, $super_class:ident) => {
        match objc::declare::ClassDecl::new($class_name, $super_class) {
            Some(c) => { c },
            None => {
                logf!(
                    "Can not create class {:?} with super class {:?}.",
                    $class_name, $super_class);
            },
        }
    }
}
