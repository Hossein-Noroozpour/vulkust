use std::env;

const ERROR_NO_VULKAN_SDK: &'static str =
    "Error in founding Vulkan SDK path, please make sure you installed vulkan sdk";

const VULKAN: &'static str = "vulkan-1";

fn main() {
    let vulkan_sdk_path = "VK_SDK_PATH";
    let vulkan_sdk_path = match env::var_os(vulkan_sdk_path) {
        Some(val) => val.into_string().expect(ERROR_NO_VULKAN_SDK),
        None => panic!(ERROR_NO_VULKAN_SDK),
    };
    // println!("cargo:rustc-link-lib=dylib={}", VULKAN);
    // println!("cargo:rustc-link-lib-search=native={}\\bin", vulkan_sdk_path);
    // println!("cargo:rustc-cfg={}", VULKAN);
    // println!("cargo:root={}\\bin", vulkan_sdk_path);
    // println!("cargo:libdir={}\\bin", vulkan_sdk_path);
    // println!("cargo:include={}\\include", vulkan_sdk_path);
    println!("cargo:rustc-flags=-l {} -L {}\\bin", VULKAN, vulkan_sdk_path);
}
