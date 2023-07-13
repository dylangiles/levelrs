use std::{error::Error, path::Path, process::Command};

const LEVELDB_LIB_PATH: &str = "./leveldb.lib";
const LEVELDB_UPSTREAM: &str = "https://github.com/google/leveldb";
const LEVELDB_PATH: &str = "deps/leveldb";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-link-lib=leveldb");

    if std::path::Path::new(LEVELDB_LIB_PATH).exists() {
        return Ok(());
    }

    let path_leveldb = std::path::Path::new(LEVELDB_PATH);
    if path_leveldb.exists() {
        std::fs::remove_dir_all(path_leveldb)?
    }

    Command::new("git")
        .args(&[
            "clone",
            LEVELDB_UPSTREAM,
            LEVELDB_PATH,
            "--recurse-submodules",
        ])
        .status()
        .unwrap();

    #[cfg(target_os = "windows")]
    build_leveldb_windows(path_leveldb)?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn build_leveldb_windows(path_leveldb: &Path) -> Result<(), Box<dyn Error>> {
    let build_path = path_leveldb.join("build");
    std::fs::create_dir(&build_path).unwrap();

    Command::new("cmake")
        .args(&["-G", "Visual Studio 17 2022", ".."])
        .current_dir(&build_path)
        .status()
        .unwrap();

    // msbuild leveldb.vcxproj -p:Configuration=Release -p:WarningLevel=1
    Command::new("msbuild")
        .args(&[
            "leveldb.vcxproj",
            "-p:Configuration=Release",
            "-p:WarningLevel=0",
        ])
        .current_dir(&build_path)
        .status()
        .unwrap();

    std::fs::copy(build_path.join("Release/leveldb.lib"), "./leveldb.lib")?;

    Ok(())
}
