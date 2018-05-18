use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut in_macos = false;
    let mut in_ios = false;
    let mut in_android = false;
    let mut in_linux = false;
    let mut in_windows = false;
    if target.contains("ios") {
        in_ios = true;
    } else if target.contains("darwin") {
        in_macos = true;
    } else if target.contains("android") {
        in_android = true;
    } else if target.contains("linux") {
        in_linux = true;
    } else if target.contains("windows") {
        in_windows = true;
    }
    if !(in_macos || in_android || in_ios || in_linux || in_windows) {
        panic!("Unsupported platform!");
    }
    if in_macos || in_ios {
        println!("cargo:rustc-cfg=apple_os");
    }
    if in_linux || in_windows || in_macos {
        println!("cargo:rustc-cfg=desktop_os");
    }
    if in_linux || in_ios || in_macos || in_android {
        println!("cargo:rustc-cfg=unix_based_os");
    }
    if in_macos {
        let vulkan_sdk = env::var("VULKAN_SDK").unwrap();
        println!("cargo:rustc-link-search=native={}/macOS/lib", vulkan_sdk);
    }
}
