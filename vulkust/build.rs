fn main() {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    println!("cargo:rustc-cfg=desktop_os");
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    println!("cargo:rustc-cfg=apple_os");
}
