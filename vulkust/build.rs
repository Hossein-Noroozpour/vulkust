fn main() {
    #[cfg(any(
        all(target_os = "windows", not(feature = "use-directx12")),
        target_os = "linux",
        target_os = "android",
        feature = "use-vulkan"))]
    println!("cargo:rustc-cfg=vulkan");
#[cfg(any( // Maybe in future Apple reasoned to use Vulkan
        all(any(target_os = "ios", target_os = "macos"), not(feature = "use-vulkan")),
        feature = "use-metal"))]
    println!("cargo:rustc-cfg=metal");
    #[cfg(all(
        target_os = "windows",
        feature = "use-directx12"))]
    println!("cargo:rustc-cfg=directx12");
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    println!("cargo:rustc-cfg=appleos");
}
