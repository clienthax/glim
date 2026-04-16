use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let shader_dir = Path::new("shaders");

    let slangc = env::var("SLANG_DIR")
        .map(|p| PathBuf::from(p).join("bin/slangc"))
        .or_else(|_| env::var("VULKAN_SDK").map(|p| PathBuf::from(p).join("bin/slangc")))
        .unwrap_or_else(|_| PathBuf::from("slangc"));

    println!("cargo:rerun-if-changed=shaders");

    let shaders = ["test.slang"];

    for shader in shaders {
        let shader_path = shader_dir.join(shader);
        let file_stem = shader_path.file_stem().unwrap().to_str().unwrap();
        let spv_path = out_dir.join(format!("{}.spv", file_stem));

        let status = Command::new(&slangc)
            .arg(shader_path.to_str().unwrap())
            .arg("-o")
            .arg(spv_path.to_str().unwrap())
            .args(["-target", "spirv"])
            .args(["-stage", "compute"])
            .args(["-entry", "main"])
            .status()
            .expect("Failed to run slangc");

        if !status.success() {
            panic!("Slang compilation failed for {:?}", shader_path);
        }

        println!("spv_path: {:?}", spv_path);
    }
}
