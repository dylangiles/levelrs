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
    // ```cmd
    // mkdir build
    // cd build
    // cmake -G "Visual Studio 15" ..
    // ```
    // The default default will build for x86. For 64-bit run:

    // ```cmd
    // cmake -G "Visual Studio 15 Win64" ..
    // ```
    // Command::new("mkdir").arg("build").current_dir(LEVELDB_PATH).status().unwrap();
    // Command::new("cd").arg("build").current_dir(LEVELDB_PATH).status().unwrap();
    //C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build
    let vcvars = match std::env::var("LEVELRS_VCVARS_LOCATION") {
        Ok(l) => l,
        Err(e) => panic!(
            "Failed to read environment variable LEVELRS_VCVARS_LOCATION: {}",
            e
        ),
    };

    let vcvars_location = std::path::Path::new(vcvars.as_str());
    Command::new("call")
        .arg(vcvars_location.join("vcvarsall.bat"))
        .status()
        .unwrap();

    // #[cfg(target_arch = "x86_64")]
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
