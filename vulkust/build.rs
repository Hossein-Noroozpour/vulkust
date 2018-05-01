use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut in_macos = false;
    let mut in_android = false;
    let mut in_linux = false;
    let mut in_windows = false;
    if target.contains("darwin") {
        in_macos = true;
    } else if target.contains("android") {
        in_android = true;
    } else if target.contains("linux") {
        in_linux = true;
    } else if target.contains("windows") {
        in_windows = true;
    }
    if !(in_macos || in_android || in_linux || in_windows) {
        panic!("Unsupported platform!");
    }
    if in_linux || in_windows || in_macos {
        println!("cargo:rustc-cfg=desktop_os");
    }
    if in_linux || in_macos || in_android {
        println!("cargo:rustc-cfg=unix_based_os");
    }
}
