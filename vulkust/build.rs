use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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
    println!("cargo:rustc-cfg=vulkan_api");
    if in_macos || in_ios {
        println!("cargo:rustc-cfg=apple_os");
    }
    if in_linux || in_windows || in_macos {
        println!("cargo:rustc-cfg=desktop_os");
    }
    if in_linux || in_ios || in_macos || in_android {
        println!("cargo:rustc-cfg=unix_based_os");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let vulkan_spirv_path = Path::new(&out_dir).join("vulkan/");
    let _ = fs::create_dir(vulkan_spirv_path.clone());
    let vulkan_spirv_path = vulkan_spirv_path.join("shaders/");
    let _ = fs::create_dir(vulkan_spirv_path.clone());
    let vulkan_shaders_path = Path::new(&src_dir).join("src/vulkan/shaders/");
    let ls = vulkan_shaders_path
        .read_dir()
        .expect("read_dir call failed");
    for entry in ls {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();
        let file_type = entry.file_type();
        if file_type.is_err() {
            continue;
        }
        let file_type = file_type.unwrap();
        if !file_type.is_file() && !file_type.is_symlink() {
            continue;
        }
        let shader_path = entry.path();
        let shader_file = shader_path.to_str().unwrap();
        let spirv_file = shader_path.file_name().unwrap().to_str().unwrap();
        let spirv_file = spirv_file.to_string() + ".spv";
        let spirv_file = vulkan_spirv_path.join(&spirv_file);
        let spirv_file = spirv_file.to_str().unwrap();
        #[cfg(debug_assertions)]
        let compile_flag = "-g";
        #[cfg(not(debug_assertions))]
        let compile_flag = "-Os";
        let output = Command::new("glslangValidator")
            .arg("-V")
            .arg(compile_flag)
            .arg(shader_file)
            .arg("-o")
            .arg(spirv_file)
            .output()
            .expect("failed to execute glsl compiler");
        if !output.status.success() {
            panic!("Compiling vulkan shaders failed.\n {:?}", output);
        }
    }
    // glslangValidator -V -g ../../vulkust/src/vulkan/shaders/main.vert -o shaders/main.vert.spv
    // let mut f = File::create(&dest_path).unwrap();

    // f.write_all(b"
    //     pub fn message() -> &'static str {
    //         \"Hello, World!\"
    //     }
    // ").unwrap();
}
