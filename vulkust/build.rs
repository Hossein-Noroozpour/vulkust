#![feature(generators)]
#![feature(future_join)]

extern crate futures;
extern crate reqwest;
extern crate tokio;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
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
    #[cfg(feature = "blank_gapi")]
    println!("cargo:rustc-cfg=blank_gapi");
    #[cfg(feature = "directx12")]
    {
        if !in_windows {
            panic!("Error directx12 API is only available in Windows platform");
        }
        println!("cargo:rustc-cfg=directx12_api");
    }
    #[cfg(all(not(feature = "directx12"), not(feature = "blank_gapi")))]
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
        println!("cargo:rustc-cfg=debug_gapi");
        println!("cargo:rustc-cfg=debug_texture");
    }
    check_shaders();
    futures::executor::block_on(check_fonts());
}

fn check_shaders() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    #[cfg(feature = "directx12")]
    let compiled_shader_path = Path::new(&out_dir).join("directx12/shaders/");
    #[cfg(not(feature = "directx12"))]
    let compiled_shader_path = Path::new(&out_dir).join("vulkan/shaders/");
    fs::create_dir_all(compiled_shader_path.clone()).expect("can not create compiled shaders path");
    #[cfg(feature = "directx12")]
    let shaders_source_path = Path::new(&src_dir).join("src/d3d12/shaders/");
    #[cfg(not(feature = "directx12"))]
    let shaders_source_path = Path::new(&src_dir).join("src/vulkan/shaders/");
    let ls = shaders_source_path
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
        #[cfg(feature = "directx12")]
        {
            let hlsl_file = shader_path.file_name().unwrap().to_str().unwrap();
            if hlsl_file.ends_with(".h.hlsl") || !hlsl_file.ends_with(".hlsl") {
                continue;
            }
            let hlsl_file = compiled_shader_path
                .join(hlsl_file)
                .to_str()
                .unwrap()
                .to_string();
            let vert_file = hlsl_file.clone() + ".vert.fxc";
            let frag_file = hlsl_file + ".frag.fxc";
            let outputs = vec![vert_file, frag_file];
            let profiles = vec!["vs_5_1", "ps_5_1"];
            let entries = vec!["vert_main", "frag_main"];
            for i in 0..outputs.len() {
                let mut output = Command::new("fxc");
                #[cfg(debug_assertions)]
                let output = output.arg("/Od");
                #[cfg(debug_assertions)]
                let output = output.arg("/Zi");
                #[cfg(not(debug_assertions))]
                let output = output.arg("/O3");
                let output = output
                    .arg("/T")
                    .arg(&profiles[i])
                    .arg("/Fo")
                    .arg(&outputs[i])
                    .arg(shader_file)
                    .arg("/E")
                    .arg(&entries[i])
                    .output()
                    .expect("failed to execute hlsl compiler");
                if !output.status.success() {
                    panic!(
                        "Compiling hlsl shaders failed.\n {:?}\n in {:?}",
                        output, shader_file
                    );
                }
            }
        }
        #[cfg(not(feature = "directx12"))]
        {
            let spirv_file = shader_path.file_name().unwrap().to_str().unwrap();
            if !spirv_file.ends_with(".vert") && !spirv_file.ends_with(".frag") {
                continue;
            }
            let spirv_file = spirv_file.to_string() + ".spv";
            let spirv_file = compiled_shader_path.join(&spirv_file);
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
}

async fn check_fonts() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let fonts_path = Path::new(&out_dir).join("render/fonts/");
    if !fonts_path.exists() {
        fs::create_dir_all(fonts_path.clone()).expect("can not create render path");
        download_fonts().await;
        return;
    }
    for font in &DEFAULT_FONTS {
        let font_path = Path::new(&fonts_path).join(font.0);
        if !font_path.is_file() {
            download_fonts().await;
            return;
        }
    }
}

async fn download_fonts() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let fonts_path = Path::new(&out_dir).join("render/fonts/");
    for font in &DEFAULT_FONTS {
        let font_path = Path::new(&fonts_path).join(font.0);
        let mut file = File::create(font_path).expect("can not create font file.");
        let resp = reqwest::get(font.1)
            .await
            .expect("Can not receive the data.")
            .bytes()
            .await
            .expect("Can not read the data.");
        file.write(&resp).expect("can not download fonts");
    }
}
