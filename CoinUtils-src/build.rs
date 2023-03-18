use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIB_NAME: &str = "CoinUtils";

fn main() {
    if cfg!(feature = "system") {
        link_lib_system(LIB_NAME);
    } else {
        if !Path::new("CoinUtils/AUTHORS").exists() {
            update_submodules();
        }
        build_lib_and_link();
    }
}

fn update_submodules() {
    let program = "git";
    let dir = "../";
    let args = ["submodule", "update", "--init"];
    println!(
        "Running command: \"{} {}\" in dir: {}",
        program,
        args.join(" "),
        dir
    );
    let ret = Command::new(program).current_dir(dir).args(args).status();

    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

fn build_lib_and_link() {
    let target = env::var("TARGET").unwrap();

    let mut config = cc::Build::new()
        .cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .define("NDEBUG", None)
        .define("HAVE_STDIO_H", None)
        .define("HAVE_STDLIB_H", None)
        .define("HAVE_STRING_H", None)
        .define("HAVE_INTTYPES_H", None)
        .define("HAVE_STDINT_H", None)
        .define("HAVE_STRINGS_H", None)
        .define("HAVE_SYS_TYPES_H", None)
        .define("HAVE_SYS_STAT_H", None)
        .define("HAVE_UNISTD_H", None)
        .define("HAVE_CMATH", None)
        .define("HAVE_CFLOAT", None)
        .define("HAVE_DLFCN_H", None)
        .define("HAVE_MEMORY_H", None)
        .to_owned();

    if target.contains("msvc") {
        config.flag("-EHsc").flag("-std:c++14");
    } else {
        config.flag("-std=c++11").flag("-w");
    }

    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(LIB_NAME)
        .join(LIB_NAME)
        .join("src");

    let includes_dir = vec![src_dir.clone()];

    config.includes(includes_dir);

    let lib_sources = include_str!("CoinUtils_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir.display(), file.trim()))
        .collect::<Vec<String>>();

    config.files(lib_sources);

    config.compile("CoinUtils");
}

fn link_lib_system(lib_name: &str) {
    let link_kind = if cfg!(features = "static") {
        "static"
    } else {
        "dylib"
    };

    if cfg!(target_os = "windows") {
        if cfg!(target_env = "gnu") {
            link_windows_gnu_system(lib_name);
        } else if cfg!(target_env = "msvc") {
            link_windows_msvc_system(lib_name);
        } else {
            panic!(
                "Can not link libs for Windows: {}",
                env::var("CARGO_CFG_TARGET_ENV").unwrap()
            );
        }
    } else if cfg!(target_os = "macos") {
        link_macos_system(lib_name);
    }

    println!("cargo:rustc-link-lib={}={}", link_kind, lib_name);
}

fn link_windows_msvc_system(lib_name: &str) {
    if cfg!(feature = "static") {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }

    vcpkg::find_package(lib_name).unwrap();
}

fn link_windows_gnu_system(_lib_name: &str) {
    let search_path = if cfg!(feature = "static") {
        "/mingw64/bin"
    } else {
        "/mingw64/lib"
    };

    let lib_path = String::from_utf8(
        Command::new("cygpath")
            .arg("-w")
            .arg(search_path)
            .output()
            .expect("Failed to exec cygpath")
            .stdout,
    )
    .expect("cygpath output includes non UTF-8 string");
    println!("cargo:rustc-link-search={}", lib_path);
}

fn link_macos_system(lib_name: &str) {
    fn brew_prefix(target: &str) -> PathBuf {
        let out = Command::new("brew")
            .arg("--prefix")
            .arg(target)
            .output()
            .expect("brew not installed");
        assert!(out.status.success(), "`brew --prefix` failed");
        let path = String::from_utf8(out.stdout).expect("Non-UTF8 path by `brew --prefix`");
        PathBuf::from(path.trim())
    }

    let lib_path = brew_prefix(lib_name);

    println!("cargo:rustc-link-search={}/lib", lib_path.display());
}
