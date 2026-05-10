use std::env;
use std::path::PathBuf;

fn main() {
    // Check for OpenImageDenoise_DIR env var first (user-specified), then fall back to common paths
    let oidn_dir = if let Ok(dir) = env::var("OpenImageDenoise_DIR") {
        PathBuf::from(dir)
    } else {
        find_oidn().expect(
            "Could not find OpenImageDenoise. Set OpenImageDenoise_DIR=/path/to/oidn or install it to a standard location."
        )
    };

    let lib_dir = oidn_dir.join("lib");
    let include_dir = oidn_dir.join("include");

    assert!(
        lib_dir.exists(),
        "OIDN lib dir not found: {}",
        lib_dir.display()
    );
    assert!(
        include_dir.exists(),
        "OIDN include dir not found: {}",
        include_dir.display()
    );

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=OpenImageDenoise");

    // Re-run if OpenImageDenoise_DIR changes
    println!("cargo:rerun-if-env-changed=OpenImageDenoise_DIR");
}

fn find_oidn() -> Option<PathBuf> {
    let candidates = if cfg!(target_os = "windows") {
        vec![
            PathBuf::from("C:/Program Files/OpenImageDenoise"),
            PathBuf::from("C:/OpenImageDenoise"),
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            PathBuf::from("/usr/local"),
            PathBuf::from("/opt/homebrew"),
            PathBuf::from("/opt/homebrew/opt/openimagedenoise"),
        ]
    } else {
        // Linux
        vec![
            PathBuf::from("/usr"),
            PathBuf::from("/usr/local"),
            PathBuf::from("/opt/OpenImageDenoise"),
        ]
    };

    candidates
        .into_iter()
        .find(|p| p.join("lib").exists() && p.join("include").exists())
}
