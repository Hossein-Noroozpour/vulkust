fn main() {
    #[cfg(
        all(
            any(target_os = "linux", target_os = "macos", target_os = "windows"),
            not(target_os = "android")
        )
    )]
    println!("cargo:rustc-cfg=desktop_os");
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    println!("cargo:rustc-cfg=apple_os");
    #[cfg(debug_assertions)]
    println!("cargo:rustc-cfg=debug_mode");
    #[cfg(any(target_os = "linux", target_os = "ios", target_os = "macos", target_os = "android"))]
    println!("cargo:rustc-cfg=in_unix");
}
