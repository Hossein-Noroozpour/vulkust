extern crate reqwest;

use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;

const DEFAULT_FONTS: [(&str, &str); 1] = [(
    "Ubuntu-B.ttf",
    "https://github.com/Hossein-Noroozpour/vulkust-static-files/blob/master/Ubuntu-B.ttf?raw=true",
)];

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
    #[cfg(debug_assertions)]
    {
        println!("cargo:rustc-cfg=debug_gx3d");
        println!("cargo:rustc-cfg=debug_mode");
    }
    check_shaders();
    check_fonts();
}

fn check_shaders() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let vulkan_spirv_path = Path::new(&out_dir).join("vulkan/shaders/");
    fs::create_dir_all(vulkan_spirv_path.clone()).expect("can not create vulkan/shaders path");;
    let vulkan_shaders_path = Path::new(&src_dir).join("src/vulkan/shaders/");
    let ls = vulkan_shaders_path
        .read_dir()
        .expect("read_dir call failed");
    for entry in ls {
        // todo it must become more conservative, it should compile changed shaders
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
        if !spirv_file.ends_with(".vert") && !spirv_file.ends_with(".frag") {
            continue;
        }
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
}

fn check_fonts() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let fonts_path = Path::new(&out_dir).join("render/fonts/");
    if !fonts_path.exists() {
        fs::create_dir_all(fonts_path.clone()).expect("can not create render path");
        download_fonts();
        return;
    }
    for font in &DEFAULT_FONTS {
        let font_path = Path::new(&fonts_path).join(font.0);
        if !font_path.is_file() {
            download_fonts();
            return;
        }
    }
}

fn download_fonts() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let fonts_path = Path::new(&out_dir).join("render/fonts/");
    for font in &DEFAULT_FONTS {
        let font_path = Path::new(&fonts_path).join(font.0);
        let mut file = File::create(font_path).expect("can not create font file.");
        let mut resp = reqwest::get(font.1).expect("can not connect to fonts server.");
        resp.copy_to(&mut file).expect("can not download fonts");
    }
}
