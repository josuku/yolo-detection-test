use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not defined"),
    );

    let (library_name, runtime_dir) = runtime_info(&manifest_dir);

    let source = runtime_dir.join(library_name);

    println!("cargo:rerun-if-changed={}", source.display());

    if !source.exists() {
        println!(
            "cargo:warning=Runtime library not found: {}",
            source.display()
        );
        return;
    }

    let target_dir = target_directory(&manifest_dir);

    fs::create_dir_all(&target_dir)
        .expect("Failed to create target directory");

    let destination = target_dir.join(library_name);

    fs::copy(&source, &destination)
        .expect("Failed to copy runtime library");

    println!(
        "cargo:warning=Copied {} -> {}",
        source.display(),
        destination.display()
    );
}

fn runtime_info(manifest_dir: &Path) -> (&'static str, PathBuf) {
    if cfg!(target_os = "windows") {
        (
            "onnxruntime.dll",
            manifest_dir.join("runtime").join("windows"),
        )
    } else if cfg!(target_os = "linux") {
        (
            "libonnxruntime.so",
            manifest_dir.join("runtime").join("linux"),
        )
    } else if cfg!(target_os = "macos") {
        (
            "libonnxruntime.dylib",
            manifest_dir.join("runtime").join("macos"),
        )
    } else {
        panic!("Unsupported operating system");
    }
}

fn target_directory(manifest_dir: &Path) -> PathBuf {
    let profile = env::var("PROFILE").unwrap();

    let mut target = if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(dir)
    } else {
        manifest_dir.join("target")
    };

    if let Ok(triple) = env::var("TARGET") {
        if !target.ends_with(&triple) {
            let candidate = target.join(&triple);

            if candidate.exists() {
                target = candidate;
            }
        }
    }

    target.join(profile)
}
