use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{utils, link, coinbuilder};

const LIB_NAME: &str = "CoinUtils";

fn main() {
    println!("cargo:rerun-if-changed={}_lib_sources.txt", LIB_NAME.to_ascii_lowercase());
    println!("cargo:rerun-if-changed=CARGO_{}_STATIC", LIB_NAME.to_ascii_uppercase());
    println!("cargo:rerun-if-changed=CARGO_{}_SYSTEM", LIB_NAME.to_ascii_uppercase());

    link::link_lib_system_if_defined(LIB_NAME);

    if !Path::new(&format!("{}/AUTHORS", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let mut config = coinbuilder::init_builder();

    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(LIB_NAME)
        .join(LIB_NAME)
        .join("src")
        .display());

    let includes_dir = vec![
        src_dir.clone()
    ];

    let lib_sources = include_str!("coinutils_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let coinflags = vec!["COINUTILS".to_string()];

    coinbuilder::print_metedata(includes_dir.clone(), coinflags.clone());

    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile(LIB_NAME);

}
